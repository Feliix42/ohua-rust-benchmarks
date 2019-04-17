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

pub fn is_not_empty(v: Vec<(Point, Point)>) -> bool {
    !v.is_empty()
}
