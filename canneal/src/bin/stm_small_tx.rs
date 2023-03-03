use canneal::stm_netlist::Netlist;
use canneal::*;
use clap::{App, Arg};
use cpu_time::ProcessTime;
use rand::{Rng, RngCore};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use stm::{atomically, TVar};

fn main() {
    let matches = App::new("STM canneal benchmark")
        .version("1.0")
        .author("Felix Suchert <dev@felixsuchert.de>")
        .about("A Rust port of the canneal benchmark from the PARSEC collection, implemented using Rust-STM.")
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
            Arg::with_name("threadcount")
                .long("threads")
                .short("n")
                .help("Number of threads to use for computation")
                .takes_value(true)
                .default_value("4")
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

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();
    let threadcount = usize::from_str(matches.value_of("threadcount").unwrap())
        .expect("Could not parse number of threads to use");

    // read and parse input data
    let input_data = Netlist::new(input_file).expect("Failed to parse input file");

    if !json_dump {
        println!(
            "[info] Loaded {} netlist elements.",
            input_data.elements.len()
        );
    }

    // run the benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_time = Vec::with_capacity(runs);
    let mut computations = Vec::with_capacity(runs);

    if !json_dump {
        print!("[info] Running benchmark");
    }

    for _ in 0..runs {
        // clone the necessary data
        let netlist = Netlist::new(input_file).expect("Failed to parse input file");

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        let comps = run_annealer(netlist, initial_temp as f64, steps, swap_count, threadcount);

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
        computations.push(comps);
    }

    // write output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/stm_small_tx-t{}-{}opt-r{}_log.json",
            out_dir,
            threadcount,
            input_data.elements.len(),
            runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"stm_small-tx\",
    \"netlist_elements\": {opt},
    \"runs\": {runs},
    \"threadcount\": {threads},
    \"initial_temperature\": {init_tmp},
    \"max_number_temp_steps\": {steps},
    \"swaps_per_temp_step\": {swaps_per_temp},
    \"computations\": {comps:?},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            opt = input_data.elements.len(),
            runs = runs,
            threads = threadcount,
            init_tmp = initial_temp,
            steps = steps.unwrap_or(-1),
            swaps_per_temp = swap_count,
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
        println!("    Computations: {:?}", computations);
        println!("\nCPU-time used (ms): {:?}", cpu_time);
        println!("Runtime (ms): {:?}", results);
    }
}

fn run_annealer(
    global_netlist: Netlist,
    starting_temperature: f64,
    max_temperature_steps: Option<i32>,
    swaps_per_temp: usize,
    threadcount: usize,
) -> usize {
    let accepted_good_moves = TVar::new(0);
    let accepted_bad_moves = TVar::new(-1);
    let mut temp_steps_completed = 0;
    let temperature = TVar::new(starting_temperature);

    // calculated property in C++ version
    let moves_per_thread = swaps_per_temp / threadcount;

    // I'll just seed that thing with 0 for now
    let mut rng = ChaCha12Rng::seed_from_u64(0);

    let mut handles = Vec::new();
    let mut start_channels = Vec::new();
    let mut end_channels = Vec::new();

    for _ in 0..threadcount {
        // seed a thread-local rng
        let mut thread_rng = ChaCha12Rng::seed_from_u64(rng.next_u64());
        let netlist = global_netlist.clone();

        // create 2 channels to control flow in threads
        let (start_sx, start_rx) = mpsc::channel();
        let (end_sx, end_rx) = mpsc::channel();
        start_channels.push(start_sx);
        end_channels.push(end_rx);

        // clone necessary data
        let local_tmp = temperature.clone();
        let accepted_good = accepted_good_moves.clone();
        let accepted_bad = accepted_bad_moves.clone();

        // initialize elements
        #[allow(unused_assignments)]
        let mut idx_a = netlist.get_random_element(None, &mut thread_rng);
        let mut idx_b = netlist.get_random_element(None, &mut thread_rng);

        // spawn individual threads
        let h = thread::spawn(move || loop {
            let mut computations = 0;
            // wait for the "go" from the main thread
            if let Err(_) = start_rx.recv() {
                break;
            }

            // run internal loop
            for _ in 0..moves_per_thread {
                // get a single new element
                idx_a = idx_b;
                idx_b = netlist.get_random_element(Some(idx_a), &mut thread_rng);

                // atomically() requires a non-mutable item, which means I cannot use the RNG within
                let random_value = thread_rng.gen();

                let delta_cost = calculate_delta_routing_cost(
                    &netlist.elements[idx_a].read_atomic(),
                    &netlist.elements[idx_b].read_atomic(),
                );

                atomically(|trans| {
                    match assess_move(delta_cost, local_tmp.read(trans)?, random_value) {
                        MoveDecision::Good => {
                            accepted_good.modify(trans, |x| x + 1)?;
                            netlist.swap_locations(idx_a, idx_b, trans)
                        }
                        MoveDecision::Bad => {
                            accepted_bad.modify(trans, |x| x + 1)?;
                            netlist.swap_locations(idx_a, idx_b, trans)
                        }
                        MoveDecision::Rejected => Ok(()),
                    }
                });

                computations += 1; // + retries;
            }

            // notify main thread we're done
            end_sx.send(computations).unwrap();
        });
        handles.push(h);
    }

    let mut computations_total = 0;

    // main thread -> takes care of the decision making
    while keep_going(
        temp_steps_completed,
        max_temperature_steps,
        &accepted_good_moves,
        &accepted_bad_moves,
    ) {
        // set conditions for next run
        atomically(|trans| {
            temperature.modify(trans, |x| x / 1.5)?;
            accepted_good_moves.write(trans, 0)?;
            accepted_bad_moves.write(trans, 0)
        });

        // run the threads
        for sx in &start_channels {
            sx.send(()).unwrap();
        }

        // wait for execution completion
        for rx in &end_channels {
            computations_total += rx.recv().unwrap();
        }

        temp_steps_completed += 1;
    }

    println!(
        "[info] Finished after {} temperature steps.",
        temp_steps_completed
    );

    computations_total
}
