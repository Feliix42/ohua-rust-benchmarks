use crate::types::*;
use std::ops::Range;
use std::sync::Arc;

pub fn calculate(options: Arc<Vec<OptionData>>, ranges: Vec<Range<usize>>) -> Vec<f32> {
    let ops: Arc<Vec<OptionData>> = id(options);
    let mut results: Vec<Vec<f32>> = Vec::new();

    for rng in ranges {
        let rng0: Range<usize> = rng;
        let op: Arc<Vec<OptionData>> = ops.clone();
        let i: Vec<f32> = batch_calculate_black_scholes_arc(op, rng0);
        results.push(i);
    }

    unpack(results)
}
