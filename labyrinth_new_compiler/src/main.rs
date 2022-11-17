#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use clap::{App, Arg};
// use futures::future::{Future, ok, lazy};
use cpu_time::ProcessTime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use time::PreciseTime;
//use tokio::runtime::{Builder, Runtime};

mod benchs;
mod generated;
mod grid;
mod original;
mod parser;


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
            Arg::with_name("sequential")
                .long("seq")
                .short("s")
                .help("Run the sequential ohua algorithm (bare)")
        )
        .get_matches();

    // JSON Dump?
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();
    let sequential = matches.is_present("sequential");
    let ndp = matches.is_present("no_data_par");
    let nam = matches.is_present("no_amorphous");

    // #runs
    let runs = usize::from_str(matches.value_of("runs").unwrap()).unwrap();

    // runtime parameters
    //let updates = usize::from_str(matches.value_of("freq").unwrap()).unwrap();
    //let threadcount = usize::from_str(matches.value_of("threads").unwrap()).unwrap();

    // input location & parsing
    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);

    let mut results = Vec::with_capacity(runs);
    let mut cpu_results = Vec::with_capacity(runs);
    let mut mapped_paths: Vec<usize> = Vec::with_capacity(runs);
    #[allow(unused_mut)]
    let mut collisions: Vec<usize> = Vec::with_capacity(runs);
    let mut computations: Vec<usize> = Vec::with_capacity(runs);

    for r in 0..runs {
        //let maze = Arc::new(Maze::new(dimensions.clone(), None));

        if !json_dump && r == 0 {
            println!("[INFO] Loaded maze data from file.");
        }

        let paths2 = paths.clone().into_iter().map(Some).collect();
        let dims2 = dimensions.clone();

        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        //#[ohua]
        //let (filled_maze, rollbacks) =
        //modified_algos::futures(maze, paths2, updates, threadcount, taskcount);
        let (filled_maze, retries) = if sequential {
            original::run(dims2, paths2, 200)
        } else {
            generated::original::run(dims2, paths2, 200)
        };

        let cpu_end = ProcessTime::now();
        let end = PreciseTime::now();

        if !json_dump {
            println!("[INFO] Routing run {} complete.", r + 1);
        }

        let runtime_ms = start.to(end).num_milliseconds();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        if filled_maze.is_valid() {
            results.push(runtime_ms);
            cpu_results.push(cpu_runtime_ms);
            mapped_paths.push(filled_maze.paths.len());
            collisions.push(retries);
            computations.push(retries + paths.len());
        } else {
            eprintln!("Incorrect path mappings found in maze: {:?}", filled_maze);
            return;
        }
    }

    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
//            "{}/new_ohua_futures-{}-p{}-freq{}-t{}-r{}_log.json",
            "{}/new_ohua_futures-{}-p{}-freq{}_log.json",
            out_dir,
            dimensions,
            paths.len(),
//            updates,
//            threadcount,
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"ohua-futures\",
    \"configuration\": \"{conf}\",
    \"paths\": {paths},
    \"runs\": {runs},
    \"sequential\": {seq},
    \"mapped\": {mapped:?},
    \"collisions\": {collisions:?},
    \"computations\": {comps:?},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
//           "{{
//    \"algorithm\": \"ohua-futures\",
//    \"configuration\": \"{conf}\",
//    \"paths\": {paths},
//    \"runs\": {runs},
//    \"sequential\": {seq},
//    \"threadcount\": {threadcount},
//    \"update_frequency\": {freq},
//    \"mapped\": {mapped:?},
//    \"collisions\": {collisions:?},
//    \"computations\": {comps:?},
//    \"cpu_time\": {cpu:?},
//    \"results\": {res:?}
//}}",
            conf = dimensions,
            paths = paths.len(),
            runs = runs,
            seq = sequential,
//            threadcount = threadcount,
            comps = computations,
//            freq = updates,
            mapped = mapped_paths,
            collisions = collisions,
            cpu = cpu_results,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] Successfully validated the maze.");
        println!("\nStatistics:");
        println!("    Maze configuration: {}", dimensions);
        println!("    Paths overall:      {}", paths.len());
        println!("    Runs:               {}", runs);
//        println!("    Threadpool Size:    {}", threadcount);
//        println!("    Update frequency:   {}", updates);
        println!("    Mapped:             {:?}", mapped_paths);
        println!("    Collisions:         {:?}", collisions);
        println!("    Computations:       {:?}", computations);
        println!("\nCPU time used: {:?} ms", cpu_results);
        println!("Routing Time: {:?} ms", results);
    }
}
