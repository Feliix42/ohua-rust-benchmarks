#![feature(proc_macro_hygiene)]
use clap::{App, Arg};
use labyrinth::parser;
use labyrinth::types::{Maze, Path, Point};
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
//                 .requires("json")
        )
        .arg(
            Arg::with_name("freq")
                .long("frequency")
                .short("f")
                .takes_value(true)
                .help("The update frequency for the maze data structure. Determines, after how many mapped paths an update is attempted.")
                .default_value("16")
        )
        .get_matches();

    // JSON Dump?
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // #runs
    let runs = usize::from_str(matches.value_of("runs").unwrap_or("1")).unwrap();

    // update frequency
    let updates = usize::from_str(matches.value_of("freq").unwrap()).unwrap();

    // input location & parsing
    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);

    let mut results = Vec::with_capacity(runs);
    let mut mapped_paths = Vec::with_capacity(runs);
    let mut collisions: Vec<usize> = Vec::with_capacity(runs);

    for _ in 0..runs {
        let maze = Maze::new(dimensions.clone(), None);

        if !json_dump {
            println!("[INFO] Loaded maze data from file.");
        }

        let paths2 = paths.clone();

        let start = PreciseTime::now();

        #[ohua]
        let (filled_maze, rollbacks) = modified_algos::split{_py_size_}_frequency(maze, paths2, updates);

        let end = PreciseTime::now();

        if !json_dump {
            println!("[INFO] Routing complete.");
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
            "{}/ohua_split_freq-{}-p{}-freq{}-t{}-r{}_log.json",
            out_dir,
            dimensions,
            paths.len(),
            updates,
            "{_py_size_}",
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"configuration\": \"{conf}\",
    \"paths\": {paths},
    \"runs\": {runs},
    \"workers\": {_py_size_},
    \"update_frequency\": {freq},
    \"mapped\": {mapped:?},
    \"collisions\": {collisions:?},
    \"results\": {res:?}
}}",
            conf = dimensions,
            paths = paths.len(),
            runs = runs,
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
        println!("    Workers:            {_py_size_}");
        println!("    Update frequency:   {}", updates);
        println!("    Mapped:             {:?}", mapped_paths);
        println!("    Collisions:         {:?}", collisions);
        println!("\nRouting Time: {:?} ms", results);
    }
}

fn splitup(
    mut v: Vec<(Point, Point)>,
) -> ({_py_splitup_args_}
) {
    let parts = {_py_size_};

    let l = v.len() / parts;
    let mut rest = v.len() % parts;

    let mut paths_to_map = vec![Vec::with_capacity(l); parts];

    for t_num in 0..parts {
        if rest > 0 {
            paths_to_map[t_num] = v.split_off(v.len() - l - 1);
            rest -= 1;
        } else {
            if v.len() <= l {
                paths_to_map[t_num] = v.split_off(0);
            } else {
                paths_to_map[t_num] = v.split_off(v.len() - l);
            }
        }
    }

    ({_py_splitup_pops_}
    )
}

fn merge(
    {_py_join_args_}
) -> Vec<Option<Path>> {{_py_join_appends_}
    v1
}
