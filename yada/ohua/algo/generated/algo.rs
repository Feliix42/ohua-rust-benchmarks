use main::compute_cavity;

use element::Triangle;

use mesh::*;

use std::*;

pub fn refine(mut mesh: Mesh, bad: Vec<  Triangle,>) -> Mesh {
  let mut computed = Vec::new2();
  let m = Arc::new1(mesh.clone());
  for item in bad {
    let cav = compute_cavity(m.clone(), item);
    computed.push(cav);
  }
  let remaining_work = mesh.apply_updates(computed);
  let is_not_empty = mesh.has_more_work();
  if is_not_empty { refine(mesh, remaining_work) } else { mesh }
}

pub fn run_refine(mesh: Mesh) -> Mesh {
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
  let (e_0_0_tx, e_0_0_rx) = std::sync::mpsc::channel();
  let (m_1_0_1_tx, m_1_0_1_rx) = std::sync::mpsc::channel::<  Mesh,>();
  let (mesh_0_0_0_0_tx, mesh_0_0_0_0_rx) = std::sync::mpsc::channel();
  let (is_not_empty_0_0_0_tx, is_not_empty_0_0_0_rx) =
    std::sync::mpsc::channel();
  let (remaining_work_0_0_0_0_tx, remaining_work_0_0_0_0_rx) =
    std::sync::mpsc::channel();
  let (bad_1_0_0_tx, bad_1_0_0_rx) = std::sync::mpsc::channel();
  let (m_1_0_0_0_tx, m_1_0_0_0_rx) = std::sync::mpsc::channel();
  let (ctrl_0_0_0_tx, ctrl_0_0_0_rx) = std::sync::mpsc::channel();
  let (mesh_0_0_3_tx, mesh_0_0_3_rx) =
    std::sync::mpsc::channel::<  Arc<  T,>,>();
  let (a_0_0_tx, a_0_0_rx) = std::sync::mpsc::channel::<  T,>();
  let (bad_0_0_0_tx, bad_0_0_0_rx) = std::sync::mpsc::channel();
  let (bad_0_n_0_0_0_tx, bad_0_n_0_0_0_rx) =
    std::sync::mpsc::channel::<  Triangle,>();
  let (computed_0_0_1_tx, computed_0_0_1_rx) =
    std::sync::mpsc::channel::<  Vec<  T,>,>();
  let (ctrl_2_0_tx, ctrl_2_0_rx) = std::sync::mpsc::channel::<  (_, _),>();
  let (m_0_0_1_tx, m_0_0_1_rx) = std::sync::mpsc::channel::<  Arc<  T,>,>();
  let (ctrl_2_1_tx, ctrl_2_1_rx) = std::sync::mpsc::channel::<  (_, _),>();
  let (d_0_tx, d_0_rx) = std::sync::mpsc::channel::<  Triangle,>();
  let (b_0_0_tx, b_0_0_rx) = std::sync::mpsc::channel::<  Arc<  Mesh,>,>();
  let (futures_0_tx, futures_0_rx) = std::sync::mpsc::channel();
  let (cav_0_0_0_tx, cav_0_0_0_rx) = std::sync::mpsc::channel::<  Duration,>();
  let (computed_0_1_0_tx, computed_0_1_0_rx) =
    std::sync::mpsc::channel::<  Vec<  Option<  Cavity,>,>,>();
  let (mesh_0_0_2_0_tx, mesh_0_0_2_0_rx) =
    std::sync::mpsc::channel::<  Mesh,>();
  let (rest_0_0_0_tx, rest_0_0_0_rx) = std::sync::mpsc::channel();
  let (remaining_work_0_1_0_tx, remaining_work_0_1_0_rx) =
    std::sync::mpsc::channel();
  let (mesh_0_0_1_0_tx, mesh_0_0_1_0_rx) =
    std::sync::mpsc::channel::<  Mesh,>();
  let mut tasks: Vec<  Box<  dyn FnOnce() -> Result<(), RunError> + Send,>,> =
    Vec::new();
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut data = bad_0_n_0_0_0_rx.recv()?;
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
          for d in data { d_0_tx.send(d)?; () }
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
          };
          let ctrl = (true, 0);
          ctrl_2_0_tx.send(ctrl)?;
          let ctrl = (true, 0);
          ctrl_2_1_tx.send(ctrl)?;
          ()
        }
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let ctrlSig = (true, 1);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        let init_0 = m_1_0_0_0_rx.recv()?;
        let init_1 = bad_1_0_0_rx.recv()?;
        mesh_0_0_3_tx.send(init_0)?;
        bad_0_0_0_tx.send(init_1)?;
        while is_not_empty_0_0_0_rx.recv()? {
          let ctrlSig = (true, 1);
          ctrl_0_0_0_tx.send(ctrlSig)?;
          let loop_res_0 = mesh_0_0_0_0_rx.recv()?;
          let loop_res_1 = remaining_work_0_0_0_0_rx.recv()?;
          mesh_0_0_3_tx.send(loop_res_0)?;
          bad_0_0_0_tx.send(loop_res_1)?;
          ()
        };
        let ctrlSig = (false, 0);
        ctrl_0_0_0_tx.send(ctrlSig)?;
        remaining_work_0_0_0_0_rx.recv()?;
        let finalResult = mesh_0_0_0_0_rx.recv()?;
        e_0_0_tx.send(finalResult)?
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      let m_1_0_1 = id(mesh);
      m_1_0_1_tx.send(m_1_0_1)?;
      Ok(())
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let var_0 = a_0_0_rx.recv()?;
        let m_0_0_1 = Arc::new1(var_0);
        m_0_0_1_tx.send(m_0_0_1)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut renew = false;
        let mut m_0_0_1_0 = m_0_0_1_rx.recv()?;
        while !renew {
          let sig = ctrl_2_1_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let b_0_0 = m_0_0_1_0.clone();
            b_0_0_tx.send(b_0_0)?;
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
        let mut renew = false;
        while !renew {
          let sig = ctrl_0_0_0_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let computed_0_0_1 = Vec::new2();
            computed_0_0_1_tx.send(computed_0_0_1)?;
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
        let mut var_0 = futures_0_rx.recv()?;
        let cav_0_0_0 = var_0.recv().unwrap();
        cav_0_0_0_tx.send(cav_0_0_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = mesh_0_0_2_0_rx.recv()?;
        let var_1 = computed_0_1_0_rx.recv()?;
        let remaining_work_0_1_0 = var_0.apply_updates(var_1);
        remaining_work_0_1_0_tx.send(remaining_work_0_1_0)?;
        mesh_0_0_1_0_tx.send(var_0)?
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      let mut rt =
        std::sync::Arc::new(tokio::runtime::Builder::new()
          .threaded_scheduler()
          .core_threads(1)
          .build()
          .unwrap());
      loop {
        let var_1 = b_0_0_rx.recv()?;
        let var_2 = d_0_rx.recv()?;
        let futures_0 =
          {
            let (tx, rx) = std::sync::mpsc::channel();
            let work =
              async move { tx.send(compute_cavity(var_1, var_2)).unwrap() };
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
        let mut var_0 = bad_0_0_0_rx.recv()?;
        let res =
          {
            let sp = if var_0.len() < 10 { var_0.len() } else { 10 };
            let chunk = var_0.split_off(sp);
            (var_0, chunk)
          };
        let bad_0_n_0_0_0 = res.0;
        bad_0_n_0_0_0_tx.send(bad_0_n_0_0_0)?;
        let rest_0_0_0 = res.1;
        rest_0_0_0_tx.send(rest_0_0_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = remaining_work_0_1_0_rx.recv()?;
        let var_1 = rest_0_0_0_rx.recv()?;
        let remaining_work_0_0_0_0 = { var_0.extend(var_1.into_iter()); var_0 };
        remaining_work_0_0_0_0_tx.send(remaining_work_0_0_0_0)?;
        ()
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = mesh_0_0_3_rx.recv()?;
        let a_0_0 = var_0.clone();
        a_0_0_tx.send(a_0_0)?;
        mesh_0_0_2_0_tx.send(var_0)?
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = m_1_0_1_rx.recv()?;
        let bad_1_0_0 = var_0.find_bad();
        bad_1_0_0_tx.send(bad_1_0_0)?;
        m_1_0_0_0_tx.send(var_0)?
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut var_0 = mesh_0_0_1_0_rx.recv()?;
        let is_not_empty_0_0_0 = var_0.has_more_work();
        is_not_empty_0_0_0_tx.send(is_not_empty_0_0_0)?;
        mesh_0_0_0_0_tx.send(var_0)?
      }
    }));
  tasks
    .push(Box::new(move || -> _ {
      loop {
        let mut renew = false;
        let mut computed_0_0_1_0 = computed_0_0_1_rx.recv()?;
        while !renew {
          let sig = ctrl_2_0_rx.recv()?;
          let count = sig.1;
          for _ in 0 .. count {
            let var_1 = cav_0_0_0_rx.recv()?;
            computed_0_0_1_0.push(var_1);
            ()
          };
          let renew_next_time = sig.0;
          renew = renew_next_time;
          ()
        };
        computed_0_1_0_tx.send(computed_0_0_1_0)?;
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
