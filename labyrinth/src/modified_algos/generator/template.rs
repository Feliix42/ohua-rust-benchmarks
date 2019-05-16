#![feature(proc_macro_hygiene, fnbox)]
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
                .requires("json")
        )
        .get_matches();

    // JSON Dump?
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // #runs
    let runs = usize::from_str(matches.value_of("runs").unwrap_or("1")).unwrap();

    // input location & parsing
    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);

    let mut results = Vec::with_capacity(runs);
    let mut mapped_paths = Vec::with_capacity(runs);
    let mut collisions = Vec::with_capacity(runs);
    let mut iterations = Vec::with_capacity(runs);

    for _ in 0..runs {
        let maze = Maze::new(dimensions.clone(), None);

        if !json_dump {
            println!("[INFO] Loaded maze data from file.");
        }

        let start = PreciseTime::now();

        let paths2 = paths.clone();

        #[ohua]
        let (filled_maze, (rollbacks, iteration_count)) = modified_algos::transact_split{_py_size_}(maze, paths2);

        let end = PreciseTime::now();

        if !json_dump {
            println!("[INFO] Routing complete.");
        }

        let runtime_ms = start.to(end).num_milliseconds();

        if filled_maze.is_valid() {
            results.push(runtime_ms);
            mapped_paths.push(filled_maze.paths.len());
            collisions.push(rollbacks);
            iterations.push(iteration_count);
        } else {
            eprintln!("Incorrect path mappings found in maze: {:?}", filled_maze);
            return;
        }
    }

    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua_split{_py_size_}-{}-p{}-r{}_log.json",
            out_dir,
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
    \"collisions\": {collisions:?},
    \"iterations\": {iterations:?},
    \"results\": {res:?}
}}",
            conf = dimensions,
            paths = paths.len(),
            runs = runs,
            mapped = mapped_paths,
            collisions = collisions,
            iterations = iterations,
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
        println!("    Collisions:         {:?}", collisions);
        println!("    Iterations:         {:?}", iterations);
        println!("\nRouting Time: {:?} ms", results);
    }
}

fn splitup(
    mut v: Vec<(Point, Point)>,
) -> ({_py_splitup_args_}
) {
    let parts = {_py_size_};

    let mut paths_to_map = vec![Vec::with_capacity(v.len() / parts); parts];
    let mut splitter = 0;
    for path in v.drain(..) {
        paths_to_map[splitter].push(path);
        splitter = (splitter + 1) % parts;
    }

    ({_py_splitup_pops_}
    )
}

fn join(
    {_py_join_args_}
) -> Vec<Option<Path>> {{_py_join_appends_}
    v1
}
