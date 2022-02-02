use crate::types::*;

pub fn calculate(ops: Vec<Vec<OptionData>>) -> Vec<f32> {
    // TODO: this loop is not correct -> the items must be collected in a vec separately
    let mut results = Vec::new();

    for op in ops {
        let i = batch_calculate_black_scholes(op);
        results.push(i);
    }

    unpack(results)
}
