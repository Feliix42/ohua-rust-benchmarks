use crate::generated::*;
use crate::types::*;
use clap::{App, Arg};
use cpu_time::ProcessTime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

mod generated;
mod original;
mod types;

fn main() {
    let matches = App::new("Ohua canneal benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the canneal benchmark from the PARSEC collection, implemented using the new Ohua compiler.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file describing the stock options to trade.")
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
        .arg(
            Arg::with_name("nswaps")
                .long("swaps")
                .help("The number of moves per temperature step")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("temp")
                .long("temperature")
                .short("t")
                .help("The starting temperature")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("nsteps")
                .long("max-steps")
                .short("m")
                .help("Maximal number of temperature steps")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("sequential")
                .long("seq")
                .short("s")
                .help("Run the sequential ohua algorithm (bare)")
        )
        .arg(
            Arg::with_name("threadcount")
                .long("threadcount")
                .short("tc")
                .help("Thread count/batch size")
        )
        .arg(
            Arg::with_name("frequency")
                .long("freq")
                .short("f")
                .help("C-Limit")
        )
       .get_matches();

    // parse parameters
    let input_file = matches.value_of("INPUT").unwrap();
    let swap_count = usize::from_str(matches.value_of("nswaps").unwrap()).unwrap();
    let initial_temp = usize::from_str(matches.value_of("temp").unwrap()).unwrap();
    let steps = matches
        .value_of("nsteps")
        .map(i32::from_str)
        .map(Result::unwrap);
    let sequential = matches.is_present("sequential");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();
    let threadcount = usize::from_str(matches.value_of("threadcount").unwrap()).expect("Could not parse thread count");
    let frequency = usize::from_str(matches.value_of("frequency").unwrap()).expect("Could not parse frequency/c-limit");

    // read and parse input data
    let input_data =
        Netlist::new(input_file, steps, swap_count).expect("Failed to parse input file");

    if !json_dump {
        println!(
            "[info] Loaded {} netlist elements.",
            input_data.elements.len()
        );
    }

    // run the benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_time = Vec::with_capacity(runs);
    let mut collisions = Vec::with_capacity(runs);
    let mut computations = Vec::with_capacity(runs);

    if !json_dump {
        print!("[info] Running benchmark");
    }

    for _ in 0..runs {
        // clone the necessary data
        // read and parse input data
        let mut netlist =
            Netlist::new(input_file, steps, swap_count).expect("Failed to parse input file");
        let workset = netlist.internal_state.generate_worklist();

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        let netlist = if sequential {
            original::annealer(netlist, workset, initial_temp as f64)
        } else {
            generated::original::annealer(netlist, workset, initial_temp as f64)
        };

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = Instant::now();
        let runtime_ms = end.duration_since(start).as_millis();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        if !json_dump {
            print!(".");
        }

        results.push(runtime_ms);
        cpu_time.push(cpu_runtime_ms);
        collisions.push(netlist.failed_updates);
        computations.push(netlist.internal_state.total_moves);
    }

    // write output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua-{}opt-t{}-r{}_log.json",
            out_dir,
            input_data.elements.len(),
            threadcount,
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"Ohua\",
    \"netlist_elements\": {opt},
    \"runs\": {runs},
    \"threadcount\": {threads},
    \"frequency\": {freq},
    \"initial_temperature\": {init_tmp},
    \"max_number_temp_steps\": {steps},
    \"swaps_per_temp_step\": {swaps_per_temp},
    \"collisions\": {coll:?},
    \"computations\": {comps:?},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            opt = input_data.elements.len(),
            runs = runs,
            threads = threadcount,
            freq = frequency,
            init_tmp = initial_temp,
            steps = steps.unwrap_or(-1),
            swaps_per_temp = swap_count,
            coll = collisions,
            comps = computations,
            cpu = cpu_time,
            res = results
        ))
        .unwrap();
    } else {
        println!(" done!");

        println!("[info] All runs completed.");
        println!("\nStatistics:");
        println!(
            "    Number of Netlist elements: {}",
            input_data.elements.len()
        );
        println!("    Input file used: {}", input_file);
        println!("    Runs: {}", runs);
        println!("    Threads: {}", threadcount);
        println!("    Initial Temperature: {}", initial_temp);
        println!("    Maximal number of temperature steps: {:?}", steps);
        println!("    Swaps per temperature step: {}", swap_count);
        println!("    Collisions: {:?}", collisions);
        println!("    Computations: {:?}", computations);
        println!("\nCPU-time used (ms): {:?}", cpu_time);
        println!("Runtime (ms): {:?}", results);
    }
}
