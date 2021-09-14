#![allow(non_snake_case, unused_mut)]
use crate::types::*;
use std::sync::Arc;

pub const THREADCOUNT: usize = 4;

// NOTE(feliix42): Check the following:
// - [ ] tokio runtime fix (channel) in place?
// - [ ] THREADCOUNT and FREQUENCY variables replaced?

/* // Old design sketch:
pub fn run(mut netlist: Netlist, dimensions: Location, worklist: Vec<(usize, usize)>, rng: ChaCha12Rng, temperature: f64, completed_steps: i32, max_steps: Option<i32>, swaps_per_temp: usize) -> i32 { 
    let mut rs = Vec::default();

    let new_temp = reduce_temp(temperature);

    let nro = Arc::new(netlist.clone());
    for item in worklist {
        let switch_info = process_move(item, nro.clone()); // gives a Good/Bad/Reject plus tuple
        let res = netlist.update(switch_info); // produces a Option<(_, _)> with the update info (success/fail)
        // TODO: How to track overrides??
        rs.push(res);
    }

    let (keep_going, rest, new_rng) = assess_updates(rs, dimensions.clone(), new_temp.clone(), completed_steps.clone(), max_steps.clone(), swaps_per_temp.clone(), rng);

    let new_temp_steps = increment(completed_steps);
    if keep_going {
        run(netlist, dimensions, rest, new_rng, new_temp, new_temp_steps, max_steps, swaps_per_temp)
    } else {
        new_temp_steps
    }
}

pub fn annealer(netlist: Netlist, dimensions: Location, completed_steps: i32, temperature: f64, max_steps: Option<i32>, swaps_per_temp: usize) -> i32 {
    let rng = ChaCha12Rng::seed_from_u64(0);

    let (worklist, new_rng) = generate_worklist(swaps_per_temp, dimensions.clone(), rng);
    run(netlist, dimensions, worklist, new_rng, temperature, completed_steps, max_steps, swaps_per_temp)
}
*/

#[allow(dead_code)]
pub fn run(
    mut netlist: Netlist,
    dimensions: Location,
    worklist: Vec<(usize, usize)>,
    mut rng: InternalRNG,
    temperature: f64,
    completed_steps: i32,
    max_steps: Option<i32>,
    swaps_per_temp: usize,
) -> i32 {
    let mut rs = Vec::default();
    let new_temp = reduce_temp(temperature);
    let new_temp2 = new_temp.clone();
    let nro = Arc::new(netlist.clone());
    for item in worklist {
        let switch_info = process_move(item, nro.clone(), new_temp2);
        let res = netlist.update(switch_info);
        rs.push(res);
    }
    let dim2 = dimensions.clone();
    let (keep_going, rest) = rng.assess_updates(
        rs,
        dimensions,
        new_temp.clone(),
        completed_steps.clone(),
        max_steps.clone(),
        swaps_per_temp.clone(),
    );
    let new_temp_steps = increment(completed_steps);
    if keep_going {
        run(
            netlist,
            dim2,
            rest,
            rng,
            new_temp,
            new_temp_steps,
            max_steps,
            swaps_per_temp,
        )
    } else {
        new_temp_steps
    }
}

pub fn annealer(
    netlist: Netlist,
    dimensions: Location,
    temperature: f64,
    completed_steps: i32,
    max_steps: Option<i32>,
    swaps_per_temp: usize,
) -> i32 {
    #[derive(Debug)]
    enum RunError {
        SendFailed,
        RecvFailed,
    }
    impl<T: Send> From<std::sync::mpsc::SendError<T>> for RunError {
        fn from(_err: std::sync::mpsc::SendError<T>) -> Self {
            RunError::SendFailed
        }
    }
    impl From<std::sync::mpsc::RecvError> for RunError {
        fn from(_err: std::sync::mpsc::RecvError) -> Self {
            RunError::RecvFailed
        }
    }
    let (k_0_0_tx, k_0_0_rx) = std::sync::mpsc::channel();
    let (d1_0_0_0_tx, d1_0_0_0_rx) = std::sync::mpsc::channel::<Location>();
    let (rng_1_0_1_tx, rng_1_0_1_rx) = std::sync::mpsc::channel::<InternalRNG>();
    let (new_temp_steps_0_0_0_tx, new_temp_steps_0_0_0_rx) = std::sync::mpsc::channel();
    let (keep_going_0_0_0_tx, keep_going_0_0_0_rx) = std::sync::mpsc::channel();
    let (swaps_per_temp_0_0_0_0_tx, swaps_per_temp_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (max_steps_0_0_0_0_tx, max_steps_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_temp_0_0_0_0_tx, new_temp_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (rng_0_0_0_0_tx, rng_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (rest_0_0_0_tx, rest_0_0_0_rx) = std::sync::mpsc::channel();
    let (dim2_0_0_0_tx, dim2_0_0_0_rx) = std::sync::mpsc::channel();
    let (netlist_0_1_0_tx, netlist_0_1_0_rx) = std::sync::mpsc::channel();
    let (rng_1_0_0_0_tx, rng_1_0_0_0_rx) = std::sync::mpsc::channel();
    let (worklist_1_0_0_tx, worklist_1_0_0_rx) = std::sync::mpsc::channel();
    let (d2_0_0_0_tx, d2_0_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (temperature_0_0_0_tx, temperature_0_0_0_rx) = std::sync::mpsc::channel::<f64>();
    let (new_temp_0_0_2_tx, new_temp_0_0_2_rx) = std::sync::mpsc::channel::<f64>();
    let (netlist_0_0_2_tx, netlist_0_0_2_rx) = std::sync::mpsc::channel();
    let (c_0_0_tx, c_0_0_rx) = std::sync::mpsc::channel();
    let (worklist_0_0_0_tx, worklist_0_0_0_rx) = std::sync::mpsc::channel::<Vec<(usize, usize)>>();
    let (netlist_0_0_1_0_tx, netlist_0_0_1_0_rx) = std::sync::mpsc::channel::<Netlist>();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (new_temp2_0_0_0_tx, new_temp2_0_0_0_rx) = std::sync::mpsc::channel::<f64>();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (nro_0_0_1_tx, nro_0_0_1_rx) = std::sync::mpsc::channel::<Arc<Netlist>>();
    let (ctrl_2_2_tx, ctrl_2_2_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (rs_0_0_1_tx, rs_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_2_3_tx, ctrl_2_3_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel::<Arc<Netlist>>();
    let (d_1_0_tx, d_1_0_rx) = std::sync::mpsc::channel::<(usize, usize)>();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel();
    let (switch_info_0_0_0_tx, switch_info_0_0_0_rx) =
        std::sync::mpsc::channel::<(MoveDecision, (usize, usize))>();
    let (res_0_0_0_tx, res_0_0_0_rx) = std::sync::mpsc::channel();
    let (dimensions_0_0_1_tx, dimensions_0_0_1_rx) = std::sync::mpsc::channel::<Location>();
    let (swaps_per_temp_0_0_1_tx, swaps_per_temp_0_0_1_rx) = std::sync::mpsc::channel::<usize>();
    let (max_steps_0_0_1_tx, max_steps_0_0_1_rx) = std::sync::mpsc::channel::<Option<i32>>();
    let (completed_steps_0_0_1_tx, completed_steps_0_0_1_rx) = std::sync::mpsc::channel::<i32>();
    let (new_temp_0_0_1_0_tx, new_temp_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (e_0_0_tx, e_0_0_rx) = std::sync::mpsc::channel::<usize>();
    let (f_0_0_tx, f_0_0_rx) = std::sync::mpsc::channel::<Option<i32>>();
    let (g_0_0_tx, g_0_0_rx) = std::sync::mpsc::channel::<i32>();
    let (h_0_0_tx, h_0_0_rx) = std::sync::mpsc::channel::<f64>();
    let (dimensions_0_0_0_0_tx, dimensions_0_0_0_0_rx) = std::sync::mpsc::channel::<Location>();
    let (rs_0_1_0_tx, rs_0_1_0_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (rng_0_0_1_tx, rng_0_0_1_rx) = std::sync::mpsc::channel::<InternalRNG>();
    let (completed_steps_0_0_0_0_tx, completed_steps_0_0_0_0_rx) =
        std::sync::mpsc::channel::<i32>();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = completed_steps_0_0_1_rx.recv()?;
            let g_0_0 = var_0.clone();
            g_0_0_tx.send(g_0_0)?;
            completed_steps_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let rs_0_0_1 = Vec::default();
                    rs_0_0_1_tx.send(rs_0_0_1)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = rng_1_0_1_rx.recv()?;
            let var_2 = d1_0_0_0_rx.recv()?;
            let worklist_1_0_0 = var_0.generate_worklist(swaps_per_temp, var_2);
            worklist_1_0_0_tx.send(worklist_1_0_0)?;
            rng_1_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = new_temp_0_0_2_rx.recv()?;
            let new_temp2_0_0_0 = var_0.clone();
            new_temp2_0_0_0_tx.send(new_temp2_0_0_0)?;
            new_temp_0_0_1_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let restup = dup(dimensions);
        let d1_0_0_0 = restup.0;
        d1_0_0_0_tx.send(d1_0_0_0)?;
        let d2_0_0_0 = restup.1;
        d2_0_0_0_tx.send(d2_0_0_0)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut data = worklist_0_0_0_rx.recv()?;
            let hasSize = {
                let tmp_has_size = data.iter().size_hint();
                tmp_has_size.1.is_some()
            };
            if hasSize {
                let size = data.len();
                let ctrl = (true, size);
                ctrl_2_3_tx.send(ctrl)?;
                let ctrl = (true, size);
                ctrl_2_2_tx.send(ctrl)?;
                let ctrl = (true, size);
                ctrl_2_1_tx.send(ctrl)?;
                let ctrl = (true, size);
                ctrl_2_0_tx.send(ctrl)?;
                for d in data {
                    d_1_0_tx.send(d)?;
                    ()
                }
            } else {
                let mut size = 0;
                for d in data {
                    d_1_0_tx.send(d)?;
                    let ctrl = (false, 1);
                    ctrl_2_3_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_2_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_1_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_0_tx.send(ctrl)?;
                    size = size + 1;
                    ()
                }
                let ctrl = (true, 0);
                ctrl_2_3_tx.send(ctrl)?;
                let ctrl = (true, 0);
                ctrl_2_2_tx.send(ctrl)?;
                let ctrl = (true, 0);
                ctrl_2_1_tx.send(ctrl)?;
                let ctrl = (true, 0);
                ctrl_2_0_tx.send(ctrl)?;
                ()
            }
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = max_steps_0_0_1_rx.recv()?;
            let f_0_0 = var_0.clone();
            f_0_0_tx.send(f_0_0)?;
            max_steps_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = dimensions_0_0_1_rx.recv()?;
            let dim2_0_0_0 = var_0.clone();
            dim2_0_0_0_tx.send(dim2_0_0_0)?;
            dimensions_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = completed_steps_0_0_0_0_rx.recv()?;
            let new_temp_steps_0_0_0 = increment(var_0);
            new_temp_steps_0_0_0_tx.send(new_temp_steps_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = swaps_per_temp_0_0_1_rx.recv()?;
            let e_0_0 = var_0.clone();
            e_0_0_tx.send(e_0_0)?;
            swaps_per_temp_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = rng_0_0_1_rx.recv()?;
            let var_1 = rs_0_1_0_rx.recv()?;
            let var_2 = dimensions_0_0_0_0_rx.recv()?;
            let var_3 = h_0_0_rx.recv()?;
            let var_4 = g_0_0_rx.recv()?;
            let var_5 = f_0_0_rx.recv()?;
            let var_6 = e_0_0_rx.recv()?;
            let restup = var_0.assess_updates(var_1, var_2, var_3, var_4, var_5, var_6);
            let keep_going_0_0_0 = restup.0;
            keep_going_0_0_0_tx.send(keep_going_0_0_0)?;
            let rest_0_0_0 = restup.1;
            rest_0_0_0_tx.send(rest_0_0_0)?;
            rng_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut nro_0_0_1_0 = nro_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_2_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let d_0_0 = nro_0_0_1_0.clone();
                    d_0_0_tx.send(d_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let mut rt = std::sync::Arc::new(
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(1)
                .build()
                .unwrap(),
        );
        loop {
            let mut renew = false;
            let new_temp2_0_0_0_0 = new_temp2_0_0_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = d_1_0_rx.recv()?;
                    let var_2 = d_0_0_rx.recv()?;
                    let futures_0 = {
                        let (tx, rx) = std::sync::mpsc::channel();
                        let work = async move {
                            tx.send(process_move(var_1, var_2, new_temp2_0_0_0_0))
                                .unwrap()
                        };
                        rt.spawn(work);
                        rx
                    };
                    futures_0_tx.send(futures_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let init_1 = d2_0_0_0_rx.recv()?;
        let init_2 = worklist_1_0_0_rx.recv()?;
        let init_3 = rng_1_0_0_0_rx.recv()?;
        netlist_0_0_2_tx.send(netlist)?;
        dimensions_0_0_1_tx.send(init_1)?;
        worklist_0_0_0_tx.send(init_2)?;
        rng_0_0_1_tx.send(init_3)?;
        temperature_0_0_0_tx.send(temperature)?;
        completed_steps_0_0_1_tx.send(completed_steps)?;
        max_steps_0_0_1_tx.send(max_steps)?;
        swaps_per_temp_0_0_1_tx.send(swaps_per_temp)?;
        while keep_going_0_0_0_rx.recv()? {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let loop_res_0 = netlist_0_1_0_rx.recv()?;
            let loop_res_1 = dim2_0_0_0_rx.recv()?;
            let loop_res_2 = rest_0_0_0_rx.recv()?;
            let loop_res_3 = rng_0_0_0_0_rx.recv()?;
            let loop_res_4 = new_temp_0_0_0_0_rx.recv()?;
            let loop_res_5 = new_temp_steps_0_0_0_rx.recv()?;
            let loop_res_6 = max_steps_0_0_0_0_rx.recv()?;
            let loop_res_7 = swaps_per_temp_0_0_0_0_rx.recv()?;
            netlist_0_0_2_tx.send(loop_res_0)?;
            dimensions_0_0_1_tx.send(loop_res_1)?;
            worklist_0_0_0_tx.send(loop_res_2)?;
            rng_0_0_1_tx.send(loop_res_3)?;
            temperature_0_0_0_tx.send(loop_res_4)?;
            completed_steps_0_0_1_tx.send(loop_res_5)?;
            max_steps_0_0_1_tx.send(loop_res_6)?;
            swaps_per_temp_0_0_1_tx.send(loop_res_7)?;
            ()
        }
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        netlist_0_1_0_rx.recv()?;
        dim2_0_0_0_rx.recv()?;
        rest_0_0_0_rx.recv()?;
        rng_0_0_0_0_rx.recv()?;
        new_temp_0_0_0_0_rx.recv()?;
        max_steps_0_0_0_0_rx.recv()?;
        swaps_per_temp_0_0_0_0_rx.recv()?;
        let finalResult = new_temp_steps_0_0_0_rx.recv()?;
        Ok(k_0_0_tx.send(finalResult)?)
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = c_0_0_rx.recv()?;
            let nro_0_0_1 = Arc::new(var_0);
            nro_0_0_1_tx.send(nro_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = futures_0_rx.recv()?;
            let switch_info_0_0_0 = var_0.recv().unwrap();
            switch_info_0_0_0_tx.send(switch_info_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = netlist_0_0_2_rx.recv()?;
            let c_0_0 = var_0.clone();
            c_0_0_tx.send(c_0_0)?;
            netlist_0_0_1_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = temperature_0_0_0_rx.recv()?;
            let new_temp_0_0_2 = reduce_temp(var_0);
            new_temp_0_0_2_tx.send(new_temp_0_0_2)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = new_temp_0_0_1_0_rx.recv()?;
            let h_0_0 = var_0.clone();
            h_0_0_tx.send(h_0_0)?;
            new_temp_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let rng_1_0_1 = InternalRNG::seed_from_u64(0);
        rng_1_0_1_tx.send(rng_1_0_1)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut rs_0_0_1_0 = rs_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_3_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = res_0_0_0_rx.recv()?;
                    rs_0_0_1_0.push(var_1);
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            rs_0_1_0_tx.send(rs_0_0_1_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut netlist_0_0_1_0_0 = netlist_0_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = switch_info_0_0_0_rx.recv()?;
                    let res_0_0_0 = netlist_0_0_1_0_0.update(var_1);
                    res_0_0_0_tx.send(res_0_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            netlist_0_1_0_tx.send(netlist_0_0_1_0_0)?;
            ()
        }
    }));
    let handles: Vec<std::thread::JoinHandle<_>> = tasks
        .into_iter()
        .map(|t| {
            std::thread::spawn(move || {
                let _ = t();
            })
        })
        .collect();
    for h in handles {
        if let Err(_) = h.join() {
            eprintln!("[Error] A worker thread of an Ohua algorithm has panicked!");
        }
    }
    match k_0_0_rx.recv() {
        Ok(res) => res,
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}
