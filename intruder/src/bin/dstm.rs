use clap::{App, Arg};
use cpu_time::ProcessTime;
use intruder::decoder::stm_decoder::{decode_packet, StmDecoderState};
use intruder::detector::{run_detector, DetectorResult};
use intruder::*;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::thread::{self, JoinHandle};
use stm::{atomically, det_atomically, dtm, freeze, DTMHandle};
use time::PreciseTime;

fn main() {
    let matches = App::new("DSTM Intruder Benchmark")
        .version("1.0")
        .author("Felix Suchert <dev@felixsuchert.de>")
        .about("A Rust port of the intruder benchmark from the STAMP collection, implemented in software transactional memeory.")
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
            Arg::with_name("threads")
            .long("threads")
            .short("t")
            .help("The number of threads to use ")
            .takes_value(true)
            .default_value("4")
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

    // generate the input data
    let (input, attacks) = generate_stream(flowcount, attack_percentage, max_packet_len, rng_seed);
    let input: Vec<Packet> = input.into_iter().collect();
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

        // start the clock
        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        // run the algorithm
        let result = run_eval(input_data, threads);

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
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/dstm-n{}-p{}-s{}-pl{}-t{}-r{}_log.json",
            out_dir, flowcount, attack_percentage, rng_seed, max_packet_len, threads, runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"rust-dstm\",
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

// So, the above version does not work because the implemented algorithm
// for deterministic STM proceeds in rounds. That is, it needs to finish
// a round first in order to re-execute the retries.
// Here is the deadlock:
// Tx1 on thread 1 gets a retry which means it will try to redo its computation in the next round.
// In order to do so it waits for the first round to be done.
// But this in turn blocks Tx2 which would normally run after Tx1.
// We are left with putting Tx1 and Tx2 into a single transaction.
// Otherwise, I would not know how pause Tx1 and continue with Tx2 *on the same* thread. This would
// require quite a different programming model because for that the library. Would need to schedule
// the transactions across a pool of threads.

fn analyze_stream(
    packet: Packet,
    decoder_state: StmDecoderState,
    handle: DTMHandle,
) -> Option<usize> {
    let decoder_result = det_atomically(handle, |trans| {
        decode_packet(&packet, &decoder_state, trans)
        //let mut ds = Vec::new();
        //for p in packets {
        //let decoder_result = decode_packet(&p, &decoder_state, trans);
        //match decoder_result {
        //Ok(d) => ds.push(d),
        //Err(e) => return Err(e) // error type cast
        //}
        //}
        //Ok(ds)
    });

    //for decoder_result in decoder_results {
    if let Some(decoded_flow) = decoder_result {
        // process the output -> run the detector
        if run_detector(&decoded_flow.data) == DetectorResult::SignatureMatch {
            //found_attacks.push(decoded_flow.flow_id);
            return Some(decoded_flow.flow_id);
        }
    }

    None
    //}

    //found_attacks
}

fn run_eval(packets: Vec<Packet>, threadcount: usize) -> Vec<usize> {
    let mut found_attacks = Vec::new();
    let decoder_state = StmDecoderState::new(threadcount);

    // TODO(feliix42): This could be improved by processing multiple elements in a single Tx. But
    // that would already be an optimization.
    for chunk in packets.chunks(threadcount) {
        // create DTM handles
        let mut dtm = dtm();
        let work: Vec<(Packet, DTMHandle)> = chunk
            .into_iter()
            .map(std::borrow::ToOwned::to_owned)
            .map(|item| (item, dtm.register()))
            .collect();
        freeze(dtm);

        // spawn threads
        let mut threads = Vec::with_capacity(threadcount);
        for item in work {
            let ds = decoder_state.clone();
            let (packet, handle) = item;
            threads.push(thread::spawn(move || analyze_stream(packet, ds, handle)));
        }

        // collect work
        found_attacks.extend(
            threads
                .into_iter()
                .map(JoinHandle::join)
                .map(Result::unwrap)
                .filter(Option::is_some)
                .map(Option::unwrap),
        );
    }

    // State verification
    assert!(atomically(|trans3| decoder_state
        .fragments_map
        .is_empty(trans3)));

    found_attacks
}
