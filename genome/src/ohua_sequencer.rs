use crate::segments::Segments;
use crate::Nucleotide;
use itertools::Itertools;
use std::sync::Arc;

pub struct SequencerData {
    pub data: Vec<SequencerItem>,
}

impl SequencerData {
    pub fn get_indices(&self) -> Vec<usize> {
        (0..self.data.len()).collect()
    }

    pub fn update(&mut self, updates: Vec<Option<(usize, usize)>>, overlap: usize) {
        for u in updates.iter() {
            if let Some((first, last)) = u {
                if self.data[*first].next.is_some() || self.data[*last].prev.is_some() {
                    eprintln!("Encountered invalid match!");
                    continue;
                }

                self.data[*first].next = Some(*last);
                self.data[*last].prev = Some(*first);
                self.data[*last].overlap_with_prev = overlap;
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequencerItem {
    pub segment: Vec<Nucleotide>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub overlap_with_prev: usize,
}

impl From<Vec<Nucleotide>> for SequencerItem {
    fn from(nucleotide_sequence: Vec<Nucleotide>) -> Self {
        SequencerItem {
            segment: nucleotide_sequence,
            prev: None,
            next: None,
            overlap_with_prev: 0,
        }
    }
}

pub fn deduplicate(segments: Segments) -> SequencerData {
    let data = segments
        .contents
        .into_iter()
        .unique() // itertools magic for deduplication: fast because of the enums representation in memory as u8
        .map(SequencerItem::from)
        .collect();

    SequencerData { data }
}

/// Searches a segment match for a single sequencer item with a given overlap.
pub fn search_match(
    segments: Arc<SequencerData>,
    overlap: usize,
    elem: usize,
) -> Option<(usize, usize)> {
    let current = &segments.data[elem];
    let segments_length = current.segment.len();

    if current.prev.is_none() {
        let slice = &current.segment[0..overlap];

        // go over all items in Vec and test whether we can append our `current` to the item. If so, stop
        for idx in 0..segments.data.len() {
            let item = &segments.data[idx];

            // skip the current item when it already has an appended segment
            if item.next.is_some() {
                continue;
            }

            let cur_slice = &item.segment[(segments_length - overlap)..segments_length];
            if slice == cur_slice {
                return Some((idx, elem));
            }
        }
    }

    None
}

pub fn reassemble(
    SequencerData {
        data: unique_segments,
    }: SequencerData,
) -> Vec<Nucleotide> {
    if cfg!(feature = "verify") {
        println!("[TEST] checking segment links");
        let mut forward_links = 0;
        let mut backward_links = 0;
        for item in unique_segments.iter() {
            if item.next.is_none() {
                forward_links += 1;
            }
            if item.prev.is_none() {
                backward_links += 1;
            }
        }
        assert_eq!(forward_links, 1);
        assert_eq!(backward_links, 1);
    }

    // Step 3 link together sequence
    // find first element
    let first = unique_segments
        .iter()
        .find(|seg| seg.prev.is_none())
        .unwrap();
    let mut nxt = first.next.unwrap();
    let mut reconstructed_sequence = first.segment.clone();

    loop {
        let cur = &unique_segments[nxt];
        reconstructed_sequence.extend_from_slice(&cur.segment[cur.overlap_with_prev..]);

        if let Some(n) = cur.next {
            // move to the next value -> have to assign to another let binding first to drop `val` due to ownership issues
            nxt = n;
        } else {
            break;
        }
    }

    reconstructed_sequence
}

pub fn get_overlap(cur: usize) -> (usize, usize) {
    (cur, cur - 1)
}

pub fn remaining_computations(overlap: usize) -> bool {
    overlap > 0
}

pub fn seq_arc_unwrap<S, T>(a: Arc<S>, x: T) -> (S, T) {
    match Arc::<S>::try_unwrap(a) {
        Ok(ap) => (ap,x),
        _ => panic!("Failed to unwrap the Arc. Please make sure that the construction of `x` has destructed all previous Arcs.")
    }
}
