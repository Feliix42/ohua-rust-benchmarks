#![allow(unused_mut, non_snake_case)]
// just for peace of mind

pub const THREADCOUNT: usize = 12;
pub const FREQUENCY: usize = 1;

use crate::benchs::*;
use std::sync::Arc;

// fn fill(
//   maze: Maze,
//   pairs: Vec<  Option<  (Point, Point),>,>,
//   its: u32,
// ) -> Maze {
//   let rs = Vec::default();
//   let m2 = maze.clone();
//   let mro = Arc::new(m2);
//   for pair in pairs { let path = find_path(mro.clone(), pair); rs.push(path); }
//   let rs1 = maze.update(rs);
//   let rs2 = rs1.clone();
//   let (new_its, not_done) = calculate_done(rs1, its);
//   if not_done { fill(maze, rs2, new_its) } else { maze }
// }

pub fn run(dimensions: Point, pairs: Vec<Option<(Point, Point)>>, max_it: u32) -> (Maze, usize) {
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
    let (e_0_0_tx, e_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_0_0_tx, maze_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (not_done_0_0_0_tx, not_done_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_its_0_0_0_tx, new_its_0_0_0_rx) = std::sync::mpsc::channel();
    let (rs2_0_0_0_tx, rs2_0_0_0_rx) = std::sync::mpsc::channel();
    let (maze_1_0_0_tx, maze_1_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (maze_0_0_2_tx, maze_0_0_2_rx) = std::sync::mpsc::channel::<Maze>();
    let (m2_0_0_0_tx, m2_0_0_0_rx) = std::sync::mpsc::channel::<Maze>();
    let (pairs_0_0_0_tx, pairs_0_0_0_rx) = std::sync::mpsc::channel();
    let (pairs_0_n_0_0_0_tx, pairs_0_n_0_0_0_rx) =
        std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    let (mro_0_0_1_tx, mro_0_0_1_rx) = std::sync::mpsc::channel::<Arc<Maze>>();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (rs_0_0_1_tx, rs_0_0_1_rx) = std::sync::mpsc::channel::<Vec<Option<Path>>>();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (d_0_tx, d_0_rx) = std::sync::mpsc::channel::<Option<(Point, Point)>>();
    let (b_0_0_tx, b_0_0_rx) = std::sync::mpsc::channel::<Arc<Maze>>();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel::<std::sync::mpsc::Receiver<_>>();
    let (path_0_0_0_tx, path_0_0_0_rx) = std::sync::mpsc::channel::<Option<Path>>();
    let (rs_0_1_0_tx, rs_0_1_0_rx) = std::sync::mpsc::channel::<Vec<Option<Path>>>();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel::<Maze>();
    let (rest_0_0_0_tx, rest_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    let (rs1_0_1_0_tx, rs1_0_1_0_rx) = std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    let (rs1_0_0_0_1_tx, rs1_0_0_0_1_rx) =
        std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    let (its_0_0_0_tx, its_0_0_0_rx) = std::sync::mpsc::channel::<u32>();
    let (rs1_0_0_0_0_0_tx, rs1_0_0_0_0_0_rx) =
        std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = futures_0_rx.recv()?;
            let path_0_0_0 = var_0.recv().unwrap();
            path_0_0_0_tx.send(path_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = m2_0_0_0_rx.recv()?;
            let mro_0_0_1 = Arc::new(var_0);
            mro_0_0_1_tx.send(mro_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = rs1_0_0_0_0_0_rx.recv()?;
            let var_1 = its_0_0_0_rx.recv()?;
            let res = calculate_done(var_0, var_1);
            let new_its_0_0_0 = res.0;
            new_its_0_0_0_tx.send(new_its_0_0_0)?;
            let not_done_0_0_0 = res.1;
            not_done_0_0_0_tx.send(not_done_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = rs1_0_1_0_rx.recv()?;
            let var_1 = rest_0_0_0_rx.recv()?;
            let rs1_0_0_0_1 = {
                var_0.extend(var_1.into_iter());
                var_0
            };
            rs1_0_0_0_1_tx.send(rs1_0_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut var_0 = maze_0_0_2_rx.recv()?;
            let m2_0_0_0 = var_0.clone();
            m2_0_0_0_tx.send(m2_0_0_0)?;
            maze_0_0_1_0_tx.send(var_0)?
        }
    }));
    let (foo_sx, foo_rx) = std::sync::mpsc::channel();
    tasks.push(Box::new(move || -> _ {
        let mut rt = std::sync::Arc::new(
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(THREADCOUNT)
                .build()
                .unwrap(),
        );
        foo_sx.send(rt.clone()).unwrap();
        loop {
            let var_1 = b_0_0_rx.recv()?;
            let var_2 = d_0_rx.recv()?;
            let futures_0 = {
                let (tx, rx) = std::sync::mpsc::channel();
                let work = async move { tx.send(find_path(var_1, var_2)).unwrap() };
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
            let mut mro_0_0_1_0 = mro_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let b_0_0 = mro_0_0_1_0.clone();
                    b_0_0_tx.send(b_0_0)?;
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
            let mut var_0 = rs1_0_0_0_1_rx.recv()?;
            let rs2_0_0_0 = var_0.clone();
            rs2_0_0_0_tx.send(rs2_0_0_0)?;
            rs1_0_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let init_0 = maze_1_0_0_rx.recv()?;
        maze_0_0_2_tx.send(init_0)?;
        pairs_0_0_0_tx.send(pairs)?;
        its_0_0_0_tx.send(max_it)?;
        while not_done_0_0_0_rx.recv()? {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let loop_res_0 = maze_0_0_0_0_rx.recv()?;
            let loop_res_1 = rs2_0_0_0_rx.recv()?;
            let loop_res_2 = new_its_0_0_0_rx.recv()?;
            maze_0_0_2_tx.send(loop_res_0)?;
            pairs_0_0_0_tx.send(loop_res_1)?;
            its_0_0_0_tx.send(loop_res_2)?;
            ()
        }
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        rs2_0_0_0_rx.recv()?;
        new_its_0_0_0_rx.recv()?;
        let finalResult = maze_0_0_0_0_rx.recv()?;
        Ok(e_0_0_tx.send(finalResult)?)
    }));
    let (retry_sx, retry_rx) = std::sync::mpsc::channel();
    tasks.push(Box::new(move || -> _ {
        let s = retry_sx;
        loop {
            let mut var_0 = maze_0_0_1_0_rx.recv()?;
            let var_1 = rs_0_1_0_rx.recv()?;
            let rs1_0_1_0 = var_0.update(var_1, &s);
            rs1_0_1_0_tx.send(rs1_0_1_0)?;
            maze_0_0_0_0_tx.send(var_0)?
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
            let mut var_0 = pairs_0_0_0_rx.recv()?;
            let res = {
                let sp = if var_0.len() < FREQUENCY {
                    var_0.len()
                } else {
                    FREQUENCY
                };
                let chunk = var_0.split_off(sp);
                (var_0, chunk)
            };
            let pairs_0_n_0_0_0 = res.0;
            pairs_0_n_0_0_0_tx.send(pairs_0_n_0_0_0)?;
            let rest_0_0_0 = res.1;
            rest_0_0_0_tx.send(rest_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut data = pairs_0_n_0_0_0_rx.recv()?;
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
                    d_0_tx.send(d)?;
                    ()
                }
            } else {
                let mut size = 0;
                for d in data {
                    d_0_tx.send(d)?;
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
        let maze_1_0_0 = Maze::init(dimensions);
        maze_1_0_0_tx.send(maze_1_0_0)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut rs_0_0_1_0 = rs_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = path_0_0_0_rx.recv()?;
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
    let _ = foo_rx.recv().unwrap();
    match e_0_0_rx.recv() {
        Ok(res) => {
            let count = Iterator::sum(retry_rx.iter());
            (res, count)
        }
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}