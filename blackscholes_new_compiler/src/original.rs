use crate::types::*;

pub fn calculate(ops: Vec<Vec<OptionData>>) -> Vec<f32> {
    // TODO: this loop is not correct -> the items must be collected in a vec separately
    let mut results: Vec<Vec<f32>> = Vec::new();

    for op0 in ops {
        let op: Vec<OptionData> = op0;
        let i: Vec<f32> = batch_calculate_black_scholes(op);
        results.push(i);
    }

    unpack(results)
}
