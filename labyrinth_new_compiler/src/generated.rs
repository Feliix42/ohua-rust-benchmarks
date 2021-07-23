// manually adjusted: added `crate::`
use crate::benchs::*;

use std::*;

// manually adjusted: Why is that here??
//fn fill(maze: Maze, pairs: Vec<(Point, Point)>, its_left: u32) -> Maze {
    //let rs = Vec::new();
    //let mro = maze.clone();
    //for pair in pairs {
        //let path = find_path(mro, pair);
        //let r = maze.update(path);
        //rs.push(r);
    //}
    //let rs1 = filter_mapped(rs);
    //let new_its_left = decrement(its_left);
    //let not_done = calculate_done(rs1, new_its_left);
    //if not_done {
        //fill(maze, rs1, new_its_left)
    //} else {
        //maze
    //}
//}

// manually adjusted: added `pub`
pub fn run(salt: i32, pairs: Vec<(Point, Point)>, max_it: u32) -> Maze {
    let (c_0_0_tx, c_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (not_done_0_0_0_tx, not_done_0_0_0_rx) = std::sync::mpsc::channel();
    let (rs1_0_0_0_tx, rs1_0_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (maze_1_0_0_tx, maze_1_0_0_rx) = std::sync::mpsc::channel();
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
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
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
                        size_0_0_tx.send(size)?;
                        let ctrl = (true, size);
                        ctrl_2_3_tx.send(ctrl)?;
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
                        size_0_0_tx.send(size)?;
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
            let lit_unit_0_0_0 = ();
            while !renew {
                let sig = ctrl_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = lit_unit_0_0_0;
                    let rs_0_0_1 = Vec::new(var_0);
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
            let var_0 = futures_0_rx.recv()?;
            let path_0_0_0 = ohua::lang::collectFuture(var_0);
            path_0_0_0_tx.send(path_0_0_0)?
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
            let renew = false;
            let mro_0_0_0_0 = mro_0_0_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = find_path;
                    let var_1 = mro_0_0_0_0;
                    let var_2 = d_0_0_rx.recv()?;
                    let futures_0 = ohua::lang::spawn_futures(var_0, var_1, var_2);
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
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let init_0 = maze_1_0_0_rx.recv()?;
            let init_1 = pairs;
            maze_0_1_0_tx.send(init_0)?;
            pairs_0_1_0_tx.send(init_1)?;
            while not_done_0_0_0_rx.recv()? {
                maze_0_0_1_0_rx.recv()?;
                let ctrlSig = (true, 1);
                ctrl_0_0_0_tx.send(ctrlSig)?;
                let loop_res_0 = maze_0_0_1_0_rx.recv()?;
                let loop_res_1 = rs1_0_0_0_rx.recv()?;
                maze_0_1_0_tx.send(loop_res_0)?;
                pairs_0_1_0_tx.send(loop_res_1)?;
                ()
            }
            let ctrlSig = (false, 0);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let finaResult = maze_0_0_1_0_rx.recv()?;
            c_0_0_tx.send(finalResult)?
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
            ()
        }
    }));
    run(tasks);
    c_0_0_rx.recv()?
}
