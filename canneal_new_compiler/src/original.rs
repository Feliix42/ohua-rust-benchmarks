#![allow(non_snake_case, unused_mut)]
use crate::types::*;
use std::sync::Arc;

fn run(mut state: Netlist, worklist: Vec<Vec<(usize, usize)>>, temperature: f64) -> Netlist {
    let mut rs: Vec<Vec<(MoveDecision, (usize, usize))>> = Vec::new(); // the new worklist
    let new_temp: f64 = reduce_temp(temperature);
    // cloning before the loop and creating the Arc is certainly more efficient than doing so
    // inside the loop.
    // challenge: I wish we would not have to do so.
    // But in order to support that, we would need to be incorporate immutable borrows into
    // our subset of the language. Future work!
    let nro : Arc<Netlist> = Arc::new(state);
    for item0 in worklist {
        let item : Vec<(usize,usize)> = item0;
        //let n2: Netlist = state.clone();
        // FIXME I do not understand this Arc.
        // There is already a clone of the state.
        // Arcs are needed only when data enters into the loop from the context!
        //let nro: Arc<Netlist> = Arc::new(state); // <--- this is also a data parallel call that is
                                              // actually not worth parallelizing
        let nro_clone: Arc<Netlist> = nro.clone();
        let new_temp_clone: f64 = new_temp.clone();
        let switch_info: Vec<(MoveDecision, (usize, usize))> =
            process_move(item, nro_clone, new_temp_clone);
        // updates the netlist by performing the switch, returning an error when there's a collision
        rs.push(switch_info);
    }

    let (mut nl, res): (Netlist, Vec<Vec<(MoveDecision, (usize, usize))>>) = seq_arc_unwrap(nro, rs);

    let remaining_work: Vec<Vec<(usize, usize)>> = nl.update(res);
    let keep_going: bool = nl.get_keep_going();

    if keep_going {
        run(nl, remaining_work, new_temp)
    } else {
        nl
    }
}

pub fn annealer(netlist: Netlist, worklist: Vec<Vec<(usize, usize)>>, temperature: f64) -> Netlist {
    run(netlist, worklist, temperature)
}
