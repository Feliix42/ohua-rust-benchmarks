#![allow(non_snake_case, unused_mut)]
use crate::types::*;
use std::sync::Arc;

fn run(mut state: Netlist, worklist: Vec<(usize, usize)>, temperature: f64) -> Netlist {
    let mut rs = Vec::new(); // the new worklist
    let new_temp: f64 = reduce_temp(temperature);
    let n2: Netlist = state.clone();
    let nro: Arc<Netlist> = Arc::new(n2);
    for item in worklist {
        let switch_info: (MoveDecision, (usize, usize)) =
            process_move(item, nro.clone(), new_temp.clone());
        // updates the netlist by performing the switch, returning an error when there's a collision
        rs.push(switch_info);
    }

    let remaining_work: Vec<(usize, usize)> = state.update(rs);
    let keep_going: bool = state.get_keep_going();

    if keep_going {
        run(state, remaining_work, new_temp)
    } else {
        state
    }
}

pub fn annealer(netlist: Netlist, worklist: Vec<(usize, usize)>, temperature: f64) -> Netlist {
    run(netlist, worklist, temperature)
}
