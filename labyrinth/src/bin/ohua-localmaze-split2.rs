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

    for _ in 0..runs {
        let maze = Maze::new(dimensions.clone(), None);

        if !json_dump {
            println!("[INFO] Loaded maze data from file.");
        }

        let start = PreciseTime::now();

        let paths2 = paths.clone();

        #[ohua]
        let filled_maze = modified_algos::transact_split2(maze, paths2);

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
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua-split2-{}-p{}-r{}_log.json",
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
    \"results\": {res:?}
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

fn is_not_empty<T>(v: Vec<T>) -> bool {
    !v.is_empty()
}

fn splitup(mut v: Vec<(Point, Point)>) -> (Vec<(Point, Point)>, Vec<(Point, Point)>) {
    let parts = 2;

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

    (paths_to_map.pop().unwrap(), paths_to_map.pop().unwrap())
}

fn join(mut v1: Vec<Option<Path>>, mut v2: Vec<Option<Path>>) -> Vec<Option<Path>> {
    v1.append(&mut v2);
    v1
}

fn insert_path(p: Option<Path>, mut maze: Maze) -> Maze {
    if let Some(path) = p {
        for pt in &path.path {
            maze.grid[pt.x][pt.y][pt.z] = Field::Used;
        }
    }

    maze
}

fn get_one(mut v: Vec<(Point, Point)>) -> ((Point, Point), Vec<(Point, Point)>) {
    let elem = v.pop().unwrap();
    (elem, v)
}

fn add_to(mut v: Vec<Option<Path>>, elem: Option<Path>) -> Vec<Option<Path>> {
    v.push(elem);
    v
}

fn empty_vec() -> Vec<Option<Path>> {
    Vec::new()
}
