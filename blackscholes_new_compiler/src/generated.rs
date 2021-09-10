#![allow(unused_mut, non_snake_case)]
use crate::types::*;

pub const THREADCOUNT: usize = 12;

pub fn calculate(options: Vec<Vec<OptionData>>) -> Vec<f32> {
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
    let (b_0_0_tx, b_0_0_rx) = std::sync::mpsc::channel();
    let (results_0_0_1_tx, results_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_tx, ctrl_0_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (d_1_0_tx, d_1_0_rx) = std::sync::mpsc::channel::<Vec<OptionData>>();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel::<std::sync::mpsc::Receiver<_>>();
    let (i_0_0_0_tx, i_0_0_0_rx) = std::sync::mpsc::channel();
    let (results_0_1_0_tx, results_0_1_0_rx) = std::sync::mpsc::channel::<Vec<Vec<f32>>>();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        let mut rt = std::sync::Arc::new(
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(1)
                .build()
                .unwrap(),
        );
        loop {
            let var_1 = d_1_0_rx.recv()?;
            let futures_0 = {
                let (tx, rx) = std::sync::mpsc::channel();
                let work = async move { tx.send(batch_calculate_black_scholes(var_1)).unwrap() };
                rt.spawn(work);
                rx
            };
            futures_0_tx.send(futures_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = futures_0_rx.recv()?;
            let i_0_0_0 = var_0.recv().unwrap();
            i_0_0_0_tx.send(i_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let results_0_0_1 = Vec::new();
        results_0_0_1_tx.send(results_0_0_1)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        let mut data = options;
        let hasSize = {
            let tmp_has_size = data.iter().size_hint();
            tmp_has_size.1.is_some()
        };
        if hasSize {
            let size = data.len();
            let ctrl = (true, size);
            ctrl_0_0_tx.send(ctrl)?;
            for d in data {
                d_1_0_tx.send(d)?;
                ()
            }
        } else {
            let mut size = 0;
            for d in data {
                d_1_0_tx.send(d)?;
                let ctrl = (false, 1);
                ctrl_0_0_tx.send(ctrl)?;
                size = size + 1;
                ()
            }
            let ctrl = (true, 0);
            ctrl_0_0_tx.send(ctrl)?;
            ()
        }
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = results_0_1_0_rx.recv()?;
            let b_0_0 = unpack(var_0);
            b_0_0_tx.send(b_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut results_0_0_1_0 = results_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = i_0_0_0_rx.recv()?;
                    results_0_0_1_0.push(var_1);
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            results_0_1_0_tx.send(results_0_0_1_0)?;
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
    match b_0_0_rx.recv() {
        Ok(res) => res,
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}
