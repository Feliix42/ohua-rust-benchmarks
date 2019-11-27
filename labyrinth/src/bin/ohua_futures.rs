#![feature(proc_macro_hygiene)]
use clap::{App, Arg};
// use futures::future::{Future, ok, lazy};
use labyrinth::parser;
use labyrinth::pathfinder::find_path;
use labyrinth::types::{Maze, Path, Point};
use ohua_codegen::ohua;
use ohua_runtime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::mpsc::{Receiver, self};
use std::str::FromStr;
use time::PreciseTime;
use tokio::runtime::{Builder, Runtime};

fn main() {
    let matches = App::new("Ohua Labyrinth Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the labyrinth benchmark from the STAMP collection using Ohua and Futures for implicit parallelism.")
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
            .default_value("1")
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .short("j")
                .help("Dump results as JSON file.")
        )
        .arg(
            Arg::with_name("outdir")
                .long("outdir")
                .short("o")
                .help("Sets the output directory for JSON dumps")
                .takes_value(true)
                .default_value("results")
        )
        .arg(
            Arg::with_name("freq")
                .long("frequency")
                .short("f")
                .takes_value(true)
                .help("The update frequency for the maze data structure. Determines, after how many mapped paths an update is attempted. Each worker in the threadpool will get (frequency) / (# of simultaneous tasks) work items to process.")
                .default_value("16")
        )
        .arg(
            Arg::with_name("threads")
                .long("threads")
                .short("t")
                .takes_value(true)
                .help("The number of threads the threadpool should encompass.")
                .default_value("4")
        )
        .arg(
            Arg::with_name("simultaneous_tasks")
                .long("tasks")
                .short("s")
                .takes_value(true)
                .help("The number of tasks that shall be scheduled on the threadpool simultaneously.")
                .default_value("8")
        )
        .get_matches();

    // JSON Dump?
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // #runs
    let runs = usize::from_str(matches.value_of("runs").unwrap()).unwrap();

    // runtime parameters
    let updates = usize::from_str(matches.value_of("freq").unwrap()).unwrap();
    let threadcount = usize::from_str(matches.value_of("threads").unwrap()).unwrap();
    let taskcount = usize::from_str(matches.value_of("simultaneous_tasks").unwrap()).unwrap();

    // input location & parsing
    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);

    let mut results = Vec::with_capacity(runs);
    let mut mapped_paths = Vec::with_capacity(runs);
    let mut collisions: Vec<usize> = Vec::with_capacity(runs);

    for r in 0..runs {
        let maze = Maze::new(dimensions.clone(), None);

        if !json_dump && r == 0 {
            println!("[INFO] Loaded maze data from file.");
        }

        let paths2 = paths.clone();

        let start = PreciseTime::now();

        #[ohua]
        let (filled_maze, rollbacks) =
            modified_algos::futures(maze, paths2, updates, threadcount, taskcount);

        let end = PreciseTime::now();

        if !json_dump {
            println!("[INFO] Routing run {} complete.", r+1);
        }

        let runtime_ms = start.to(end).num_milliseconds();

        if filled_maze.is_valid() {
            results.push(runtime_ms);
            mapped_paths.push(filled_maze.paths.len());
            collisions.push(rollbacks);
        } else {
            eprintln!("Incorrect path mappings found in maze: {:?}", filled_maze);
            return;
        }
    }

    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua_futures-{}-p{}-freq{}-t{}-ts{}-r{}_log.json",
            out_dir,
            dimensions,
            paths.len(),
            updates,
            threadcount,
            taskcount,
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"configuration\": \"{conf}\",
    \"paths\": {paths},
    \"runs\": {runs},
    \"threads_in_pool\": {threadcount},
    \"simultaneous_tasks\": {taskcount},
    \"update_frequency\": {freq},
    \"mapped\": {mapped:?},
    \"collisions\": {collisions:?},
    \"results\": {res:?}
}}",
            conf = dimensions,
            paths = paths.len(),
            runs = runs,
            threadcount = threadcount,
            taskcount = taskcount,
            freq = updates,
            mapped = mapped_paths,
            collisions = collisions,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] Successfully validated the maze.");
        println!("\nStatistics:");
        println!("    Maze configuration: {}", dimensions);
        println!("    Paths overall:      {}", paths.len());
        println!("    Runs:               {}", runs);
        println!("    Threadpool Size:    {}", threadcount);
        println!("    Simultaneous Tasks: {}", taskcount);
        println!("    Update frequency:   {}", updates);
        println!("    Mapped:             {:?}", mapped_paths);
        println!("    Collisions:         {:?}", collisions);
        println!("\nRouting Time: {:?} ms", results);
    }
}

/// Splits the input vector into evenly sized vectors for `taskcount` workers.
fn split_evenly(mut points: Vec<(Point, Point)>, taskcount: usize) -> Vec<Vec<(Point, Point)>> {
    let l = points.len() / taskcount;
    let mut rest = points.len() % taskcount;

    let mut paths_to_map = vec![Vec::with_capacity(l); taskcount];

    for t_num in 0..taskcount {
        if rest > 0 {
            paths_to_map[t_num] = points.split_off(points.len() - l - 1);
            rest -= 1;
        } else {
            if points.len() <= l {
                paths_to_map[t_num] = points.split_off(0);
            } else {
                paths_to_map[t_num] = points.split_off(points.len() - l);
            }
        }
    }

    paths_to_map
}

fn vec_pathfind(maze: Maze, mut points: Vec<(Point, Point)>) -> Vec<Option<Path>> {
    points.drain(..).map(|p| find_path(&maze, p)).collect()
}

fn spawn_onto_pool(
    mut worklist: Vec<Vec<(Point, Point)>>,
    maze: Maze,
    threadcount: usize,
) -> (Runtime, Vec<Receiver<Vec<Option<Path>>>>) {
    let rt = Builder::new().threaded_scheduler().num_threads(threadcount).build().unwrap();
    let mut handles = Vec::with_capacity(worklist.len());

    for lst in worklist.drain(..) {
        let m = maze.clone();
        let (sx, rx) = mpsc::channel();

        rt.spawn(async move { sx.send(vec_pathfind(m, lst)).unwrap() });

        handles.push(rx);
    }

    (rt, handles)
}

fn collect_and_shutdown(
    tokio_data: (Runtime, Vec<Receiver<Vec<Option<Path>>>>),
) -> Vec<Option<Path>> {
    let (_rt, mut handles) = tokio_data;

    // moved up
    // rt.shutdown_on_idle().wait().unwrap();

    let results = handles.drain(..).map(|h| h.recv().unwrap()).flatten().collect();

    results
}
