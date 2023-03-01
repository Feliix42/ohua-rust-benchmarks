use crate::types::*;
use std::ops::Range;
use std::sync::Arc;

pub fn calculate(
    options: Arc<Vec<OptionData>>,
    items: Arc<Vec<f32>>,
    ranges: Vec<Range<usize>>,
) -> Vec<f32> {
    let ops: Arc<Vec<OptionData>> = id(options);
    let its: Arc<Vec<f32>> = id(items);
    let mut results: Vec<Arc<Vec<f32>>> = Vec::new();

    for rng in ranges {
        let rng0: Range<usize> = rng;
        let op: Arc<Vec<OptionData>> = ops.clone();
        let it: Arc<Vec<f32>> = its.clone();
        let i: Arc<Vec<f32>> = batch_calculate_black_scholes_unsafe(op, it, rng0);
        results.push(i);
    }

    seq_arc_unpack(its, results)
}
