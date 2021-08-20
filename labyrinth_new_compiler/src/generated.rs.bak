use crate::benchs::*;

// FIXME(feliix42): mutable Maze here
pub fn fill(mut maze: Maze, pairs: Vec<(Point, Point)>, its_left: u32) -> Maze {
    // FIXME(feliix42): mutable `rs` here
    let mut rs = Vec::new();
    let mro = maze.clone();
    for pair in pairs {
        let path = find_path(mro.clone(), pair);
        let r = maze.update(path);
        rs.push(r);
    }
    let rs1 = filter_mapped(rs);
    let rs2 = rs1.clone();
    let new_its_left = decrement(its_left);
    let new_its_left1 = new_its_left.clone();
    let not_done = calculate_done(rs1, new_its_left);
    if not_done {
        fill(maze, rs2, new_its_left1)
    } else {
        maze
    }
}

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
    let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel();
    let (maze_0_1_0_tx, maze_0_1_0_rx) = std::sync::mpsc::channel();
    let (not_done_0_0_0_tx, not_done_0_0_0_rx) = std::sync::mpsc::channel();
    let (new_its_left1_0_0_0_tx, new_its_left1_0_0_0_rx) = std::sync::mpsc::channel();
    let (rs2_0_0_0_tx, rs2_0_0_0_rx) = std::sync::mpsc::channel();
    let (maze_1_0_0_tx, maze_1_0_0_rx) = std::sync::mpsc::channel();
    let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    // FIXME: This channel needs a type -> See where it could come from!
    let (maze_0_0_2_tx, maze_0_0_2_rx) = std::sync::mpsc::channel::<Maze>();
    // FIXME: Needs a type!
    let (pairs_0_0_0_tx, pairs_0_0_0_rx) = std::sync::mpsc::channel::<Vec<(Point, Point)>>();
    let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) = std::sync::mpsc::channel();
    let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (mro_0_0_1_tx, mro_0_0_1_rx) = std::sync::mpsc::channel();
    let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<(_, _)>();
    // FIXME: Needs a type
    let (rs_0_0_1_tx, rs_0_0_1_rx) = std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    // FIXME(feliix42): Need type anno here!
    let (ctrl_2_2_tx, ctrl_2_2_rx) = std::sync::mpsc::channel::<(_, _)>();
    let (d_1_0_tx, d_1_0_rx) = std::sync::mpsc::channel::<(Point, Point)>();
    let (a_0_0_tx, a_0_0_rx) = std::sync::mpsc::channel::<Maze>();
    let (path_0_0_0_tx, path_0_0_0_rx) = std::sync::mpsc::channel::<Vec<Point>>();
    // FIXME(feliix42): This had a generic type parameter `T` instead of Maze!
    let (r_0_0_0_tx, r_0_0_0_rx) = std::sync::mpsc::channel::<Option<(Point, Point)>>();
    let (maze_0_0_0_0_tx, maze_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (size_0_2_tx, size_0_2_rx) = std::sync::mpsc::channel();
    let (rs_0_0_0_0_tx, rs_0_0_0_0_rx) = std::sync::mpsc::channel();
    let (size_0_3_tx, size_0_3_rx) = std::sync::mpsc::channel();
    let (rs_0_1_0_tx, rs_0_1_0_rx) = std::sync::mpsc::channel::<Vec<Option<(Point, Point)>>>();
    let (rs1_0_0_1_tx, rs1_0_0_1_rx) = std::sync::mpsc::channel();
    let (its_left_0_0_0_tx, its_left_0_0_0_rx) = std::sync::mpsc::channel::<u32>();
    // FIXME: Needs type annotations
    let (new_its_left_0_0_1_tx, new_its_left_0_0_1_rx) = std::sync::mpsc::channel::<u32>();
    let (new_its_left_0_0_0_0_tx, new_its_left_0_0_0_0_rx) = std::sync::mpsc::channel::<u32>();
    let (rs1_0_0_0_0_tx, rs1_0_0_0_0_rx) = std::sync::mpsc::channel::<Vec<(Point, Point)>>();
    let mut tasks: Vec<Box<dyn FnOnce() -> Result<(), RunError> + Send>> = Vec::new();
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = rs1_0_0_0_0_rx.recv()?;
            let var_1 = new_its_left_0_0_0_0_rx.recv()?;
            let not_done_0_0_0 = calculate_done(var_0, var_1);
            not_done_0_0_0_tx.send(not_done_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = maze_0_0_2_rx.recv()?;
            let mro_0_0_1 = var_0.clone();
            mro_0_0_1_tx.send(mro_0_0_1)?;
            maze_0_0_1_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mut rs_0_0_1_0 = rs_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_2_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = r_0_0_0_rx.recv()?;
                    rs_0_0_1_0.push(var_1);
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            rs_0_0_0_0_tx.send(rs_0_0_1_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = new_its_left_0_0_1_rx.recv()?;
            let new_its_left1_0_0_0 = var_0.clone();
            new_its_left1_0_0_0_tx.send(new_its_left1_0_0_0)?;
            new_its_left_0_0_0_0_tx.send(var_0)?
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let data = pairs_0_0_0_rx.recv()?;
            let hasSize = {
                let tmp_has_size = data.iter().size_hint();
                tmp_has_size.1.is_some()
            };
            if hasSize {
                let size = data.len();
                size_0_3_tx.send(size)?;
                size_0_2_tx.send(size)?;
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
                // FIXME(feliix42): Needs mut!
                let mut size = 0;
                for d in data {
                    d_1_0_tx.send(d)?;
                    let ctrl = (false, 1);
                    ctrl_2_2_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_1_tx.send(ctrl)?;
                    let ctrl = (false, 1);
                    ctrl_2_0_tx.send(ctrl)?;
                    size = size + 1;
                    ()
                }
                size_0_3_tx.send(size)?;
                size_0_2_tx.send(size)?;
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
            let var_0 = a_0_0_rx.recv()?;
            let var_1 = d_1_0_rx.recv()?;
            let path_0_0_0 = find_path(var_0, var_1);
            path_0_0_0_tx.send(path_0_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = its_left_0_0_0_rx.recv()?;
            let new_its_left_0_0_1 = decrement(var_0);
            new_its_left_0_0_1_tx.send(new_its_left_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            let mro_0_0_1_0 = mro_0_0_1_rx.recv()?;
            while !renew {
                let sig = ctrl_2_1_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let a_0_0 = mro_0_0_1_0.clone();
                    a_0_0_tx.send(a_0_0)?;
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
            let var_0 = rs_0_1_0_rx.recv()?;
            let rs1_0_0_1 = filter_mapped(var_0);
            rs1_0_0_1_tx.send(rs1_0_0_1)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let mut renew = false;
            // FIXME: Need mutable here!
            let mut maze_0_0_1_0_0 = maze_0_0_1_0_rx.recv()?;
            while !renew {
                let sig = ctrl_2_0_rx.recv()?;
                let count = sig.1;
                for _ in 0..count {
                    let var_1 = path_0_0_0_rx.recv()?;
                    let r_0_0_0 = maze_0_0_1_0_0.update(var_1);
                    r_0_0_0_tx.send(r_0_0_0)?;
                    ()
                }
                let renew_next_time = sig.0;
                renew = renew_next_time;
                ()
            }
            maze_0_0_0_0_tx.send(maze_0_0_1_0_0)?;
            ()
        }
    }));
    tasks.push(Box::new(move || -> _ {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let init_0 = maze_1_0_0_rx.recv()?;
        maze_0_0_2_tx.send(init_0)?;
        pairs_0_0_0_tx.send(pairs)?;
        its_left_0_0_0_tx.send(max_it)?;
        while not_done_0_0_0_rx.recv()? {
            maze_0_1_0_rx.recv()?;
            let ctrlSig = (true, 1);
            ctrl_0_0_0_tx.send(ctrlSig)?;
            let loop_res_0 = maze_0_1_0_rx.recv()?;
            let loop_res_1 = rs2_0_0_0_rx.recv()?;
            let loop_res_2 = new_its_left1_0_0_0_rx.recv()?;
            maze_0_0_2_tx.send(loop_res_0)?;
            pairs_0_0_0_tx.send(loop_res_1)?;
            its_left_0_0_0_tx.send(loop_res_2)?;
            ()
        }
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let finalResult = maze_0_1_0_rx.recv()?;
        Ok(d_0_0_tx.send(finalResult)?)
    }));
    tasks.push(Box::new(move || -> _ {
        let maze_1_0_0 = Maze::init(salt);
        maze_1_0_0_tx.send(maze_1_0_0)?;
        Ok(())
    }));
    tasks.push(Box::new(move || -> _ {
        let num = size_0_3_rx.recv()?;
        let toDrop = num - 1;
        for _ in 0..toDrop {
            rs_0_0_0_0_rx.recv()?;
            ()
        }
        let s = rs_0_0_0_0_rx.recv()?;
        Ok(rs_0_1_0_tx.send(s)?)
    }));
    tasks.push(Box::new(move || -> _ {
        let num = size_0_2_rx.recv()?;
        let toDrop = num - 1;
        for _ in 0..toDrop {
            maze_0_0_0_0_rx.recv()?;
            ()
        }
        let s = maze_0_0_0_0_rx.recv()?;
        Ok(maze_0_1_0_tx.send(s)?)
    }));
    tasks.push(Box::new(move || -> _ {
        loop {
            let var_0 = rs1_0_0_1_rx.recv()?;
            let rs2_0_0_0 = var_0.clone();
            rs2_0_0_0_tx.send(rs2_0_0_0)?;
            rs1_0_0_0_0_tx.send(var_0)?
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
    match d_0_0_rx.recv() {
        Ok(res) => res,
        Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
    }
}
