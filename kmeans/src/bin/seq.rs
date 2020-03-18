use clap::{App, Arg};
use kmeans::{self, Value};
use std::str::FromStr;

fn main() {
    let matches = App::new("Sequential kmeans benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the kmeans benchmark from the STAMP collection, implemented in a sequential manner.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("clusters-count")
            .long("clusters")
            .short("n")
            .help("Number of clusters")
            .takes_value(true)
            .default_value("40")
        )
        .arg(
            Arg::with_name("threshold")
            .long("threshold")
            .short("t")
            .help("Threshold below which a convergence is assumed.")
            .takes_value(true).default_value("0.05")
        )
        .arg(
            Arg::with_name("no_zscore")
            .short("z")
            .help("Don't perform zscore transformations on the data")
            .takes_value(false)
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
        .get_matches();

    // parse benchmark parameters
    let input_path = matches.value_of("INPUT").unwrap();
    let cluster_count = usize::from_str(matches.value_of("clusters-count").unwrap())
        .expect("Provided invalid value for `clusters`. Must be an uint.");
    let threshold = f32::from_str(matches.value_of("threshold").unwrap()).expect(
        "Provided invalid value for `threshold`. Must be a non-negative floatin gpoint number",
    );
    let dont_use_zscore = matches.is_present("no_zscore");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // read and prepare the input data
    let mut input_data =
        Value::load_from_text_file(input_path).expect("Failed to load input from file");
    // TODO: zscore transform?

    kmeans::randomly_assign_cluster(&mut input_data, cluster_count);

    // run benchmark itself
}
