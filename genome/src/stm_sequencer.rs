use crate::segments::Segments;
use crate::Nucleotide;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Range;
use std::thread::{self, JoinHandle};
use stm::{atomically, TVar};

#[derive(Clone, Debug)]
pub struct SequencerItem {
    pub segment: Vec<Nucleotide>,
    pub prev: TVar<Option<SequencerItem>>,
    pub next: TVar<Option<SequencerItem>>,
    pub overlap_with_prev: TVar<usize>,
}

impl From<Vec<Nucleotide>> for SequencerItem {
    fn from(nucleotide_sequence: Vec<Nucleotide>) -> Self {
        SequencerItem {
            segment: nucleotide_sequence,
            prev: TVar::new(None),
            next: TVar::new(None),
            overlap_with_prev: TVar::new(0),
        }
    }
}

pub fn deduplicate(mut segments: Segments) -> VecDeque<SequencerItem> {
    // Step 1: deduplicate all segments
    let mut tmp: HashSet<Vec<Nucleotide>> = segments.contents.drain(..).collect();

    tmp.drain().map(SequencerItem::from).collect()
}

pub fn run_sequencer(
    unique_segments: &VecDeque<SequencerItem>,
    segment_length: usize,
    iteration_ranges: Vec<Range<usize>>,
) {
    // Step 2: go through the prefixes and suffixes of the genomes in descending size and stitch the genome back together
    for match_length in (1..segment_length).rev() {
        /*
         * - loop through possible subsegment lengths
         * - loop through all segments once (x)
         * - loop through all segments again (y) and match against the starts and ends of x and y, stitch together on match
         */

        // spawn threads to work in parallel
        // has to happen here since we want to parallelize the work but also synchronize before
        // reducing the match_length by one
        let mut handles = Vec::new();
        for rng in iteration_ranges.clone() {
            let iteration_range = rng.clone();
            let segments = unique_segments.clone();
            handles.push(thread::spawn(move || {
                for idx in iteration_range {
                    atomically(|trans| {
                        let cur_seg = &segments[idx];

                        // only continue if the current segment is not linked already
                        if cur_seg.prev.read(trans)?.is_none() {
                            let slice = &cur_seg.segment[0..match_length];

                            // go over all items in Vec and test whether we can append our `cur_seg` to the item. If so, stop
                            'inner: for it in 0..segments.len() {
                                // skip the element itself -- this might be unnecessary but we want to avoid
                                // breaking the matching algorithm by linking an element to itself
                                if idx == it {
                                    continue;
                                }
                                // skip the current item when it already has an appended segment
                                if segments[it].next.read(trans)?.is_some() {
                                    continue;
                                }

                                let cur_slice = &segments[it].segment
                                    [(segment_length - match_length)..segment_length];
                                if slice == cur_slice {
                                    // link both items together
                                    segments[it].next.write(trans, Some(cur_seg.clone()))?;
                                    cur_seg.prev.write(trans, Some(segments[it].clone()))?;
                                    cur_seg.overlap_with_prev.write(trans, match_length)?;
                                    break 'inner;
                                }
                            }
                        }
                        Ok(())
                    });
                }
            }));

            // wait for threads to finish
            let _: Vec<()> = handles
                .drain(..)
                .map(JoinHandle::join)
                .map(Result::unwrap)
                .collect();
        }
    }
}

pub fn reconstruct(unique_segments: &VecDeque<SequencerItem>) -> Vec<Nucleotide> {
    // TMP test
    println!("[TEST] checking segment links");
    atomically(|trans| {
        let mut forward_links = 0;
        let mut backward_links = 0;
        for item in unique_segments {
            if item.next.read(trans)?.is_none() {
                forward_links += 1;
            }
            if item.prev.read(trans)?.is_none() {
                backward_links += 1;
            }
        }
        assert_eq!(forward_links, 1);
        assert_eq!(backward_links, 1);
        Ok(())
    });

    // Step 3 link together sequence
    atomically(|trans| {
        // find first element
        let mut cur = unique_segments
            .iter()
            .find(|seg| seg.prev.read_atomic().is_none())
            .unwrap()
            .clone();

        let mut reconstructed_sequence = Vec::new();

        loop {
            reconstructed_sequence
                .extend_from_slice(&cur.segment[cur.overlap_with_prev.read(trans)?..]);

            if cur.next.read(trans)?.is_some() {
                // move to the next value -> have to assign to another let binding first to drop `val` due to ownership issues
                let next = cur.next.read(trans)?.unwrap().clone();
                cur = next;
            } else {
                break;
            }
        }

        Ok(reconstructed_sequence)
    })
}
