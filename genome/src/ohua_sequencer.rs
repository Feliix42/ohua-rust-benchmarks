use crate::segments::Segments;
use crate::Nucleotide;
use std::collections::HashSet;
use std::sync::Arc;

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

pub fn deduplicate(mut segments: Segments) -> Vec<SequencerItem> {
    let mut tmp: HashSet<Vec<Nucleotide>> = segments.contents.drain(..).collect();

    tmp.drain().map(SequencerItem::from).collect()
}

/// Searches a segment match for a single sequencer item with a given overlap.
pub fn search_match(segments: Arc<Vec<SequencerItem>>, overlap: usize, elem: usize) -> Option<(usize, usize)> {
    let current = &segments[elem];
    let segments_length = current.segment.len();

    if current.prev.is_none() {
        let slice = &current.segment[0..overlap];

        // go over all items in Vec and test whether we can append our `current` to the item. If so, stop
        for idx in 0..segments.len() {
            let item = &segments[idx];

            // skip the current item when it already has an appended segment
            if item.next.is_some() {
                continue;
            }

            let cur_slice =
                &item.segment[(segments_length - overlap)..segments_length];
            if slice == cur_slice {
                return Some((idx, elem))
            }
        }
    }

    None
}

pub fn reassemble(unique_segments: Arc<Vec<SequencerItem>>) -> Vec<Nucleotide> {
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

pub fn update_sequence(mut seq_arc: Arc<Vec<SequencerItem>>, updates: Arc<Vec<Option<(usize, usize)>>>, overlap: usize) -> Arc<Vec<SequencerItem>> {
    //use std::borrow::Borrow;
    //let bla: &Vec<SequencerItem> = seq_arc.borrow();
    //let mut seq: Vec<SequencerItem> = bla.clone();
    
    unsafe {
        let mut seq: &mut Vec<SequencerItem> = Arc::get_mut_unchecked(&mut seq_arc);

        for u in updates.iter() {
            if let Some((first, last)) = u {
                if seq[*first].next.is_some() || seq[*last].prev.is_some() {
                    eprintln!("Encountered invalid match!");
                    continue;
                }

                seq[*first].next = Some(*last);
                seq[*last].prev = Some(*first);
                seq[*last].overlap_with_prev = overlap;
            }
        }
    }

    // Arc::new(seq)
    seq_arc
}
