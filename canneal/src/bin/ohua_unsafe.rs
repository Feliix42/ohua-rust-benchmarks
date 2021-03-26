#![feature(proc_macro_hygiene, get_mut_unchecked)]
use canneal::ohua_netlist::Netlist;
use canneal::*;
use clap::{App, Arg};
use cpu_time::ProcessTime;
use ohua_codegen::ohua;
use ohua_runtime;
use rand::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Instant;
use std::{
    fs::{create_dir_all, File},
    ops::Deref,
    sync::mpsc,
};
use tokio::runtime::{Builder, Runtime};

static TASKS_PER_THREAD: usize = 2;

fn main() {
    let matches = App::new("Ohua canneal benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the canneal benchmark from the PARSEC collection, implemented using Ohua.")
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
        .expect("Expected valid thread count");

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

    if !json_dump {
        print!("[info] Running benchmark");
    }

    for _ in 0..runs {
        // clone the necessary data
        let netlist = Arc::new(input_data.clone());

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        #[ohua]
        let runs = annealer(netlist, initial_temp as f64, steps, swap_count, threadcount);

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
    \"algorithm\": \"sequential\",
    \"netlist_elements\": {opt},
    \"runs\": {runs},
    \"threadcount\": {threads},
    \"initial_temperature\": {init_tmp},
    \"max_number_temp_steps\": {steps},
    \"swaps_per_temp_step\": {swaps_per_temp},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            opt = input_data.elements.len(),
            runs = runs,
            threads = threadcount,
            init_tmp = initial_temp,
            steps = steps.unwrap_or(-1),
            swaps_per_temp = swap_count,
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
        println!("\nCPU-time used (ms): {:?}", cpu_time);
        println!("Runtime (ms): {:?}", results);
    }
}

fn reduce_temp(temperature: f64) -> f64 {
    temperature / 1.5
}

fn get_rng() -> ChaCha12Rng {
    ChaCha12Rng::seed_from_u64(0)
}

fn get_swaps(swaps_per_temp: usize, threadcount: usize) -> usize {
    swaps_per_temp / (threadcount * TASKS_PER_THREAD)
}

fn increment(completed_steps: i32) -> i32 {
    completed_steps + 1
}

/// run the inner loop and create a log of changes
///
/// Returns: (List of changes, good moves, bad moves)
fn do_work(
    netlist: Arc<Netlist>,
    temperature: f64,
    swaps_per_step: usize,
    mut rng: ChaCha12Rng,
) -> (Vec<(usize, usize)>, usize, usize) {
    let mut changelog = Vec::new();
    let mut accepted_bad_moves = 0;
    let mut accepted_good_moves = 0;

    // TODO: Keep local state
    let mut idx_a;
    let mut idx_b = netlist.get_random_element(None, &mut rng);

    for _ in 0..swaps_per_step {
        // get a single new element
        idx_a = idx_b;
        idx_b = netlist.get_random_element(Some(idx_a), &mut rng);

        let delta_cost = netlist.calculate_delta_routing_cost(idx_a, idx_b);

        match assess_move(delta_cost, temperature, &mut rng) {
            MoveDecision::Good => {
                accepted_good_moves += 1;
                changelog.push((idx_a, idx_b));
                // netlist.swap_locations(idx_a, idx_b);
            }
            MoveDecision::Bad => {
                accepted_bad_moves += 1;
                changelog.push((idx_a, idx_b));
                // netlist.swap_locations(idx_a, idx_b);
            }
            MoveDecision::Rejected => (),
        }
    }

    (changelog, accepted_good_moves, accepted_bad_moves)
}

fn apply_changes(mut netlist: Arc<Netlist>, log: Vec<(usize, usize)>) -> Arc<Netlist> {
    //let mut new_netlist: Netlist = netlist.deref().to_owned();

    // TODO: One could possibly check for `strong_count` == 2 here to ensure no one besides the
    // ctrl_8 operator maintains a copy

    // the sanity of this is questionable
    unsafe {
        // opening pandoras box:
        let new_netlist = Arc::get_mut_unchecked(&mut netlist);
        let mut was_changed_before = vec![false; new_netlist.elements.len()];
        let mut errors = 0;
    
        for (from, to) in log {
            // quick check to detect overwrites
            if was_changed_before[from] {
                errors += 1;
            } else {
                was_changed_before[from] = true;
            }
            if was_changed_before[to] {
                errors += 1;
            }
            new_netlist.swap_locations(from, to);
        }
    
        // if errors > 0 {
        //     println!("Encountered {} overwrites of previous changes", errors);
        // }
    }

    netlist
}

// this function is problematic
fn create_rngs(mut rng: ChaCha12Rng, threadcount: usize) -> (ChaCha12Rng, Vec<ChaCha12Rng>) {
    let mut rngs = Vec::with_capacity(threadcount);

    for _ in 0..(threadcount * TASKS_PER_THREAD) {
        rngs.push(ChaCha12Rng::seed_from_u64(rng.next_u64()));
    }

    (rng, rngs)
}

fn spawn_onto_pool(
    netlist: Arc<Netlist>,
    rngs: Vec<ChaCha12Rng>,
    temperature: f64,
    swaps_per_thread: usize,
    rt: Arc<Runtime>,
) -> (
    Arc<Runtime>,
    Vec<Receiver<(Vec<(usize, usize)>, usize, usize)>>,
) {
    let mut handles = Vec::with_capacity(rngs.len());

    for rng in rngs.into_iter() {
        let (sx, rx) = mpsc::channel();
        let nl = netlist.clone();

        rt.spawn(async move {
            sx.send(do_work(nl, temperature, swaps_per_thread, rng))
                .unwrap()
        });

        handles.push(rx);
    }

    (rt, handles)
}

fn create_runtime(threadcount: usize) -> Arc<Runtime> {
    Arc::new(
        Builder::new()
            .threaded_scheduler()
            .core_threads(threadcount)
            .thread_name("ohua-tokio-worker")
            .build()
            .unwrap(),
    )
}

// fn collect_work<T>(tokio_data: (Arc<Runtime>, Vec<Receiver<Vec<T>>>)) -> Vec<T> {
//     let (_rt, mut receivers) = tokio_data;
//     receivers
//         .drain(..)
//         .map(|h| h.recv().unwrap())
//         .flatten()
//         .collect()
// }

fn collect_work(
    tokio_data: (
        Arc<Runtime>,
        Vec<Receiver<(Vec<(usize, usize)>, usize, usize)>>,
    ),
) -> (Vec<(usize, usize)>, i32, i32) {
    let (_rt, mut receivers) = tokio_data;
    let res: Vec<(Vec<(usize, usize)>, usize, usize)> =
        receivers.drain(..).map(|h| h.recv().unwrap()).collect();

    let mut good = 0;
    let mut bad = 0;
    let mut flattened = Vec::new();
    let _: Vec<()> = res
        .into_iter()
        .map(|(mut result, good_mv, bad_mv)| {
            flattened.append(&mut result);
            good += good_mv;
            bad += bad_mv;
        })
        .collect();

    (flattened, good as i32, bad as i32)
}
