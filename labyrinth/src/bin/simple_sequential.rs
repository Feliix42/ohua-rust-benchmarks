use clap::{App, Arg};
use labyrinth::grid;
use labyrinth::parser;
use labyrinth::pathfinder;
use labyrinth::types::{Maze, Point};
use time::PreciseTime;

fn main() {
    let matches = App::new("Sequential Labyrinth Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the labyrinth benchmark from the STAMP collection without any parallelism.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let (dimensions, paths) = parser::parse_file(input_file);
    let maze = Maze::new(dimensions.clone(), None);
    let path_count = paths.len();

    println!("[INFO] Loaded maze data from file.");

    let start = PreciseTime::now();
    let filled_maze = route_paths(maze, paths);
    let end = PreciseTime::now();

    println!("[INFO] Routing complete.");

    let runtime_ms = start.to(end).num_milliseconds();

    if filled_maze.is_valid() {
        println!("[INFO] Successfully validated the maze.");
        println!("\nStatistics:");
        println!("    Maze configuration: {}", dimensions);
        println!("    Paths overall:      {}", path_count);
        println!("    Mapped:             {}", filled_maze.paths.len());
        println!(
            "    Unmapped:           {}",
            filled_maze.unmappable_paths.len()
        );
        println!("\nRouting Time: {} ms", runtime_ms);
    } else {
        eprintln!("Incorrect path mappings found in maze: {:#?}", filled_maze);
    }
}

fn route_paths(mut maze: Maze, mut to_map: Vec<(Point, Point)>) -> Maze {
    let mut mapped = Vec::new();

    // search for a path for all point pairs (sort out any pairs w/o path)
    for pair in to_map.drain(..) {
        if let Some(path) = pathfinder::find_path(pair.clone(), &maze.grid) {
            mapped.push(path);
        } else {
            maze.unmappable_paths.push(pair);
        }
    }

    // update the maze
    let (new_maze, to_remap) = grid::update_maze(maze, mapped);

    if to_remap.is_empty() {
        new_maze
    } else {
        route_paths(new_maze, to_remap)
    }
}
