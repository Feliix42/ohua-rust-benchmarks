#![allow(non_snake_case, unused_mut)]
use crate::types::*;
use std::sync::Arc;

pub const THREADCOUNT: usize = 4;
pub const FREQUENCY: usize = 7500;

// NOTE(feliix42): Check the following:
// - [x] tokio runtime fix (channel) in place?
// - [x] THREADCOUNT and FREQUENCY variables replaced?

//fn run(
    //mut netlist: Netlist,
    //worklist: Vec<Result<MoveDecision, (usize, usize)>>,
    //temperature: f64,
    //mut internal_state: InternalState,
//) -> Netlist {
    //let mut rs = Vec::new();
    //let new_temp = reduce_temp(temperature);
    //let n2 = netlist.clone();
    //let nro = Arc::new(n2);
    //for item in worklist {
        //let switch_info = process_move(item, nro.clone(), new_temp.clone());
        //let result = netlist.update(switch_info);
        //rs.push(result);
    //}
    //netlist.clear_changes();
    //let rs2 = rs.clone();
    //let mut remaining_work = filter_work(rs);
    //let length = remaining_work.len();
    //let (new_temp2, new_temp3) = dup(new_temp);
    //let (new_work, keep_going) = internal_state.assess_updates(rs2, length);
    //remaining_work.exp(new_work);
    //if keep_going {
        //run(netlist, remaining_work, new_temp3, internal_state)
    //} else {
        //netlist
    //}
//}


pub fn annealer(
    netlist: Netlist,
    elements: usize,
    temperature: f64,
    max_steps: Option<i32>,
    swaps_per_temp: usize,
) -> Netlist {
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
    let (g_0_0_tx, g_0_0_rx) = std::sync::mpsc::channel();
    let (st_0_0_1_tx, st_0_0_1_rx) = std::sync::mpsc::channel::<InternalState>();
    let (netlist_0_1_0_0_tx, netlist_0_1_0_0_rx) = std::sync::mpsc::channel();
    let (keep_going_0_0_0_tx, keep_going_0_0_0_rx) = std::sync::mpsc::channel();
    let (internal_state_0_0_0_0_tx, internal_state_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_temp3_0_0_0_tx, new_temp3_0_0_0_rx) = std::sync::mpsc::channel();
    let (remaining_work_0_0_0_0_tx, remaining_work_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (st_0_0_0_0_tx, st_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (worklist_1_0_0_tx, worklist_1_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (temperature_0_0_0_tx, temperature_0_0_0_rx) = std::sync::mpsc::channel::<f64>();
    let (netlist_0_0_2_tx, netlist_0_0_2_rx) = std::sync::mpsc::channel::<Netlist>();
    let (n2_0_0_0_tx, n2_0_0_0_rx) = std::sync::mpsc::channel::<Netlist>();
    let (worklist_0_0_0_tx, worklist_0_0_0_rx) = std::sync::mpsc::channel();
    let (worklist_0_n_0_0_0_tx, worklist_0_n_0_0_0_rx) =
        std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (netlist_0_0_1_0_tx, netlist_0_0_1_0_rx) = std::sync::mpsc::channel::<Netlist>();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (new_temp_0_0_1_tx, new_temp_0_0_1_rx) = std::sync::mpsc::channel::<f64>();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (nro_0_0_1_tx, nro_0_0_1_rx) = std::sync::mpsc::channel::<Arc<Netlist>>();
    let (ctrl_2_2_tx, ctrl_2_2_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (rs_0_1_1_tx, rs_0_1_1_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (ctrl_2_3_tx, ctrl_2_3_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (c_0_0_tx, c_0_0_rx) = std::sync::mpsc::channel::<f64>();
    let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel::<Arc<Netlist>>();
    let (d_1_0_tx, d_1_0_rx) = std::sync::mpsc::channel::<Result<MoveDecision, (usize, usize)>>();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel::<std::sync::mpsc::Receiver<_>>();
    let (switch_info_0_0_0_tx, switch_info_0_0_0_rx) =
        std::sync::mpsc::channel::<(MoveDecision, (usize, usize))>();
    let (result_0_0_0_tx, result_0_0_0_rx) = std::sync::mpsc::channel::<Result<MoveDecision, (usize, usize)>>();
    let (rest_0_0_0_tx, rest_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (rs_0_2_0_tx, rs_0_2_0_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (netlist_0_1_1_tx, netlist_0_1_1_rx) = std::sync::mpsc::channel::<Netlist>();
    let (rs_0_0_0_1_tx, rs_0_0_0_1_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (rs_0_0_0_0_0_tx, rs_0_0_0_0_0_rx) =
        std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (remaining_work_0_0_2_tx, remaining_work_0_0_2_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (new_temp_0_1_0_tx, new_temp_0_1_0_rx) = std::sync::mpsc::channel::<f64>();
    let (new_temp2_0_0_0_tx, _new_temp2_0_0_0_rx) = std::sync::mpsc::channel::<f64>();
    let (length_0_0_0_tx, length_0_0_0_rx) = std::sync::mpsc::channel::<usize>();
    let (rs2_0_0_0_tx, rs2_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (internal_state_0_0_1_tx, internal_state_0_0_1_rx) =
        std::sync::mpsc::channel::<InternalState>();
    let (new_work_0_0_0_tx, new_work_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let (remaining_work_0_0_1_0_tx, remaining_work_0_0_1_0_rx) =
        std::sync::mpsc::channel::<Vec<Result<MoveDecision, (usize, usize)>>>();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        let st_0_0_1 = InternalState::initialize(elements, max_steps, swaps_per_temp);
        st_0_0_1_tx.send(st_0_0_1)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = internal_state_0_0_1_rx.recv()?;
            let var_1 = rs2_0_0_0_rx.recv()?;
            let var_2 = length_0_0_0_rx.recv()?;
            let restup = var_0.assess_updates(var_1, var_2);
            let new_work_0_0_0 = restup.0;
            new_work_0_0_0_tx.send(new_work_0_0_0)?;
            let keep_going_0_0_0 = restup.1;
            keep_going_0_0_0_tx.send(keep_going_0_0_0)?;
            internal_state_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = n2_0_0_0_rx.recv()?;
            let nro_0_0_1 = Arc::new(var_0);
            nro_0_0_1_tx.send(nro_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = netlist_0_0_2_rx.recv()?;
            let n2_0_0_0 = var_0.clone();
            n2_0_0_0_tx.send(n2_0_0_0)?;
            netlist_0_0_1_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = remaining_work_0_0_2_rx.recv()?;
            let length_0_0_0 = var_0.len();
            length_0_0_0_tx.send(length_0_0_0)?;
            remaining_work_0_0_1_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = rs_0_2_0_rx.recv()?;
            let var_1 = rest_0_0_0_rx.recv()?;
            let rs_0_0_0_1 = {
                var_0.extend(var_1.into_iter());
                var_0
            };
            rs_0_0_0_1_tx.send(rs_0_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let init_1 = worklist_1_0_0_rx.recv()?;
        let init_3 = st_0_0_0_0_rx.recv()?;
        netlist_0_0_2_tx.send(netlist)?;
        worklist_0_0_0_tx.send(init_1)?;
        temperature_0_0_0_tx.send(temperature)?;
        internal_state_0_0_1_tx.send(init_3)?;
        while keep_going_0_0_0_rx.recv()? {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let loop_res_0 = netlist_0_1_0_0_rx.recv()?;
            let loop_res_1 = remaining_work_0_0_0_0_rx.recv()?;
            let loop_res_2 = new_temp3_0_0_0_rx.recv()?;
            let loop_res_3 = internal_state_0_0_0_0_rx.recv()?;
            netlist_0_0_2_tx.send(loop_res_0)?;
            worklist_0_0_0_tx.send(loop_res_1)?;
            temperature_0_0_0_tx.send(loop_res_2)?;
            internal_state_0_0_1_tx.send(loop_res_3)?;
            ()
        }
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        remaining_work_0_0_0_0_rx.recv()?;
        new_temp3_0_0_0_rx.recv()?;
        internal_state_0_0_0_0_rx.recv()?;
        let finalResult = netlist_0_1_0_0_rx.recv()?;
        Ok(g_0_0_tx.send(finalResult)?)
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = temperature_0_0_0_rx.recv()?;
            let new_temp_0_0_1 = reduce_temp(var_0);
            new_temp_0_0_1_tx.send(new_temp_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = netlist_0_1_1_rx.recv()?;
            var_0.clear_changes();
            netlist_0_1_0_0_tx.send(var_0)?
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
        loop {
            let var_0 = rs_0_0_0_0_0_rx.recv()?;
            let remaining_work_0_0_2 = filter_work(var_0);
            remaining_work_0_0_2_tx.send(remaining_work_0_0_2)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = remaining_work_0_0_1_0_rx.recv()?;
            let var_1 = new_work_0_0_0_rx.recv()?;
            var_0.exp(var_1);
            remaining_work_0_0_0_0_tx.send(var_0)?
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
            let mut var_0 = st_0_0_1_rx.recv()?;
            let worklist_1_0_0 = var_0.generate_worklist();
            worklist_1_0_0_tx.send(worklist_1_0_0)?;
            st_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = rs_0_0_0_1_rx.recv()?;
            let rs2_0_0_0 = var_0.clone();
            rs2_0_0_0_tx.send(rs2_0_0_0)?;
            rs_0_0_0_0_0_tx.send(var_0)?
        }
    }));
    let (tokio_sx, tokio_rx) = std::sync::mpsc::channel();
    tasks.push(Box::new(move || -> _ {
        let mut rt = std::sync::Arc::new(
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(THREADCOUNT)
                .build()
                .unwrap(),
        );
        tokio_sx.send(rt.clone()).unwrap();
        loop {
            let var_1 = d_1_0_rx.recv()?;
            let var_2 = d_0_0_rx.recv()?;
            let var_3 = c_0_0_rx.recv()?;
            let futures_0 = {
                let (tx, rx) = std::sync::mpsc::channel();
                let work = async move { tx.send(process_move(var_1, var_2, var_3)).unwrap() };
                rt.spawn(work);
                rx
            };
            futures_0_tx.send(futures_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let rs_0_1_1 = Vec::new();
                    rs_0_1_1_tx.send(rs_0_1_1)?;
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
            let mut var_0 = worklist_0_0_0_rx.recv()?;
            let restup = {
                let sp = if var_0.len() < FREQUENCY { var_0.len() } else { FREQUENCY };
                let chunk = var_0.split_off(sp);
                (var_0, chunk)
            };
            let worklist_0_n_0_0_0 = restup.0;
            worklist_0_n_0_0_0_tx.send(worklist_0_n_0_0_0)?;
            let rest_0_0_0 = restup.1;
            rest_0_0_0_tx.send(rest_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = new_temp_0_1_0_rx.recv()?;
            let restup = dup(var_0);
            let new_temp2_0_0_0 = restup.0;
            new_temp2_0_0_0_tx.send(new_temp2_0_0_0)?;
            let new_temp3_0_0_0 = restup.1;
            new_temp3_0_0_0_tx.send(new_temp3_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut data = worklist_0_n_0_0_0_rx.recv()?;
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
            let mut renew = false;
            let mut rs_0_1_1_0 = rs_0_1_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_3_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = result_0_0_0_rx.recv()?;
                    rs_0_1_1_0.push(var_1);
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            rs_0_2_0_tx.send(rs_0_1_1_0)?;
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
                    let result_0_0_0 = netlist_0_0_1_0_0.update(var_1);
                    result_0_0_0_tx.send(result_0_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            netlist_0_1_1_tx.send(netlist_0_0_1_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut new_temp_0_0_1_0 = new_temp_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let c_0_0 = new_temp_0_0_1_0.clone();
                    c_0_0_tx.send(c_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            new_temp_0_1_0_tx.send(new_temp_0_0_1_0)?;
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
    let _ = tokio_rx.recv().unwrap();
    match g_0_0_rx.recv() {
        Ok(res) => res,
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}

