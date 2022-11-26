#![allow(non_snake_case, unused_mut)]
use crate::ohua_sequencer::*;
use crate::segments::Segments;
use crate::Nucleotide;
use std::sync::Arc;

pub fn sequencer(segments: Segments, starting_overlap: usize) -> Vec<Nucleotide> {
    // phase 1: deduplication. Can't really do that in parallel.
    let unique_segments: SequencerData = deduplicate(segments);

    let matched: SequencerData = match_segments(unique_segments, starting_overlap);

    reassemble(matched)
}

pub fn match_segments(items: SequencerData, overlap: usize) -> SequencerData {
    let (o, next): (usize, usize) = get_overlap(overlap);
    let next2: usize = next.clone();

    let indices: Vec<usize> = items.get_indices();
    let items_arc: Arc<SequencerData> = Arc::new(items);

    let mut rs: Vec<Option<(usize, usize)>> = Vec::new();
    for i in indices {
        let idx: usize = i;
        let items_cloned: Arc<SequencerData> = items_arc.clone();
        let o2: usize = o.clone();

        let res: Option<(usize, usize)> = search_match(items_cloned, o2, idx);

        rs.push(res);
    }

    let (mut items2, res): (SequencerData, Vec<Option<(usize, usize)>>) =
        seq_arc_unwrap(items_arc, rs);

    items2.update(res, o);

    let cont: bool = remaining_computations(next2);

    if cont {
        match_segments(items2, next)
    } else {
        items2
    }
}
