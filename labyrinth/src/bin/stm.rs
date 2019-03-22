use clap::{App, Arg};
use labyrinth::parser;
use labyrinth::pathfinder;
use labyrinth::types::{Maze, Point, Path, Grid};
use labyrinth::stm_grid;
use std::str::FromStr;
use std::thread;
use stm::atomically;
use time::PreciseTime;

fn main() {
    let matches = App::new("STM Labyrinth Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the labyrinth benchmark from the STAMP collection using software transactional memory.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("threadcount")
            .long("threads")
            .short("t")
            .takes_value(true)
            .help("The number of threads to run on.")
            .required(true)
        )
        .get_matches();

    // thread number
    let thread_number = usize::from_str(matches.value_of("threadcount").unwrap())
        .expect("The entered thread count was not a valid uint");

    // input location & parsing
    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);
    let maze = Maze::new(dimensions, None);
    let path_count = paths.len();

    println!("[INFO] Loaded maze data from file.");

    let start = PreciseTime::now();
    let filled_maze = route_paths(maze, paths, thread_number);
    let end = PreciseTime::now();

    println!("[INFO] Routing complete.");

    let runtime_ms = start.to(end).num_milliseconds();

    if filled_maze.is_valid() {
        println!("[INFO] Successfully validated the maze.");
        println!("\nStatistics:");
        println!("    Paths overall: {}", path_count);
        println!("    Mapped:        {}", filled_maze.paths.len());
        println!("    Unmapped:      {}", filled_maze.unmappable_paths.len());
        println!("\nRouting Time: {} ms", runtime_ms);
    } else {
        eprintln!("Incorrect path mappings found in maze: {:#?}", filled_maze);
    }
}

fn route_paths(mut maze: Maze, mut to_map: Vec<(Point, Point)>, thread_number: usize) -> Maze {
    // partition the vec
    let mut paths_to_map = vec![Vec::with_capacity(to_map.len() / thread_number); thread_number];
    let mut splitter = 0;
    for path in to_map.drain(..) {
        paths_to_map[splitter].push(path);
        splitter = (splitter + 1) % thread_number;
    }

        let mut handles = Vec::new();

        for points in paths_to_map.drain(..) {
            let g = maze.grid.clone();
            handles.push(thread::spawn(move || {
                route(&g, points)
            }));
        }

        for handle in handles {
            let (mut mapped, mut not_mapped) = handle.join().unwrap();
            maze.paths.append(&mut mapped);
            maze.unmappable_paths.append(&mut not_mapped);
        }

    maze
}

/// Attempts to route the paths from `to_map` on he grid using STM.
fn route(
    grid: &Grid,
    mut to_map: Vec<(Point, Point)>,
) -> (Vec<Path>, Vec<(Point, Point)>) {
    let mut mapped = Vec::new();
    let mut unmappable_paths = Vec::new();

    // search for a path for all point pairs (sort out any pairs w/o path)
    for pair in to_map.drain(..) {
        let ta_result = atomically(|trans| {
            if let Some(path) = pathfinder::find_path(pair.clone(), &grid, trans)? {
                stm_grid::update_grid(&grid, &path, trans)?;
                Ok(Some(path))
            } else {
                Ok(None)
            }
        });

        if let Some(path) = ta_result {
            mapped.push(path);
        } else {
            unmappable_paths.push(pair);
        }
    }

    (mapped, unmappable_paths)
}
