#![allow(non_snake_case, unused_mut)]
use crate::types::*;
use std::sync::Arc;

fn run(
    mut netlist: Netlist,
    worklist: Vec<Result<MoveDecision, (usize, usize)>>,
    temperature: f64,
    mut internal_state: InternalState,
) -> Netlist {
    let mut rs = Vec::new(); // the new worklist

    let new_temp = reduce_temp(temperature);

    for item in worklist {
        let n2 = netlist.clone();
        let nro = Arc::new(n2);
        let switch_info = process_move(item, nro.clone(), new_temp.clone());
        // updates the netlist by performing the switch, returning an error when there's a collision
        let result = netlist.update(switch_info);
        rs.push(result);
        netlist.clear_changes();
    }

    let rs2 = rs.clone();
    let mut remaining_work = filter_work(rs);
    let length = remaining_work.len();

    // FIXME: I need this to show the compiler that new_temp is *not* the amorphous variable
    let (_new_temp2, new_temp3) = dup(new_temp);

    // get new work if necessary
    // get whether to continue
    let (new_work, keep_going) = internal_state.assess_updates(rs2, length);
    // add new work items
    remaining_work.exp(new_work);

    if keep_going {
        run(netlist, remaining_work, new_temp3, internal_state)
    } else {
        netlist
    }
}

pub fn annealer(
    netlist: Netlist,
    elements: usize,
    temperature: f64,
    max_steps: Option<i32>,
    swaps_per_temp: usize,
) -> Netlist {
    let mut st = InternalState::initialize(elements, max_steps, swaps_per_temp);

    let worklist = st.generate_worklist();
    run(netlist, worklist, temperature, st)
}
