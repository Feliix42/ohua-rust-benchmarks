#![feature(drain_filter)]
mod cavity;
mod element;
mod mesh;
mod point;

use crate::mesh::Mesh;
use clap::{App, Arg};
use cpu_time::ProcessTime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let matches = App::new("Sequential yada benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the yada benchmark from the Galois collection, implemented in a sequential manner.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name stem.")
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
            Arg::with_name("threadcount")
                .long("threads")
                .short("t")
                .takes_value(true)
                .help("The number of threads to use.")
                .default_value("4")
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

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();
    let threadcount = usize::from_str(matches.value_of("threadcount").unwrap())
        .expect("Could not parse threadcount");

    // read and parse input data
    let input_data = Mesh::load_from_file(&input_file)
        .expect("Loading of input data failed. Ensure that all necessary files are present.");

    if !json_dump {
        println!(
            "[info] Loaded {} mesh elements.",
            input_data.elements.read_atomic().len() + input_data.boundary_set.read_atomic().len()
        );
    }

    // run the benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_time = Vec::with_capacity(runs);

    if !json_dump {
        print!("[info] Running benchmark");
    }

    for _ in 0..runs {
        // clone the necessary data
        let mesh = Mesh::load_from_file(&input_file).expect("Failed to parse input file");

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        let _res = run_refining(mesh.clone(), threadcount);

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = Instant::now();
        let runtime_ms = end.duration_since(start).as_millis();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        // verification
        assert!(mesh.find_bad().is_empty());

        if !json_dump {
            print!(".");
        }

        results.push(runtime_ms);
        cpu_time.push(cpu_runtime_ms);
    }

    // write output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/seq-{}ele-r{}_log.json",
            out_dir,
            input_data.elements.read_atomic().len(),
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"sequential\",
    \"elements\": {ele},
    \"runs\": {runs},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            ele = input_data.elements.read_atomic().len(),
            runs = runs,
            cpu = cpu_time,
            res = results
        ))
        .unwrap();
    } else {
        println!(" done!");

        println!("[info] All runs completed.");
        println!("\nStatistics:");
        println!(
            "    Number of Mesh elements: {}",
            input_data.elements.read_atomic().len()
        );
        println!("    Input file used: {}", input_file);
        println!("    Runs: {}", runs);
        println!("\nCPU-time used (ms): {:?}", cpu_time);
        println!("Runtime (ms): {:?}", results);
    }
}

fn run_refining(mesh: Mesh, threadcount: usize) {
    let bad_queue = mesh.find_bad();

    mesh::refine(mesh, bad_queue, threadcount);
}
