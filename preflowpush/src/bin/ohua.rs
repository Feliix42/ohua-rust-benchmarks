use clap::{App, Arg};
use cpu_time::ProcessTime;
use preflow_push::functions::{self, Graph, PreflowPush};
use preflow_push::generated;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let matches = App::new("Sequential preflow push benchmark")
        .version("1.0")
        .author("Sebastian Ertel <sebastian.ertel@barkhauseninstitut.org>")
        .about("A Rust port of the preflow push benchmark from the Galois collection, implemented using Ohua. Finds the maximum flow in a network using the preflow push technique.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("sourceID")
                .long("source")
                .short("s")
                .takes_value(true)
                .help("ID of the source node")
                .default_value("0")
        )
        .arg(
            Arg::with_name("sinkID")
                .long("sink")
                .short("k")
                .takes_value(true)
                .help("ID of the sink node")
                .default_value("100")
        )
        .arg(
            Arg::with_name("relabel")
                .long("relabel")
                .takes_value(true)
                .help("Relabel interval X: relabel every X iterations")
                .default_value("0")
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

    // parse parameters
    let input_file = matches.value_of("INPUT").unwrap();
    let mut global_relabel_interval = u64::from_str(matches.value_of("relabel").unwrap())
        .expect("Could not parse relabel interval");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // read and parse input data
    let input_data = Graph::read_from_file(&input_file)
        .expect("Loading of input data failed. Ensure that all necessary files are present.");

    if global_relabel_interval == 0 {
        global_relabel_interval =
            (input_data.nodes.len() * functions::ALPHA * input_data.edge_count()) as u64;
    }

    let config = PreflowPush::new(global_relabel_interval);

    // if !json_dump {
    //     println!(
    //         "[info] Loaded {} mesh elements.",
    //         input_data.elements.len() + input_data.boundary_set.len()
    //     );
    // }

    // run the benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_time = Vec::with_capacity(runs);

    if !json_dump {
        println!("[info] Running benchmark");
    }

    for _ in 0..runs {
        // clone the necessary data
        // let mut mesh = Mesh::load_from_file(&input_file).expect("Failed to parse input file");
        let g = input_data.clone();
        let c = config.clone();

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        let _res = generated::run(g, c);

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = Instant::now();
        let runtime_ms = end.duration_since(start).as_millis();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        // verification
        // assert!(mesh.find_bad().is_empty());

        if !json_dump {
            print!(".");
        }

        results.push(runtime_ms);
        cpu_time.push(cpu_runtime_ms);
    }

    let input = input_file.split_once(".").unwrap().0;

    // write output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!("{}/ohua-{}-r{}_log.json", out_dir, input, runs);
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"sequential\",
    \"input\": {inp},
    \"runs\": {runs},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            inp = input,
            runs = runs,
            cpu = cpu_time,
            res = results
        ))
        .unwrap();
    } else {
        println!(" done!");

        println!("[info] All runs completed.");
        println!("\nStatistics:");
        println!("    Input file used: {}", input_file);
        println!("    Runs: {}", runs);
        println!("\nCPU-time used (ms): {:?}", cpu_time);
        println!("Runtime (ms): {:?}", results);
    }
}
