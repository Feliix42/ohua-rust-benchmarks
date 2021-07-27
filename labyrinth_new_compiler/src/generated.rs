// manually adjusted: added `crate::`
use crate::benchs::*;

use std::*;
// manually inserted:
use std::sync::Arc;
use tokio::runtime::Builder;

fn fill(maze: Maze, pairs: Vec<(Point, Point)>, its_left: u32) -> Maze {
    let rs = Vec::new();
    let mro = maze.clone();
    for pair in pairs {
        let path = find_path(mro, pair);
        let r = maze.update(path);
        rs.push(r);
    }
    let rs1 = filter_mapped(rs);
    let new_its_left = decrement(its_left);
    let not_done = calculate_done(rs1, new_its_left);
    if not_done {
        fill(maze, rs1, new_its_left)
    } else {
        maze
    }
}

// manually adjusted: added `pub`
pub fn run(salt: i32, pairs: Vec<(Point, Point)>, max_it: u32) -> Maze {
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
    let (c_0_0_tx, c_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (not_done_0_0_0_tx, not_done_0_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (maze_1_0_0_tx, maze_1_0_0_rx) = std::sync::mpsc::channel();
    let (its_left_0_1_0_tx, its_left_0_1_0_rx) = std::sync::mpsc::channel();
    let (maze_0_1_0_tx, maze_0_1_0_rx) = std::sync::mpsc::channel();
    let (pairs_0_1_0_tx, pairs_0_1_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_0_1_0_tx, maze_0_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel();
    let (mro_0_0_0_tx, mro_0_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel();
    let (rs_0_0_1_tx, rs_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_2_2_tx, ctrl_2_2_rx) = std::sync::mpsc::channel();
    let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel();
    let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel();
    let (path_0_0_0_tx, path_0_0_0_rx) = std::sync::mpsc::channel();
    let (r_0_0_0_tx, r_0_0_0_rx) = std::sync::mpsc::channel();
    let (rs_0_1_0_tx, rs_0_1_0_rx) = std::sync::mpsc::channel();
    let (new_its_left_0_0_0_tx, new_its_left_0_0_0_rx) = std::sync::mpsc::channel();
    let (rs1_0_0_0_tx, rs1_0_0_0_rx) = std::sync::mpsc::channel();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        loop {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let init_0 = maze_1_0_0_rx.recv()?;
            let init_1 = pairs;
            let init_2 = max_it;
            maze_0_1_0_tx.send(init_0)?;
            pairs_0_1_0_tx.send(init_1)?;
            its_left_0_1_0_tx.send(init_2)?;
            while not_done_0_0_0_rx.recv()? {
                maze_0_0_1_0_rx.recv()?;
                let ctrlSig = (true, 1);
                ctrl_0_0_0_tx.send(ctrlSig)?;
                let loop_res_0 = maze_0_0_1_0_rx.recv()?;
                let loop_res_1 = rs1_0_0_0_rx.recv()?;
                let loop_res_2 = new_its_left_0_0_0_rx.recv()?;
                maze_0_1_0_tx.send(loop_res_0)?;
                pairs_0_1_0_tx.send(loop_res_1)?;
                its_left_0_1_0_tx.send(loop_res_2)?;
                ()
            }
            let ctrlSig = (false, 0);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let finalResult = maze_0_0_1_0_rx.recv()?;
            c_0_0_tx.send(finalResult)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let rt = Arc::new(
            Builder::new()
                .threaded_scheduler()
                .core_threads(threadcount)
                .thread_name("ohua-tokio-worker")
                .build()
                .unwrap(),
        );
        loop {
            let renew = false;
            let mro_0_0_0_0 = mro_0_0_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = find_path;
                    let var_1 = mro_0_0_0_0;
                    let var_2 = d_0_0_rx.recv()?;
                    let futures_0 = {
                        let comp = (|| -> _ { var_0(var_1, var_2) });
                        let (tx, rx) = std::sync::mpsc::channel();
                        let work = async move { tx.send(comp()).unwrap() };
                        rt.spawn(work);
                        rx
                    };
                    futures_0_tx.send(futures_0)?
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
            let renew = false;
            let its_left_0_0_0_0 = its_left_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = its_left_0_0_0_0;
                    let new_its_left_0_0_0 = decrement(var_0);
                    new_its_left_0_0_0_tx.send(new_its_left_0_0_0)?
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let var_0 = salt;
        let maze_1_0_0 = Maze::init(var_0);
        maze_1_0_0_tx.send(maze_1_0_0)?
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = rs_0_1_0_rx.recv()?;
            let rs1_0_0_0 = filter_mapped(var_0);
            rs1_0_0_0_tx.send(rs1_0_0_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let renew = false;
            let pairs_0_0_0_0 = pairs_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let data = pairs_0_0_0_0;
                    let hasSize = {
                        let tmp_has_size = data.iter().size_hint();
                        tmp_has_size.1.is_some()
                    };
                    if hasSize {
                        let size = data.len();
                        size_0_2_tx.send(size)?;
                        size_0_1_tx.send(size)?;
                        let ctrl = (true, size);
                        ctrl_2_2_tx.send(ctrl)?;
                        let ctrl = (true, size);
                        ctrl_2_1_tx.send(ctrl)?;
                        let ctrl = (true, size);
                        ctrl_2_0_tx.send(ctrl)?;
                        for d in data {
                            d_0_0_tx.send(d)?;
                            ()
                        }
                    } else {
                        let size = 0;
                        for d in data {
                            d_0_0_tx.send(d)?;
                            let ctrl = (false, 1);
                            ctrl_2_2_tx.send(ctrl)?;
                            let ctrl = (false, 1);
                            ctrl_2_1_tx.send(ctrl)?;
                            let ctrl = (false, 1);
                            ctrl_2_0_tx.send(ctrl)?;
                            size = size + 1;
                            ()
                        }
                        size_0_2_tx.send(size)?;
                        size_0_1_tx.send(size)?;
                        let ctrl = (true, 0);
                        ctrl_2_2_tx.send(ctrl)?;
                        let ctrl = (true, 0);
                        ctrl_2_1_tx.send(ctrl)?;
                        let ctrl = (true, 0);
                        ctrl_2_0_tx.send(ctrl)?;
                        ()
                    }
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
            let var_0 = futures_0_rx.recv()?;
            let path_0_0_0 = (|future| future.recv().unwrap())(var_0);
            path_0_0_0_tx.send(path_0_0_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let renew = false;
            let lit_unit_0_0_0 = ();
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = lit_unit_0_0_0;
                    let rs_0_0_1 = Vec::new();
                    rs_0_0_1_tx.send(rs_0_0_1)?
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = rs1_0_0_0_rx.recv()?;
            let var_1 = new_its_left_0_0_0_rx.recv()?;
            let not_done_0_0_0 = calculate_done(var_0, var_1);
            not_done_0_0_0_tx.send(not_done_0_0_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let renew = false;
            let maze_0_0_0_2 = maze_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = maze_0_0_0_2;
                    let var_1 = ();
                    let mro_0_0_0 = var_0.clone();
                    mro_0_0_0_tx.send(mro_0_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            maze_0_0_0_1_0_tx.send(maze_0_0_0_2)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let renew = false;
            let rs_0_0_1_0 = rs_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_2_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = rs_0_0_1_0;
                    let var_1 = r_0_0_0_rx.recv()?;
                    var_0.push(var_1);
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
            let renew = false;
            let maze_0_0_0_1_0_0 = maze_0_0_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = maze_0_0_0_1_0_0;
                    let var_1 = path_0_0_0_rx.recv()?;
                    let r_0_0_0 = var_0.update(var_1);
                    r_0_0_0_tx.send(r_0_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            maze_0_0_1_0_tx.send(maze_0_0_0_1_0_0)?;
            ()
        }
    }));
    let mut handles = tasks
        .into_iter()
        .map(|t| {
            thread::spawn(move || {
                let _ = t();
            })
        })
        .collect();
    for h in handles {
        if let Err(_) = h.join() {
            eprintln!("[Error] A worker thread of an Ohua algorithm has panicked!");
        }
    }
    c_0_0_rx.recv()?
}
