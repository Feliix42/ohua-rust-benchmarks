use functions::*;

use helpers;

use std::*;

/**
Note that in Galois, EVERY function to the state (graph) has to declare whether the access is UNPROTECTED or WRITE.
That is, it is left to the developer to do the synchronization!
Even more so, the iterators have unclear semantics and there are places in the code where the developer
has to explicitly "lock" state!
 */
pub fn nondet_discharge(
    mut graph: Graph,
    mut counter: Counter,
    initial: HashSet<NodeID>,
    preflow: PreflowPush,
) -> Graph {
    let mut results = Vec::new2();
    let g2 = Arc::new1(graph.clone());
    for src in initial {
        let result = discharge(g2.clone(), src);
        results.push(result);
    }
    let (relabel_count, wl_new) = graph.update(results);
    let should_global_relabel = counter.detect_global_relabel(relabel_count, preflow);
    let wl_new0 = graph.global_relabel(should_global_relabel);
    let (not_empty, wl_new1) = helpers::combine(wl_new, wl_new0);
    if not_empty {
        nondet_discharge(graph, counter, wl_new1, preflow)
    } else {
        graph
    }
}

pub fn run(g: Graph, preflow: PreflowPush) -> Graph {
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
    let (i_0_0_tx, i_0_0_rx) = std::sync::mpsc::channel();
    let (graph_1_0_1_tx, graph_1_0_1_rx) = std::sync::mpsc::channel::<Graph>();
    let (graph_0_0_0_0_tx, graph_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (not_empty_0_0_0_tx, not_empty_0_0_0_rx) = std::sync::mpsc::channel();
    let (wl_new1_0_0_0_tx, wl_new1_0_0_0_rx) = std::sync::mpsc::channel();
    let (counter_0_0_0_0_tx, counter_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (initial_1_0_0_tx, initial_1_0_0_rx) = std::sync::mpsc::channel();
    let (c_0_0_tx, c_0_0_rx) = std::sync::mpsc::channel();
    let (graph_1_0_0_0_tx, graph_1_0_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel();
    let (graph_0_0_3_tx, graph_0_0_3_rx) = std::sync::mpsc::channel::<Arc<T>>();
    let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel::<T>();
    let (initial_0_0_0_tx, initial_0_0_0_rx) = std::sync::mpsc::channel::<NodeID>();
    let (g2_0_0_1_tx, g2_0_0_1_rx) = std::sync::mpsc::channel::<Arc<T>>();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (results_0_0_1_tx, results_0_0_1_rx) = std::sync::mpsc::channel::<Vec<T>>();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (d_1_tx, d_1_rx) = std::sync::mpsc::channel::<NodeID>();
    let (e_0_0_tx, e_0_0_rx) = std::sync::mpsc::channel::<&Graph>();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel();
    let (result_0_0_0_tx, result_0_0_0_rx) = std::sync::mpsc::channel::<Duration>();
    let (results_0_1_0_tx, results_0_1_0_rx) =
        std::sync::mpsc::channel::<Vec<(u64, (NodeID, Graph))>>();
    let (graph_0_0_2_0_tx, graph_0_0_2_0_rx) = std::sync::mpsc::channel::<Graph>();
    let (preflow_0_0_0_tx, preflow_0_0_0_rx) = std::sync::mpsc::channel::<PreflowPush>();
    let (relabel_count_0_0_0_tx, relabel_count_0_0_0_rx) = std::sync::mpsc::channel::<u64>();
    let (counter_0_0_1_tx, counter_0_0_1_rx) = std::sync::mpsc::channel::<Counter>();
    let (should_global_relabel_0_0_0_tx, should_global_relabel_0_0_0_rx) =
        std::sync::mpsc::channel::<bool>();
    let (graph_0_0_1_0_tx, graph_0_0_1_0_rx) = std::sync::mpsc::channel::<Graph>();
    let (wl_new0_0_0_0_tx, wl_new0_0_0_0_rx) = std::sync::mpsc::channel::<HashSet<NodeID>>();
    let (wl_new_0_0_0_tx, wl_new_0_0_0_rx) = std::sync::mpsc::channel::<HashSet<NodeID>>();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = graph_0_0_1_0_rx.recv()?;
            let var_1 = should_global_relabel_0_0_0_rx.recv()?;
            let wl_new0_0_0_0 = var_0.global_relabel(var_1);
            wl_new0_0_0_0_tx.send(wl_new0_0_0_0)?;
            graph_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let results_0_0_1 = Vec::new2();
                    results_0_0_1_tx.send(results_0_0_1)?;
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
            let mut var_0 = graph_1_0_1_rx.recv()?;
            let initial_1_0_0 = var_0.initialize_preflow();
            initial_1_0_0_tx.send(initial_1_0_0)?;
            graph_1_0_0_0_tx.send(var_0)?
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
            let var_1 = e_0_0_rx.recv()?;
            let var_2 = d_1_rx.recv()?;
            let futures_0 = {
                let (tx, rx) = std::sync::mpsc::channel();
                let work = async move { tx.send(discharge(var_1, var_2)).unwrap() };
                rt.spawn(work);
                rx
            };
            futures_0_tx.send(futures_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = counter_0_0_1_rx.recv()?;
            let var_1 = relabel_count_0_0_0_rx.recv()?;
            let var_2 = preflow_0_0_0_rx.recv()?;
            let should_global_relabel_0_0_0 = var_0.detect_global_relabel(var_1, var_2);
            should_global_relabel_0_0_0_tx.send(should_global_relabel_0_0_0)?;
            counter_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut data = initial_0_0_0_rx.recv()?;
            let hasSize = {
                let tmp_has_size = data.iter().size_hint();
                tmp_has_size.1.is_some()
            };
            if hasSize {
                let size = data.len();
                let ctrl = (true, size);
                ctrl_2_0_tx.send(ctrl)?;
                let ctrl = (true, size);
                ctrl_2_1_tx.send(ctrl)?;
                for d in data {
                    d_1_tx.send(d)?;
                    ()
                }
            } else {
                let mut size = 0;
                for d in data {
                    d_1_tx.send(d)?;
                    let ctrl = (false, 1);
                    ctrl_2_0_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_1_tx.send(ctrl)?;
                    size = size + 1;
                    ()
                }
                let ctrl = (true, 0);
                ctrl_2_0_tx.send(ctrl)?;
                let ctrl = (true, 0);
                ctrl_2_1_tx.send(ctrl)?;
                ()
            }
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = graph_0_0_3_rx.recv()?;
            let d_0_0 = var_0.clone();
            d_0_0_tx.send(d_0_0)?;
            graph_0_0_2_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = wl_new_0_0_0_rx.recv()?;
            let var_1 = wl_new0_0_0_0_rx.recv()?;
            let res = helpers::combine(var_0, var_1);
            let not_empty_0_0_0 = res.0;
            not_empty_0_0_0_tx.send(not_empty_0_0_0)?;
            let wl_new1_0_0_0 = res.1;
            wl_new1_0_0_0_tx.send(wl_new1_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let init_0 = graph_1_0_0_0_rx.recv()?;
        let init_1 = c_0_0_rx.recv()?;
        let init_2 = initial_1_0_0_rx.recv()?;
        graph_0_0_3_tx.send(init_0)?;
        counter_0_0_1_tx.send(init_1)?;
        initial_0_0_0_tx.send(init_2)?;
        preflow_0_0_0_tx.send(preflow)?;
        while not_empty_0_0_0_rx.recv()? {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let loop_res_0 = graph_0_0_0_0_rx.recv()?;
            let loop_res_1 = counter_0_0_0_0_rx.recv()?;
            let loop_res_2 = wl_new1_0_0_0_rx.recv()?;
            let loop_res_3 = preflow_0_0_0_rx.recv()?;
            graph_0_0_3_tx.send(loop_res_0)?;
            counter_0_0_1_tx.send(loop_res_1)?;
            initial_0_0_0_tx.send(loop_res_2)?;
            preflow_0_0_0_tx.send(loop_res_3)?;
            ()
        }
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        counter_0_0_0_0_rx.recv()?;
        wl_new1_0_0_0_rx.recv()?;
        preflow_0_0_0_rx.recv()?;
        let finalResult = graph_0_0_0_0_rx.recv()?;
        Ok(i_0_0_tx.send(finalResult)?)
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = graph_0_0_2_0_rx.recv()?;
            let var_1 = results_0_1_0_rx.recv()?;
            let res = var_0.update(var_1);
            let relabel_count_0_0_0 = res.0;
            relabel_count_0_0_0_tx.send(relabel_count_0_0_0)?;
            let wl_new_0_0_0 = res.1;
            wl_new_0_0_0_tx.send(wl_new_0_0_0)?;
            graph_0_0_1_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let c_0_0 = Counter::default();
        c_0_0_tx.send(c_0_0)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = d_0_0_rx.recv()?;
            let g2_0_0_1 = Arc::new1(var_0);
            g2_0_0_1_tx.send(g2_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = futures_0_rx.recv()?;
            let result_0_0_0 = var_0.recv().unwrap();
            result_0_0_0_tx.send(result_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut g2_0_0_1_0 = g2_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let e_0_0 = g2_0_0_1_0.clone();
                    e_0_0_tx.send(e_0_0)?;
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
        let graph_1_0_1 = id(g);
        graph_1_0_1_tx.send(graph_1_0_1)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut results_0_0_1_0 = results_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = result_0_0_0_rx.recv()?;
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
    match i_0_0_rx.recv() {
        Ok(res) => res,
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}
