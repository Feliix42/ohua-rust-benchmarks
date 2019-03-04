mod labyrinth_types;
mod parser;

use clap::{App, Arg};
use labyrinth_types::Maze;

fn main() {
    let matches = App::new("Labyrinth Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the labyrinth benchmark from the STAMP collection.")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .value_name("usize")
                .help("Defines the number of threads to use.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();

    let (dimensions, paths) = parser::parse_file(input_file);

    let mut maze = Maze::new(dimensions, paths, None);
}
