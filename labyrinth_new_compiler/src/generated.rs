// manually adjusted: added `crate::`
use crate::benchs::*;

use std::*;
// FIXME(easy): manually inserted the following. Maybe fully quantify the the paths in the code or
// add the import
// FIXED
use std::sync::Arc;
use tokio::runtime::Builder;

// FIXME(medium?): Threadcount variable in runtime creation is generated but not defined
//  -> possible solution is the use of 2n threads (for n.. number of physical threads)
const THREADCOUNT: usize = 8;

// FIXME(idk): Commented this out as it only produces compiler errors and nothing else
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

//FIXME(easy-medium): manually adjusted: added `pub`
//  -> either generate this automatically add visibility to language
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
    // FIXME(needs triage!): Channel is created twice!!
    //let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (not_done_0_0_0_tx, not_done_0_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (maze_1_0_0_tx, maze_1_0_0_rx) = std::sync::mpsc::channel();
    let (its_left_0_1_0_tx, its_left_0_1_0_rx) = std::sync::mpsc::channel();
    let (maze_0_1_0_tx, maze_0_1_0_rx) = std::sync::mpsc::channel();
    let (pairs_0_1_0_tx, pairs_0_1_0_rx) = std::sync::mpsc::channel();
    // FIXME(medium): Dispatch Insertion needs to be applied to `recurFun` as well
    let (ctrl_0_0_0_0_tx, ctrl_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_1_tx, ctrl_0_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_2_tx, ctrl_0_0_0_2_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_3_tx, ctrl_0_0_0_3_rx) = std::sync::mpsc::channel();
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
    // FIXME(complicated): This might be one of the hardest issues: both `recurFun` and
    // `calculate_done` use the same receivers (new_its_left_0_0_0_rx and rs1_0_0_0_rx) because the
    // data is both sent to the recursion function and the function that determines if another
    // recursion is to occur
    //   => My workaround: Added two pairs of senders/receivers to temporarily fix the issue in
    //   rustc
    let (new_its_left_0_0_0_tx, new_its_left_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_its_left_0_0_0_tx1, new_its_left_0_0_0_rx1) = std::sync::mpsc::channel();
    let (rs1_0_0_0_tx, rs1_0_0_0_rx) = std::sync::mpsc::channel();
    let (rs1_0_0_0_tx1, rs1_0_0_0_rx1) = std::sync::mpsc::channel();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        //loop {
            let ctrlSig = (true, 1);
            ctrl_0_0_0_0_tx.send(ctrlSig)?;
            ctrl_0_0_0_1_tx.send(ctrlSig)?;
            ctrl_0_0_0_2_tx.send(ctrlSig)?;
            ctrl_0_0_0_3_tx.send(ctrlSig)?;
            let init_0 = maze_1_0_0_rx.recv()?;
            // FIXME(medium/complicated): value is moved here originally (let init_1 = pairs;)
            //   -> do we need the surrounding `loop` here?
            let init_1 = pairs;
            let init_2 = max_it;
            maze_0_1_0_tx.send(init_0)?;
            pairs_0_1_0_tx.send(init_1)?;
            its_left_0_1_0_tx.send(init_2)?;
            while not_done_0_0_0_rx.recv()? {
                maze_0_0_1_0_rx.recv()?;
                let ctrlSig = (true, 1);
                ctrl_0_0_0_0_tx.send(ctrlSig)?;
                ctrl_0_0_0_1_tx.send(ctrlSig)?;
                ctrl_0_0_0_2_tx.send(ctrlSig)?;
                ctrl_0_0_0_3_tx.send(ctrlSig)?;
                let loop_res_0 = maze_0_0_1_0_rx.recv()?;
                let loop_res_1 = rs1_0_0_0_rx.recv()?;
                let loop_res_2 = new_its_left_0_0_0_rx.recv()?;
                maze_0_1_0_tx.send(loop_res_0)?;
                pairs_0_1_0_tx.send(loop_res_1)?;
                its_left_0_1_0_tx.send(loop_res_2)?;
                ()
            }
            let ctrlSig = (false, 0);
            ctrl_0_0_0_0_tx.send(ctrlSig)?;
            ctrl_0_0_0_1_tx.send(ctrlSig)?;
            ctrl_0_0_0_2_tx.send(ctrlSig)?;
            ctrl_0_0_0_3_tx.send(ctrlSig)?;
            let finalResult = maze_0_0_1_0_rx.recv()?;
            // FIXME(easy): Inserted semicolon and `Ok(())`
            c_0_0_tx.send(finalResult)?;
            Ok(())
        //}
    }));
    tasks.push(Box::new(move || -> _ {
        let rt = Arc::new(
            Builder::new()
                .threaded_scheduler()
                .core_threads(THREADCOUNT)
                .thread_name("ohua-tokio-worker")
                .build()
                .unwrap(),
        );
        loop {
            // FIXME(easy): insert `mut`
            // see sertel/ohuac-integrations#4
            let mut renew = false;
            // FIXME(complicated): I don't even know where to start here. This whole block is a
            // mess - at least from rustc's point of view. The problem has several layers:
            // 1) The function name (var_0) should not be hidden behind this indirection, because
            //    that makes the borrow checker angry (the variable var_0 would outlive the scope
            //    it was created in -- i suppose you could solve that with lifetime annotations but
            //    that's not really a road we want to go down)
            // 2) The mro variable gives me a headache. Used as is, it creates a use after move
            //    error, since it's used at least `count` times in a loop. The obvious quick hack
            //    would be cloning the data structure, in the algorithm described like this:
            //    for _ in 0..count {
            //        foo(mro.clone());
            //    }
            //    But I'm not sure if the compiler supports this and if that would be the actual solution.
            let mro_0_0_0_0: Maze = mro_0_0_0_rx.recv()?;
            while !renew {
                // FIXME(easy): Need to add a type annotation to this!
                let sig: (bool, usize) = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    //let var_0 = find_path;
                    let var_1 = mro_0_0_0_0.clone();
                    let var_2 = d_0_0_rx.recv()?;
                    let futures_0 = {
                        //let comp = (|| -> _ { var_0(var_1, var_2) });
                        let comp = (|| -> _ { find_path(var_1, var_2) });
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
            // FIXME(easy): Insert `mut`
            // see sertel/ohuac-integrations#4
            let mut renew = false;
            let its_left_0_0_0_0 = its_left_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_0_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_0 = its_left_0_0_0_0;
                    let new_its_left_0_0_0 = decrement(var_0);
                    new_its_left_0_0_0_tx.send(new_its_left_0_0_0.clone())?;
                    new_its_left_0_0_0_tx1.send(new_its_left_0_0_0)?
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
        // FIXME(easy): Appended semicolon and `Ok(())`
        maze_1_0_0_tx.send(maze_1_0_0)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = rs_0_1_0_rx.recv()?;
            let rs1_0_0_0 = filter_mapped(var_0);
            rs1_0_0_0_tx.send(rs1_0_0_0.clone())?;
            rs1_0_0_0_tx1.send(rs1_0_0_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            // FIXME(easy): Insert `mut`
            // see sertel/ohuac-integrations#4
            let mut renew = false;
            // FIXME(needs triage): The `pairs_0_0_0_0` variable is assigned to `data` and then
            // used in an owned fashion. This creates serious issues, as this results in 'use after
            // move' issues. My fix here is inserting a clone but the validity of this is yet to
            // be determined.
            // see sertel/ohua-core#20 - FIXED
            let pairs_0_0_0_0 = pairs_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_0_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    // see sertel/ohua-core#20 - FIXED
                    // TODO -> here
                    let data = pairs_0_0_0_0.clone();
                    let hasSize = {
                        let tmp_has_size = data.iter().size_hint();
                        tmp_has_size.1.is_some()
                    };
                    if hasSize {
                        let size = data.len();
                        // FIXME(medium-complicated): Size is dropped when fusing runSTClang-smap with the stateful
                        // see sertel/ohua-backend#14
                        // update function
                        //size_0_2_tx.send(size)?;
                        //size_0_1_tx.send(size)?;
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
                        // FIXME(easy): add `mut`
                        // see sertel/ohuac-integrations#4
                        let mut size = 0;
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
                        //size_0_2_tx.send(size)?;
                        //size_0_1_tx.send(size)?;
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
            // FIXME(easy): `future` argument will need a Type annotation
            // FIXED
            let path_0_0_0 = (|future: std::sync::mpsc::Receiver<_>| future.recv().unwrap())(var_0);
            path_0_0_0_tx.send(path_0_0_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            // FIXME(easy): insert `mut`
            // sertel/ohuac-integrations#4
            let mut renew = false;
            let lit_unit_0_0_0 = ();
            while !renew {
                let sig = ctrl_0_0_0_2_rx.recv()?;
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
            let var_0 = rs1_0_0_0_rx1.recv()?;
            let var_1 = new_its_left_0_0_0_rx1.recv()?;
            let not_done_0_0_0 = calculate_done(var_0, var_1);
            not_done_0_0_0_tx.send(not_done_0_0_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            // FIXME(medium/complicated): Alter codegen so that `maze_0_0_0_2` is used in the body
            // because otherwise we get use after move errors -> This is directly related to the
            // fixme in the task below!
            let maze_0_0_0_2 = maze_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_0_0_0_3_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    // FIXME: see above note
                    //let var_0 = maze_0_0_0_2;
                    let var_1 = ();
                    let mro_0_0_0 = maze_0_0_0_2.clone();
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
            let mut renew = false;
            // FIXME(medium/complicated): Alter codegen so that `rs_0_0_1_0 is used in the
            // body because tossing the state around like this (assigning it to another var)
            // creates a _lot_ of problems!
            // FIXME(medium): insert mut in state receive
            let mut rs_0_0_1_0 = rs_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_2_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    // FIXME: see above note
                    //let var_0 = rs_0_0_1_0;
                    let var_1 = r_0_0_0_rx.recv()?;
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
            // FIXME(medium/complicated): Alter codegen so that `maze_0_0_0_1_0_0 is used in the
            // body because tossing the state around like this (assigning it to another var)
            // creates a _lot_ of problems!
            // FIXME(medium): insert mut in state receive
            let mut maze_0_0_0_1_0_0 = maze_0_0_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    // FIXME: see above note
                    //let var_0 = maze_0_0_0_1_0_0;
                    let var_1 = path_0_0_0_rx.recv()?;
                    let r_0_0_0 = maze_0_0_0_1_0_0.update(var_1);
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
    // FIXME(easy): Type annotations for the JoinHandle Vec
    // FIXED
    let mut handles: Vec<std::thread::JoinHandle<_>> = tasks
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

    // FIXME(easy): Need to replace receiving the result with `Try` with something that catches the `Err`
    // variant solely yields the result
    // FIXED
    match c_0_0_rx.recv() {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    }
    //c_0_0_rx.recv()?
}
