#![allow(unused_mut, non_snake_case)]
use crate::types::*;
use std::sync::Arc;

pub const THREADCOUNT: usize = 4;

#[allow(dead_code)]
fn run(
    values: Vec<Vec<Value>>,
    centroids: Arc<Vec<Centroid>>,
    threshold: f32,
    iterations: u32,
) -> u32 {
    let mut new_values = Vec::default();
    for v in values {
        let i = reassign_values(v, centroids.clone());
        new_values.push(i);
    }
    let (vals, delta) = evaluate_results(new_values);
    let cont = should_continue(delta, threshold.clone(), iterations.clone());
    let (new_vals, new_centroids) = create_centroids(vals, centroids);
    let inc_iter = inc(iterations);
    if cont {
        run(new_vals, new_centroids, threshold, inc_iter)
    } else {
        iterations
    }
}

pub fn calculate(
    values: Vec<Vec<Value>>,
    centroids: Arc<Vec<Centroid>>,
    threshold: f32,
    iterations: u32,
) -> (u32, usize) {
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
    let (h_0_0_tx, h_0_0_rx) = std::sync::mpsc::channel();
    let (cont_0_0_0_tx, cont_0_0_0_rx) = std::sync::mpsc::channel();
    let (inc_iter_0_0_0_tx, inc_iter_0_0_0_rx) = std::sync::mpsc::channel();
    let (threshold_0_0_0_0_tx, threshold_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_centroids_0_0_0_tx, new_centroids_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_vals_0_0_0_tx, new_vals_0_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (values_0_0_0_tx, values_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Vec<Value>>>();
    let (centroids_0_0_1_tx, centroids_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (new_values_0_0_1_tx, new_values_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (c_0_0_tx, c_0_0_rx) = std::sync::mpsc::channel::<Arc<Vec<Centroid>>>();
    let (d_1_0_tx, d_1_0_rx) = std::sync::mpsc::channel::<Vec<Value>>();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel::<std::sync::mpsc::Receiver<_>>();
    let (i_0_0_0_tx, i_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_values_0_1_0_tx, new_values_0_1_0_rx) =
        std::sync::mpsc::channel::<Vec<(Vec<Value>, u32)>>();
    let (iterations_0_0_1_tx, iterations_0_0_1_rx) = std::sync::mpsc::channel::<u32>();
    let (threshold_0_0_1_tx, threshold_0_0_1_rx) = std::sync::mpsc::channel::<f32>();
    let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel::<u32>();
    let (e_0_0_tx, e_0_0_rx) = std::sync::mpsc::channel::<f32>();
    let (delta_0_0_0_tx, delta_0_0_0_rx) = std::sync::mpsc::channel::<f32>();
    let (centroids_0_1_0_tx, centroids_0_1_0_rx) = std::sync::mpsc::channel::<Arc<Vec<Centroid>>>();
    let (vals_0_0_0_tx, vals_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Vec<Value>>>();
    let (iterations_0_0_0_0_tx, iterations_0_0_0_0_rx) = std::sync::mpsc::channel::<u32>();
    let (tokio_tx, tokio_rx) = std::sync::mpsc::channel();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = futures_0_rx.recv()?;
            let i_0_0_0 = var_0.recv().unwrap();
            i_0_0_0_tx.send(i_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = iterations_0_0_0_0_rx.recv()?;
            let inc_iter_0_0_0 = inc(var_0);
            inc_iter_0_0_0_tx.send(inc_iter_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = iterations_0_0_1_rx.recv()?;
            let d_0_0 = var_0.clone();
            d_0_0_tx.send(d_0_0)?;
            iterations_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = new_values_0_1_0_rx.recv()?;
            let restup = evaluate_results(var_0);
            let vals_0_0_0 = restup.0;
            vals_0_0_0_tx.send(vals_0_0_0)?;
            let delta_0_0_0 = restup.1;
            delta_0_0_0_tx.send(delta_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = threshold_0_0_1_rx.recv()?;
            let e_0_0 = var_0.clone();
            e_0_0_tx.send(e_0_0)?;
            threshold_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut data = values_0_0_0_rx.recv()?;
            let hasSize = {
                let tmp_has_size = data.iter().size_hint();
                tmp_has_size.1.is_some()
            };
            if hasSize {
                let size = data.len();
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
                    ctrl_2_1_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_0_tx.send(ctrl)?;
                    size = size + 1;
                    ()
                }
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
            let var_0 = vals_0_0_0_rx.recv()?;
            let var_1 = centroids_0_1_0_rx.recv()?;
            let restup = create_centroids(var_0, var_1);
            let new_vals_0_0_0 = restup.0;
            new_vals_0_0_0_tx.send(new_vals_0_0_0)?;
            let new_centroids_0_0_0 = restup.1;
            new_centroids_0_0_0_tx.send(new_centroids_0_0_0)?;
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
                    let new_values_0_0_1 = Vec::default();
                    new_values_0_0_1_tx.send(new_values_0_0_1)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
        }
    }));
    let (ctr_sx, ctr_rx) = std::sync::mpsc::channel();
    tasks.push(Box::new(move || -> _ {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        ctr_sx.send(values.iter().map(Vec::len).sum::<usize>())?;
        values_0_0_0_tx.send(values)?;
        centroids_0_0_1_tx.send(centroids)?;
        threshold_0_0_1_tx.send(threshold)?;
        iterations_0_0_1_tx.send(iterations)?;
        while cont_0_0_0_rx.recv()? {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let loop_res_0 = new_vals_0_0_0_rx.recv()?;
            let loop_res_1 = new_centroids_0_0_0_rx.recv()?;
            let loop_res_2 = threshold_0_0_0_0_rx.recv()?;
            let loop_res_3 = inc_iter_0_0_0_rx.recv()?;
            ctr_sx.send(loop_res_0.iter().map(Vec::len).sum::<usize>())?;
            values_0_0_0_tx.send(loop_res_0)?;
            centroids_0_0_1_tx.send(loop_res_1)?;
            threshold_0_0_1_tx.send(loop_res_2)?;
            iterations_0_0_1_tx.send(loop_res_3)?;
            ()
        }
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        new_vals_0_0_0_rx.recv()?;
        new_centroids_0_0_0_rx.recv()?;
        threshold_0_0_0_0_rx.recv()?;
        let finalResult = inc_iter_0_0_0_rx.recv()?;
        Ok(h_0_0_tx.send(finalResult)?)
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = delta_0_0_0_rx.recv()?;
            let var_1 = e_0_0_rx.recv()?;
            let var_2 = d_0_0_rx.recv()?;
            let cont_0_0_0 = should_continue(var_0, var_1, var_2);
            cont_0_0_0_tx.send(cont_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let mut rt = std::sync::Arc::new(
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(THREADCOUNT)
                .build()
                .unwrap(),
        );
        tokio_tx.send(rt.clone()).unwrap();
        loop {
            let var_1 = d_1_0_rx.recv()?;
            let var_2 = c_0_0_rx.recv()?;
            let futures_0 = {
                let (tx, rx) = std::sync::mpsc::channel();
                let work = async move { tx.send(reassign_values(var_1, var_2)).unwrap() };
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
            let mut new_values_0_0_1_0 = new_values_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = i_0_0_0_rx.recv()?;
                    new_values_0_0_1_0.push(var_1);
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            new_values_0_1_0_tx.send(new_values_0_0_1_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut centroids_0_0_1_0 = centroids_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let c_0_0 = centroids_0_0_1_0.clone();
                    c_0_0_tx.send(c_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            centroids_0_1_0_tx.send(centroids_0_0_1_0)?;
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
    let comps = Iterator::sum(ctr_rx.iter());
    match h_0_0_rx.recv() {
        Ok(res) => (res, comps),
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}
