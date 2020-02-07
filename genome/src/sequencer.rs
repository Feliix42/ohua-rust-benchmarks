use crate::segments::Segments;
use crate::Nucleotide;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Eq, Hash, PartialEq)]
struct SequencerItem {
    pub segment: Rc<Vec<Nucleotide>>,
    pub prev: Option<Rc<Vec<Nucleotide>>>,
    pub next: Option<Rc<Vec<Nucleotide>>>,
    pub overlap_with_prev: usize,
}

impl From<Vec<Nucleotide>> for SequencerItem {
    fn from(nucleotide_sequence: Vec<Nucleotide>) -> Self {
        SequencerItem {
            segment: Rc::new(nucleotide_sequence),
            prev: None,
            next: None,
            overlap_with_prev: 0,
        }
    }
}

pub fn run_sequencer(mut segments: Segments) -> Vec<Nucleotide> {
    // Step 1: deduplicate all segments
    let mut tmp: HashSet<SequencerItem> = segments
        .contents
        .drain(..)
        .map(SequencerItem::from)
        .collect();
    
    // TODO: Isn't that super ineffective? :thinking:
    let mut unique_segments: Vec<SequencerItem> = tmp.drain().collect();
    let segment_count = unique_segments.len();

    // Step 2: go through the prefixes and suffixes of the genomes in descending size and stitch the genome back together
    for match_length in segments.length..0 {
        /*
         * What I want to do:
         * - loop through possible subsegment lengths
         * - loop through all segments once (x)
         * - loop through all segments again (y) and match against the starts and ends of x and y, stitch together on match
         *
         * Question: *How* do I remove matches from the set on the go?
         * Plot Twist: Should be parallelizable for STM and Ohua.
         */

        for _ in 0..segment_count {
            let mut cur_seg = unique_segments.pop().expect("No segments were generated");

            // only continue if the current segment is not 
            if cur_seg.prev.is_none() {
                let slice = &cur_seg.segment[(segments.length - match_length)..segments.length];

                // go over all items in Vec and test whether we can append our `cur_seg` to the item. If so, stop
                for item in unique_segments.iter_mut() {
                    // skip the current item when it already has an appended segment
                    if item.next.is_none() {
                        continue;
                    }

                    // TODO: get slices from both the cur_seg and this segment and compare them for equality
                    let cur_slice = &item.segment[(segments.length - match_length)..segments.length];
                    if slice == cur_slice {
                        // link both items together
                        item.next = Some(cur_seg.segment.clone());
                        cur_seg.prev = Some(item.segment.clone());
                        cur_seg.overlap_with_prev = match_length;
                        break;
                    } 
                }
            }

            unique_segments.push(cur_seg);
        }
    }

    // TMP test
    println!("[TEST] checking segment links");
    let mut forward_links = 0;
    let mut backward_links = 0;
    for item in unique_segments {
        if item.next.is_none() {
            forward_links += 1;
        }
        if item.prev.is_none() {
            backward_links += 1;
        }
    }
    assert_eq!(forward_links, 1);
    assert_eq!(backward_links, 1);

    // Step 3 link together sequence
    // find first element
    let mut cur = unique_segments.iter().find(|seg| seg.prev.is_none()).unwrap();
    let mut reconstructed_sequence = Vec::new();

    loop {
        reconstructed_sequence.extend_from_slice(&cur.segment[cur.overlap_with_prev..]);

        if cur.next.is_some() {
            cur = cur.next.unwrap();
            // TODO: Switch the whole datastructure to have next and prev of type `Rc<RefCell<SequencerItem>>` and segment of `Vec<Nucleotide>` again
        } else {
            break;
        }
    }

    reconstructed_sequence
}
