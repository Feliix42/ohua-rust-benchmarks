use crate::types::*;
use std::sync::Arc;

fn run(
    values: Vec<Vec<Value>>,
    centroids: Arc<Vec<Centroid>>,
    threshold: f32,
    iterations: u32,
) -> u32 {
    let mut new_values: Vec<(Vec<Value>, u32)>= Vec::default();

    for v0 in values {
        let v: Vec<Value> = v0;
        let c_clone:Arc<Vec<Centroid>> = centroids.clone();
        let i: (Vec<Value>, u32) = reassign_values(v, c_clone); // -> (Value, f32 or u32)
        new_values.push(i);
    }

    // FIXME Could be done inside the loop to benefit from pipeline or even data parallelism
    // now calculate the new centroids and the delta
    let (vals, delta): (Vec<Vec<Value>>, f32) = evaluate_results(new_values);

    let t_clone: f32 = threshold.clone();
    let i_clone: u32 = iterations.clone();
    let cont: bool = should_continue(delta, t_clone, i_clone);
    let (new_vals, new_centroids): (Vec<Vec<Value>>, Arc<Vec<Centroid>>) = create_centroids(vals, centroids);
    let inc_iter: u32 = inc(iterations);

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
