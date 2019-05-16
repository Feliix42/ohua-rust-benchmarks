use clap::{App, Arg};
use labyrinth::grid;
use labyrinth::parser;
use labyrinth::pathfinder;
use labyrinth::types::{Grid, Maze, Path, Point};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::thread;
use time::PreciseTime;

fn main() {
    let matches = App::new("threads-data-par Labyrinth Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the labyrinth benchmark from the STAMP collection using threads for data parallelism.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("threadcount")
                .long("splits")
                .short("s")
                .takes_value(true)
                .help("The number of worklist splits, equals the number of threads to run on.")
                .default_value("1")
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

    // thread number
    let thread_number = usize::from_str(matches.value_of("threadcount").unwrap())
        .expect("The entered thread count was not a valid uint");

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

        let paths2 = paths.clone();

        let start = PreciseTime::now();
        let filled_maze = route_paths(maze, paths2, thread_number);
        let end = PreciseTime::now();

        if !json_dump {
            println!("[INFO] Routing complete.");
        }

        let runtime_ms = start.to(end).num_milliseconds();

        if filled_maze.is_valid() {
            results.push(runtime_ms);
            mapped_paths.push(filled_maze.paths.len());
        } else {
            eprintln!("Incorrect path mappings found in maze!");
            return;
        }
    }

    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/thread-data-par-{}-p{}-t{}-r{}_log.json",
            out_dir,
            dimensions,
            paths.len(),
            thread_number,
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"configuration\": \"{conf}\",
    \"paths\": {paths},
    \"threads\": {threads},
    \"runs\": {runs},
    \"mapped\": {mapped:?},
    \"results\": {res:?}
}}",
            conf = dimensions,
            paths = paths.len(),
            threads = thread_number,
            runs = runs,
            mapped = mapped_paths,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] Successfully validated the maze.");
        println!("\nStatistics:");
        println!("    Maze configuration: {}", dimensions);
        println!("    Thread number:      {}", thread_number);
        println!("    Paths overall:      {}", paths.len());
        println!("    Runs:               {}", runs);
        println!("    Mapped:             {:?}", mapped_paths);
        println!("\nRouting Time: {:?} ms", results);
    }
}

fn route_paths(mut maze: Maze, mut to_map: Vec<(Point, Point)>, thread_number: usize) -> Maze {
    loop {
        // partition the vec
        let mut paths_to_map =
            vec![Vec::with_capacity(to_map.len() / thread_number); thread_number];
        let mut splitter = 0;
        for path in to_map.drain(..) {
            paths_to_map[splitter].push(path);
            splitter = (splitter + 1) % thread_number;
        }

        let mut handles = Vec::new();

        for points in paths_to_map.drain(..) {
            let g = maze.grid.clone();
            handles.push(thread::spawn(move || route(&g, points)));
        }

        to_map.clear();
        let mut found_paths = Vec::new();

        for handle in handles.drain(..) {
            found_paths.append(&mut handle.join().unwrap());
        }

        for p in found_paths.drain(..) {
            if grid::path_available(&maze.grid, &p) {
                grid::update_maze(&mut maze, p);
            } else {
                to_map.push((p.start, p.end));
            }
        }

        if to_map.is_empty() {
            return maze;
        }
    }
}

/// Attempts to route the paths from `to_map` on the grid.
fn route(grid: &Grid, mut to_map: Vec<(Point, Point)>) -> Vec<Path> {
    let mut found_paths = Vec::new();

    // search for a path for all point pairs (sort out any pairs w/o path)
    for pair in to_map.drain(..) {
        if let Some(path) = pathfinder::find_path(pair, grid) {
            found_paths.push(path);
        }
    }

    found_paths
}
