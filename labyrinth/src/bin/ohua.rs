#![feature(proc_macro_hygiene, fnbox)]
use clap::{App, Arg};
use labyrinth::parser;
use labyrinth::types::{Maze, Point};
use ohua_codegen::ohua;
use ohua_runtime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use time::PreciseTime;

fn main() {
    let matches = App::new("Ohua Labyrinth Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the labyrinth benchmark from the STAMP collection using Ohua for implicit parallelism.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("runs")
            .long("runs")
            .short("r")
            .takes_value(true)
            .help("The number of runs to conduct.")
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .short("j")
                .help("Dump results as JSON file.")
        )
        .get_matches();

    // JSON Dump?
    let json_dump = matches.is_present("json");

    // #runs
    let runs = usize::from_str(matches.value_of("runs").unwrap_or("1")).unwrap();

    // input location & parsing
    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);

    let mut results = Vec::with_capacity(runs);
    let mut mapped_paths = Vec::with_capacity(runs);

    for _ in 0..runs {
        let maze = Maze::new(dimensions.clone(), None);

        if !json_dump {
            println!("[INFO] Loaded maze data from file.");
        }

        let start = PreciseTime::now();

        /* let filled_maze = {
            use crate::is_empty;
            use labyrinth::grid::update_maze;
            use labyrinth::pathfinder::find_path;
            use ohua_runtime::arcs::*;
            use ohua_runtime::lang::collect;
            use ohua_runtime::lang::id;
            use ohua_runtime::lang::smapFun;
            use ohua_runtime::lang::{send_once, Unit};
            use ohua_runtime::*;
            use std::boxed::FnBox;
            use std::sync::mpsc::Receiver;
            fn ctrl_1<T0: Clone + Send>(
                ctrl_inp: &Receiver<(bool, isize)>,
                var_in_0: &Receiver<T0>,
                var_out_0: &dyn ArcInput<T0>,
            ) -> Result<(), RunError> {
                let (renew_next_time, count) = ctrl_inp.recv()?;
                let (var_0,) = (var_in_0.recv()?,);
                for _ in 0..count {
                    var_out_0.dispatch(var_0.clone())?;
                }
                ctrl_sf_1(ctrl_inp, var_in_0, var_out_0, renew_next_time, (var_0,))
            };
            fn ctrl_sf_1<T0: Clone + Send>(
                ctrl_inp: &Receiver<(bool, isize)>,
                var_in_0: &Receiver<T0>,
                var_out_0: &dyn ArcInput<T0>,
                renew: bool,
                state_vars: (T0,),
            ) -> Result<(), RunError> {
                let (renew_next_time, count) = ctrl_inp.recv()?;
                let (var_0,) = if renew {
                    (var_in_0.recv()?,)
                } else {
                    state_vars
                };
                for _ in 0..count {
                    var_out_0.dispatch(var_0.clone())?;
                }
                ctrl_sf_1(ctrl_inp, var_in_0, var_out_0, renew_next_time, (var_0,))
            };
            fn ctrl_2<T0: Clone + Send, T1: Clone + Send>(
                ctrl_inp: &Receiver<(bool, isize)>,
                var_in_0: &Receiver<T0>,
                var_in_1: &Receiver<T1>,
                var_out_0: &dyn ArcInput<T0>,
                var_out_1: &dyn ArcInput<T1>,
            ) -> Result<(), RunError> {
                let (renew_next_time, count) = ctrl_inp.recv()?;
                let (var_0, var_1) = (var_in_0.recv()?, var_in_1.recv()?);
                for _ in 0..count {
                    var_out_0.dispatch(var_0.clone())?;
                    var_out_1.dispatch(var_1.clone())?;
                }
                ctrl_sf_2(
                    ctrl_inp,
                    var_in_0,
                    var_in_1,
                    var_out_0,
                    var_out_1,
                    renew_next_time,
                    (var_0, var_1),
                )
            };
            fn ctrl_sf_2<T0: Clone + Send, T1: Clone + Send>(
                ctrl_inp: &Receiver<(bool, isize)>,
                var_in_0: &Receiver<T0>,
                var_in_1: &Receiver<T1>,
                var_out_0: &dyn ArcInput<T0>,
                var_out_1: &dyn ArcInput<T1>,
                renew: bool,
                state_vars: (T0, T1),
            ) -> Result<(), RunError> {
                let (renew_next_time, count) = ctrl_inp.recv()?;
                let (var_0, var_1) = if renew {
                    (var_in_0.recv()?, var_in_1.recv()?)
                } else {
                    state_vars
                };
                for _ in 0..count {
                    var_out_0.dispatch(var_0.clone())?;
                    var_out_1.dispatch(var_1.clone())?;
                }
                ctrl_sf_2(
                    ctrl_inp,
                    var_in_0,
                    var_in_1,
                    var_out_0,
                    var_out_1,
                    renew_next_time,
                    (var_0, var_1),
                )
            };
            fn nth_0_2<T0, T1>(t: (T0, T1)) -> T0 {
                let (var_0, var_1) = t;
                var_0
            };
            fn nth_1_2<T0, T1>(t: (T0, T1)) -> T1 {
                let (var_0, var_1) = t;
                var_1
            };
            fn recur_2<T0: Send, T1: Send, R: Send>(
                condition: &Receiver<bool>,
                result_arc: &Receiver<R>,
                init_0: &Receiver<T0>,
                init_1: &Receiver<T1>,
                loop_0: &Receiver<T0>,
                loop_1: &Receiver<T1>,
                ctrl_arc: &dyn ArcInput<(bool, isize)>,
                cont_arc: &dyn ArcInput<(T0, T1)>,
                finish_arc: &dyn ArcInput<R>,
            ) -> Result<(), RunError> {
                ctrl_arc.dispatch((true, 1));
                cont_arc.dispatch((init_0.recv()?, init_1.recv()?));
                while (condition.recv()?) {
                    ctrl_arc.dispatch((true, 1));
                    cont_arc.dispatch((loop_0.recv()?, loop_1.recv()?));
                }
                ctrl_arc.dispatch((false, 0));
                finish_arc.dispatch(result_arc.recv()?);
                Ok(())
            }
            let (sf_19_out_0__sf_1_in_0, sf_1_in_0) = std::sync::mpsc::channel();
            let (sf_18_out_0__sf_1_in_1, sf_1_in_1) = std::sync::mpsc::channel();
            let (sf_20_out_0__sf_1_in_2, sf_1_in_2) = std::sync::mpsc::channel();
            let (sf_21_out_0__sf_1_in_3, sf_1_in_3) = std::sync::mpsc::channel();
            let (sf_18_out_0__sf_1_in_4, sf_1_in_4) = std::sync::mpsc::channel();
            let (sf_17_out_0__sf_1_in_5, sf_1_in_5) = std::sync::mpsc::channel();
            let (sf_1_out_0__sf_5_in_0, sf_5_in_0) = std::sync::mpsc::channel();
            let (sf_1_out_1__sf_5_in_1, sf_5_in_1) = std::sync::mpsc::channel();
            let (sf_1_out_2__sf_5_in_2, sf_5_in_2) = std::sync::mpsc::channel();
            let (sf_5_out_1__sf_8_in_0, sf_8_in_0) = std::sync::mpsc::channel();
            let (sf_8_out_1__sf_12_in_0, sf_12_in_0) = std::sync::mpsc::channel();
            let (sf_5_out_0__sf_12_in_1, sf_12_in_1) = std::sync::mpsc::channel();
            let (sf_12_out_0__sf_14_in_0, sf_14_in_0) = std::sync::mpsc::channel();
            let (sf_8_out_0__sf_14_in_1, sf_14_in_1) = std::sync::mpsc::channel();
            let (sf_8_out_2__sf_15_in_0, sf_15_in_0) = std::sync::mpsc::channel();
            let (sf_14_out_0__sf_15_in_1, sf_15_in_1) = std::sync::mpsc::channel();
            let (sf_5_out_0__sf_16_in_0, sf_16_in_0) = std::sync::mpsc::channel();
            let (sf_15_out_0__sf_16_in_1, sf_16_in_1) = std::sync::mpsc::channel();
            let (sf_16_out_0__sf_17_in_0, sf_17_in_0) = std::sync::mpsc::channel();
            let (sf_16_out_0__sf_18_in_0, sf_18_in_0) = std::sync::mpsc::channel();
            let (sf_17_out_0__sf_19_in_0, sf_19_in_0) = std::sync::mpsc::channel();
            let sf_5_out_0 =
                DispatchQueue::new(vec![sf_5_out_0__sf_12_in_1, sf_5_out_0__sf_16_in_0]);
            let sf_16_out_0 =
                DispatchQueue::new(vec![sf_16_out_0__sf_17_in_0, sf_16_out_0__sf_18_in_0]);
            let sf_17_out_0 =
                DispatchQueue::new(vec![sf_17_out_0__sf_1_in_5, sf_17_out_0__sf_19_in_0]);
            let sf_18_out_0 =
                DispatchQueue::new(vec![sf_18_out_0__sf_1_in_1, sf_18_out_0__sf_1_in_4]);
            let (result_snd, result_rcv) = std::sync::mpsc::channel();
            let mut tasks: Vec<Box<FnBox() -> Result<(), RunError> + Send + 'static>> = Vec::new();
            tasks.push(Box::new(move || loop {
                let r = find_path(sf_14_in_0.recv()?, sf_14_in_1.recv()?);
                sf_14_out_0__sf_15_in_1.dispatch(r)?
            }));
            tasks.push(Box::new(move || loop {
                let r = update_maze(sf_16_in_0.recv()?, sf_16_in_1.recv()?);
                sf_16_out_0.dispatch(r)?
            }));
            tasks.push(Box::new(move || loop {
                let r = nth_0_2(sf_17_in_0.recv()?);
                sf_17_out_0.dispatch(r)?
            }));
            tasks.push(Box::new(move || loop {
                let r = nth_1_2(sf_18_in_0.recv()?);
                sf_18_out_0.dispatch(r)?
            }));
            tasks.push(Box::new(move || loop {
                let r = is_empty(sf_19_in_0.recv()?);
                sf_19_out_0__sf_1_in_0.dispatch(r)?
            }));
            tasks.push(Box::new(move || {
                let r = id(maze);
                sf_20_out_0__sf_1_in_2.dispatch(r)?;
                Ok(())
            }));
            tasks.push(Box::new(move || {
                let r = id(paths.clone());
                sf_21_out_0__sf_1_in_3.dispatch(r)?;
                Ok(())
            }));
            tasks.push(Box::new(move || {
                recur_2(
                    &sf_1_in_0,
                    &sf_1_in_1,
                    &sf_1_in_2,
                    &sf_1_in_3,
                    &sf_1_in_4,
                    &sf_1_in_5,
                    &sf_1_out_0__sf_5_in_0,
                    &sf_1_out_1__sf_5_in_1,
                    &sf_1_out_2__sf_5_in_2,
                )?;
                Ok(())
            }));
            tasks.push(Box::new(move || {
                ctrl_2(
                    &sf_5_in_0,
                    &sf_5_in_1,
                    &sf_5_in_2,
                    &sf_5_out_0,
                    &sf_5_out_1__sf_8_in_0,
                )?;
                Ok(())
            }));
            tasks.push(Box::new(move || loop {
                smapFun(
                    &sf_8_in_0,
                    &sf_8_out_0__sf_14_in_1,
                    &sf_8_out_1__sf_12_in_0,
                    &sf_8_out_2__sf_15_in_0,
                )?;
            }));
            tasks.push(Box::new(move || {
                ctrl_1(&sf_12_in_0, &sf_12_in_1, &sf_12_out_0__sf_14_in_0)?;
                Ok(())
            }));
            tasks.push(Box::new(move || loop {
                collect(&sf_15_in_0, &sf_15_in_1, &sf_15_out_0__sf_16_in_1)?;
            }));
            run_tasks(tasks);
            result_rcv.recv().unwrap()
        }; */

        let paths2 = paths.clone();

        #[ohua]
        let filled_maze = transact(maze, paths2);

        // TODO: update "mapped paths stats"
        let end = PreciseTime::now();

        if !json_dump {
            println!("[INFO] Routing complete.");
        }

        let runtime_ms = start.to(end).num_milliseconds();

        if filled_maze.is_valid() {
            results.push(runtime_ms);
            mapped_paths.push(filled_maze.paths.len());
        } else {
            eprintln!("Incorrect path mappings found in maze: {:?}", filled_maze);
            return;
        }
    }

    if json_dump {
        create_dir_all("results").unwrap();
        let filename = format!(
            "results/ohua-{}-p{}-r{}_log.json",
            dimensions,
            paths.len(),
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"configuration\": \"{conf}\",
    \"paths\": {paths},
    \"runs\": {runs},
    \"mapped\": {mapped:?},
    \"results\": {res:?},
}}",
            conf = dimensions,
            paths = paths.len(),
            runs = runs,
            mapped = mapped_paths,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] Successfully validated the maze.");
        println!("\nStatistics:");
        println!("    Maze configuration: {}", dimensions);
        println!("    Paths overall:      {}", paths.len());
        println!("    Runs:               {}", runs);
        println!("    Mapped:             {:?}", mapped_paths);
        println!("\nRouting Time: {:?} ms", results);
    }
}

pub fn is_empty(v: Vec<(Point, Point)>) -> bool {
    v.len() == 0
}
