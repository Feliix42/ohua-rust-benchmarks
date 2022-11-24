use crate::ohua::benchs::*;

use crate::types::Point;

use std::sync::Arc;

fn fill(mut maze: Maze, pairs: Vec<  OPoint,>, its: u32) -> Maze {
  let mut rs: Unmapped = Unmapped::default();
  let m2: Maze = maze.clone();
  let mro: Arc<  Maze,> = Arc::new(m2);
  for pair0 in pairs {
    let pair: Option<  (Point, Point),> = pair0;
    let mro_c: Arc<  Maze,> = mro.clone();
    let path: Option<  Path,> = find_path(mro_c, pair);
    let r: Option<  (Point, Point),> = maze.update(path);
    rs.push(r);
  }
  let (new_its, not_done, pending): (u32, bool, Vec<  OPoint,>) =
    rs.calculate_done(its);
  if not_done { fill(maze, pending, new_its) } else { maze }
}

pub fn run(
  dimensions: Point,
  pairs: Vec<  Option<  (Point, Point),>,>,
  max_it: u32,
) -> (Maze, usize) {
  #[derive(Debug)]
  enum RunError {
    SendFailed,
    RecvFailed,
  }
  impl<  T: Send,> From<  std::sync::mpsc::SendError<  T,>,> for RunError {
    fn from(_err: std::sync::mpsc::SendError<  T,>) -> Self {
      RunError::SendFailed
    }
  }
  impl From<  std::sync::mpsc::RecvError,> for RunError {
    fn from(_err: std::sync::mpsc::RecvError) -> Self {
      RunError::RecvFailed
    }
  }
  let (e_0_0_tx, e_0_0_rx) = std::sync::mpsc::channel::<  (Maze, usize),>();
  let (maze_0_1_0_tx, maze_0_1_0_rx) = std::sync::mpsc::channel::<  Maze,>();
  let (not_done_0_0_0_tx, not_done_0_0_0_rx) =
    std::sync::mpsc::channel::<  bool,>();
  let (new_its_0_0_0_tx, new_its_0_0_0_rx) =
    std::sync::mpsc::channel::<  u32,>();
  let (pending_0_0_0_0_tx, pending_0_0_0_0_rx) =
    std::sync::mpsc::channel::<  Vec<  _,>,>();
  let (maze_1_0_0_tx, maze_1_0_0_rx) = std::sync::mpsc::channel::<  Maze,>();
  let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) =
    std::sync::mpsc::channel::<  (bool, usize),>();
  let (maze_0_0_2_tx, maze_0_0_2_rx) = std::sync::mpsc::channel::<  Maze,>();
  let (m2_0_0_0_tx, m2_0_0_0_rx) = std::sync::mpsc::channel::<  Maze,>();
  let (pairs_0_0_0_tx, pairs_0_0_0_rx) =
    std::sync::mpsc::channel::<  Vec<  _,>,>();
  let (pairs_0_n_0_0_0_tx, pairs_0_n_0_0_0_rx) =
    std::sync::mpsc::channel::<  Vec<  _,>,>();
  let (maze_0_0_1_0_tx, maze_0_0_1_0_rx) =
    std::sync::mpsc::channel::<  Maze,>();
  let (ctrl_2_0_tx, ctrl_2_0_rx) =
    std::sync::mpsc::channel::<  (bool, usize),>();
  let (mro_0_0_1_tx, mro_0_0_1_rx) =
    std::sync::mpsc::channel::<  Arc<  Maze,>,>();
  let (ctrl_2_1_tx, ctrl_2_1_rx) =
    std::sync::mpsc::channel::<  (bool, usize),>();
  let (rs_0_0_1_tx, rs_0_0_1_rx) = std::sync::mpsc::channel::<  Unmapped,>();
  let (ctrl_2_2_tx, ctrl_2_2_rx) =
    std::sync::mpsc::channel::<  (bool, usize),>();
  let (d_1_tx, d_1_rx) =
    std::sync::mpsc::channel::<  Option<  (Point, Point),>,>();
  let (mro_c_0_0_0_tx, mro_c_0_0_0_rx) =
    std::sync::mpsc::channel::<  Arc<  Maze,>,>();
  let (futures_0_tx, futures_0_rx) =
    std::sync::mpsc::channel::<
      std::sync::mpsc::Receiver<  Option<  Path,>,>,
    >();
  let (path_0_0_0_tx, path_0_0_0_rx) =
    std::sync::mpsc::channel::<  Option<  Path,>,>();
  let (r_0_0_0_tx, r_0_0_0_rx) =
    std::sync::mpsc::channel::<  Option<  (Point, Point),>,>();
  let (its_0_0_0_tx, its_0_0_0_rx) = std::sync::mpsc::channel::<  u32,>();
  let (rs_0_1_1_tx, rs_0_1_1_rx) = std::sync::mpsc::channel::<  Unmapped,>();
  let (rest_0_0_0_tx, rest_0_0_0_rx) =
    std::sync::mpsc::channel::<  Vec<  _,>,>();
  let (pending_0_1_0_tx, pending_0_1_0_rx) =
    std::sync::mpsc::channel::<  Vec<  _,>,>();
  let (d_0_0_tx, d_0_0_rx) = std::sync::mpsc::channel::<  Maze,>();
  let mut tasks: Vec<  Box<  dyn FnOnce() -> Result<(), RunError> + Send,>,> =
    Vec::new();
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut renew = false;
        while !renew {
          let sig = ctrl_0_0_0_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let rs_0_0_1 = Unmapped::default();
            rs_0_0_1_tx.send(rs_0_0_1)?;
            ()
          };
          let renew_next_time = sig.0;
          renew = renew_next_time;
          ()
        }
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut renew = false;
        let mut mro_0_0_1_0 = mro_0_0_1_rx.recv()?;
        while !renew {
          let sig = ctrl_2_1_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let mro_c_0_0_0 = mro_0_0_1_0.clone();
            mro_c_0_0_0_tx.send(mro_c_0_0_0)?;
            ()
          };
          let renew_next_time = sig.0;
          renew = renew_next_time;
          ()
        };
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut data = pairs_0_n_0_0_0_rx.recv()?;
        let hasSize =
          {
            let tmp_has_size = data.iter().size_hint();
            tmp_has_size.1.is_some()
          };
        if hasSize {
          let size = data.len();
          let ctrl = (true, size);
          ctrl_2_0_tx.send(ctrl)?;
          let ctrl = (true, size);
          ctrl_2_1_tx.send(ctrl)?;
          let ctrl = (true, size);
          ctrl_2_2_tx.send(ctrl)?;
          for d in data { d_1_tx.send(d)?; () }
        } else {
          let mut size = 0;
          for d in data {
            d_1_tx.send(d)?;
            let ctrl = (false, 1);
            ctrl_2_0_tx.send(ctrl)?;
            let ctrl = (false, 1);
            ctrl_2_1_tx.send(ctrl)?;
            let ctrl = (false, 1);
            ctrl_2_2_tx.send(ctrl)?;
            size = size + 1;
            ()
          };
          let ctrl = (true, 0);
          ctrl_2_0_tx.send(ctrl)?;
          let ctrl = (true, 0);
          ctrl_2_1_tx.send(ctrl)?;
          let ctrl = (true, 0);
          ctrl_2_2_tx.send(ctrl)?;
          ()
        }
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let var_0 = d_0_0_rx.recv()?;
        let e_0_0 = (var_0, 42);
        e_0_0_tx.send(e_0_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = pending_0_1_0_rx.recv()?;
        let var_1 = rest_0_0_0_rx.recv()?;
        let pending_0_0_0_0 = { var_0.extend(var_1.into_iter()); var_0 };
        pending_0_0_0_0_tx.send(pending_0_0_0_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = rs_0_1_1_rx.recv()?;
        let var_1 = its_0_0_0_rx.recv()?;
        let res = var_0.calculate_done(var_1);
        let new_its_0_0_0 = res.0;
        new_its_0_0_0_tx.send(new_its_0_0_0)?;
        let not_done_0_0_0 = res.1;
        not_done_0_0_0_tx.send(not_done_0_0_0)?;
        let pending_0_1_0 = res.2;
        pending_0_1_0_tx.send(pending_0_1_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = futures_0_rx.recv()?;
        let path_0_0_0 = var_0.recv().unwrap();
        path_0_0_0_tx.send(path_0_0_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      let mut rt =
        std::sync::Arc::new(tokio::runtime::Builder::new()
          .threaded_scheduler()
          .core_threads(42)
          .build()
          .unwrap());
      loop {
        let var_1 = mro_c_0_0_0_rx.recv()?;
        let var_2 = d_1_rx.recv()?;
        let futures_0 =
          {
            let (tx, rx) = std::sync::mpsc::channel();
            let work = async move { tx.send(find_path(var_1, var_2)).unwrap() };
            rt.spawn(work);
            rx
          };
        futures_0_tx.send(futures_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = pairs_0_0_0_rx.recv()?;
        let res =
          {
            let sp = if var_0.len() < 13 { var_0.len() } else { 13 };
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
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let var_0 = m2_0_0_0_rx.recv()?;
        let mro_0_0_1 = Arc::new(var_0);
        mro_0_0_1_tx.send(mro_0_0_1)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = maze_0_0_2_rx.recv()?;
        let m2_0_0_0 = var_0.clone();
        m2_0_0_0_tx.send(m2_0_0_0)?;
        maze_0_0_1_0_tx.send(var_0)?
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      let ctrlSig = (true, 1);
      ctrl_0_0_0_tx.send(ctrlSig)?;
      let init_0 = maze_1_0_0_rx.recv()?;
      maze_0_0_2_tx.send(init_0)?;
      pairs_0_0_0_tx.send(pairs)?;
      its_0_0_0_tx.send(max_it)?;
      while not_done_0_0_0_rx.recv()? {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let loop_res_0 = maze_0_1_0_rx.recv()?;
        let loop_res_1 = pending_0_0_0_0_rx.recv()?;
        let loop_res_2 = new_its_0_0_0_rx.recv()?;
        maze_0_0_2_tx.send(loop_res_0)?;
        pairs_0_0_0_tx.send(loop_res_1)?;
        its_0_0_0_tx.send(loop_res_2)?;
        ()
      };
      let ctrlSig = (false, 0);
      ctrl_0_0_0_tx.send(ctrlSig)?;
      pending_0_0_0_0_rx.recv()?;
      new_its_0_0_0_rx.recv()?;
      let finalResult = maze_0_1_0_rx.recv()?;
      Ok(d_0_0_tx.send(finalResult)?)
    }));
  tasks
    .push(Box::new(move || -> _ {
      let maze_1_0_0 = Maze::init(dimensions);
      maze_1_0_0_tx.send(maze_1_0_0)?;
      Ok(())
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut renew = false;
        let mut rs_0_0_1_0 = rs_0_0_1_rx.recv()?;
        while !renew {
          let sig = ctrl_2_2_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let var_1 = r_0_0_0_rx.recv()?;
            rs_0_0_1_0.push(var_1);
            ()
          };
          let renew_next_time = sig.0;
          renew = renew_next_time;
          ()
        };
        rs_0_1_1_tx.send(rs_0_0_1_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut renew = false;
        let mut maze_0_0_1_0_0 = maze_0_0_1_0_rx.recv()?;
        while !renew {
          let sig = ctrl_2_0_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let var_1 = path_0_0_0_rx.recv()?;
            let r_0_0_0 = maze_0_0_1_0_0.update(var_1);
            r_0_0_0_tx.send(r_0_0_0)?;
            ()
          };
          let renew_next_time = sig.0;
          renew = renew_next_time;
          ()
        };
        maze_0_1_0_tx.send(maze_0_0_1_0_0)?;
        ()
      }
    }));
  let handles: Vec<  std::thread::JoinHandle<  _,>,> =
    tasks
      .into_iter()
      .map(|t| { std::thread::spawn(move || { let _ = t(); }) })
      .collect();
  for h in handles {
    if let Err(_) = h.join() {
      eprintln!("[Error] A worker thread of an Ohua algorithm has panicked!");
    }
  }
  match e_0_0_rx.recv() {
    Ok(res) => res,
    Err(e) => panic!("[Ohua Runtime Internal Exception] {}", e),
  }
}
