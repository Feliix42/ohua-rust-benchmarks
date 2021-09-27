use crate::types::*;
use std::sync::Arc;

fn run(
    values: Vec<Vec<Value>>,
    centroids: Arc<Vec<Centroid>>,
    threshold: f32,
    iterations: u32,
) -> u32 {
    let mut new_values = Vec::default();

    for v in values {
        let i = reassign_values(v, centroids.clone()); // -> (Value, f32 or u32)
        new_values.push(i);
    }

    // now calculate the new centroids and the delta
    let (vals, delta) = evaluate_results(new_values);

    let cont = should_continue(delta, threshold.clone(), iterations.clone());
    let (new_vals, new_centroids) = create_centroids(vals, centroids);
    let inc_iter = inc(iterations);

    if cont {
        run(new_vals, new_centroids, threshold, inc_iter)
    } else {
        inc_iter
    }
}

pub fn calculate(
    values: Vec<Vec<Value>>,
    centroids: Arc<Vec<Centroid>>,
    threshold: f32,
    iterations: u32,
) -> u32 {
    run(values, centroids, threshold, iterations)
}
