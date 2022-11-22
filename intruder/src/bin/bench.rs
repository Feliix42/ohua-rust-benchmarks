use clap::{App, Arg};
use cpu_time::ProcessTime;
use std::str::FromStr;
use std::fs::{create_dir_all, File};
use std::io::Write;
use time::PreciseTime;
use strum_macros::EnumString;

use intruder::seq;
use intruder::stm;
use intruder::dstm;
use intruder::ohua;
use intruder::generated;

#[derive(EnumString)]
enum Runtime {
    Seq,
    STM,
    DSTM,
    OhuaSeq,
    Ohua
}

fn main() {
    let matches = App::new("Intruder Benchmark")
        .version("1.0")
        .author("Felix Suchert <dev@felixsuchert.de>")
        .author("Sebastian Ertel <sebastian.ertel@barkhauseninstitut.org>")
        .about("A Rust port of the intruder benchmark from the STAMP transactional memory benchmarks.")
        .arg(
            Arg::with_name("attacks")
                .long("attacks")
                .short("a")
                .help("Percentage of attacks to generate.")
                .takes_value(true)
                .default_value("10")
        )
        .arg(
            Arg::with_name("max_length")
                .long("length")
                .short("l")
                .help("Maximum number of flows per packet.")
                .takes_value(true)
                .default_value("16")
        )
        .arg(
            Arg::with_name("flowcount")
                .long("number_flows")
                .short("n")
                .help("Number of flows to generate as input.")
                .takes_value(true)
                .default_value("1048576")
        )
        .arg(
            Arg::with_name("seed")
                .long("seed")
                .short("s")
                .help("Seed to use for the random number generator")
                .takes_value(true)
                .default_value("1")
        )
        .arg(
            Arg::with_name("runs")
                .long("runs")
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
            Arg::with_name("threads")
            .long("threads")
            .short("t")
            .help("The number of threads to use ")
            .takes_value(true)
            .default_value("4")
        )
        .arg(
            Arg::with_name("runtime")
            .long("runtime")
            .short("rt")
            .help("The runtime to be executed: Seq | STM | DSTM | OhuaSeq | Ohua")
            .takes_value(true)
            .default_value("Seq")
        )
       .get_matches();

    // parse benchmark parameters
    let attack_percentage = u8::from_str(matches.value_of("attacks").unwrap())
        .expect("provided invalid input for `attacks`");
    let max_packet_len = u64::from_str(matches.value_of("max_length").unwrap())
        .expect("provided invalid input for `length`");
    let flowcount = usize::from_str(matches.value_of("flowcount").unwrap())
        .expect("provided invalid input for `number_flows`");
    let rng_seed = u64::from_str(matches.value_of("seed").unwrap())
        .expect("provided invalid input for `seed`");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();
    let threads = usize::from_str(matches.value_of("threads").unwrap())
        .expect("Could not parse number of threads");
    let rt = Runtime::from_str(matches.value_of("runtime").unwrap())
        .expect("Could not parse runtime");


    // generate the input data
    let (input, attacks) = intruder::generate_stream(flowcount, attack_percentage, max_packet_len, rng_seed);
    if !json_dump {
        println!(
            "[INFO] Generated flows containing an attack: {}",
            attacks.len()
        );
    }

    let mut results = Vec::with_capacity(runs);
    let mut cpu_results = Vec::with_capacity(runs);

    for r in 0..runs {
        // prepare the data for the run
        // let input_data = TVar::new(input.clone());
        let input_data = input.clone();
        let input_vec: Vec<_> = input.clone().into_iter().collect();

        // start the clock
        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        // run the algorithm
        let result = match rt {
            Runtime::Seq => seq::analyze_flow(input_data),
            Runtime::STM => stm::run_eval(input_data, threads),
            Runtime::DSTM => dstm::run_eval(input_vec, threads),
            Runtime::OhuaSeq => ohua::analyze_flow_3(input_data),
            Runtime::Ohua => generated::ohua::analyze_flow_3(input_data)
        };

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = PreciseTime::now();
        let runtime_ms = start.to(end).num_milliseconds();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        if !json_dump {
            println!("[INFO] Routing run {} completed.", r + 1);
        }

        // verify correctness
        if result.len() != attacks.len() {
            println!("[ERROR] Output verification failed. An incorrect number of attacks has been found. ({}/{})", result.len(), attacks.len());
        } else {
            results.push(runtime_ms);
            cpu_results.push(cpu_runtime_ms);
        }
    }

    // note time
    if json_dump {
        let algo = match rt {
            Runtime::Seq => "sequential",
            Runtime::STM => "rust-stm",
            Runtime::DSTM => "rust-dstm",
            Runtime::OhuaSeq => "ohua-seq",
            Runtime::Ohua => "ohua",
        };

        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/{}-n{}-p{}-s{}-pl{}-t{}-r{}_log.json",
            out_dir, algo, flowcount, attack_percentage, rng_seed, max_packet_len, threads, runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"{alg}\",
    \"flow_count\": {flows},
    \"attack_percentage\": {attack_perc},
    \"attack_count\": {attacks},
    \"runs\": {runs},
    \"prng_seed\": {seed},
    \"max_packet_len\": {packet_len},
    \"threadcount\": {threadcount},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            alg = algo,
            flows = flowcount,
            attack_perc = attack_percentage,
            attacks = attacks.len(),
            runs = runs,
            seed = rng_seed,
            packet_len = max_packet_len,
            threadcount = threads,
            cpu = cpu_results,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] All runs completed successfully.");
        println!("\nStatistics:");
        println!("    Number of flows:       {}", flowcount);
        println!("    Percentage of attacks: {}%", attack_percentage);
        println!("    PRNG seed:             {}", rng_seed);
        println!("    Maximal Packet Length: {}", max_packet_len);
        println!("    Generated Attacks:     {}", attacks.len());
        println!("    Threads used:          {}", threads);
        println!("    Runs:                  {}", runs);
        println!("\nCPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}


