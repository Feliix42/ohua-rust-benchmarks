use crate::types::*;
use rand_chacha::ChaCha12Rng;
use std::sync::Arc;

pub const THREADCOUNT: usize = 4;

pub fn run(netlist: Netlist, worklist: Vec<(usize, usize)>, mut rng: ChaCha12Rng, temperature: f64, completed_steps: i32, max_steps: Option<i32>, swaps_per_temp: usize) -> i32 { 
    let mut rs = Vec::default();

    let new_temp = reduce_temp(temperature);

    let nro = Arc::new(netlist.clone());
    for item in worklist {
        let switch_info = process_move(item, nro.clone()); // gives a Good/Bad/Reject plus tuple
        let res = netlist.update(switch_info); // produces a Option<(_, _)> with the update info (success/fail)
        // TODO: How to track overrides??
        rs.push(res);
    }

    let (keep_going, rest, new_rng) = assess_updates(rs, new_temp.clone(), completed_steps.clone(), max_steps.clone(), swaps_per_temp.clone(), rng);

    let new_temp_steps = increment(completed_steps);
    if keep_going {
        run(netlist, rest, new_rng, new_temp, new_temp_steps, max_steps, swaps_per_temp)
    } else {
        new_temp_steps
    }
}
