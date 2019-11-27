#![feature(proc_macro_hygiene)]
use clap::{App, Arg};
use intruder::decoder::simple::{decode_packet, DecoderState};
use intruder::decoder::DecodedPacket;
use intruder::detector::{run_detector, DetectorResult};
use intruder::*;
use ohua_codegen::ohua;
use ohua_runtime;
use std::collections::VecDeque;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use time::PreciseTime;

fn main() {
    let matches = App::new("Ohua Intruder Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about(
            "A Rust port of the intruder benchmark from the STAMP collection, implemented in Ohua, using {_py_size_} splits on smap.",
        )
        .arg(
            Arg::with_name("attacks")
                .long("attacks")
                .short("a")
                .help("Percentage of attacks to generate.")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("max_length")
                .long("length")
                .short("l")
                .help("Maximum length of a packet in bytes.")
                .takes_value(true)
                .default_value("16"),
        )
        .arg(
            Arg::with_name("flowcount")
                .long("number_flows")
                .short("n")
                .help("Number of flows to generate as input.")
                .takes_value(true)
                .default_value("1048576"),
        )
        .arg(
            Arg::with_name("seed")
                .long("seed")
                .short("s")
                .help("Seed to use for the random number generator")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("runs")
                .long("runs")
                .short("r")
                .takes_value(true)
                .help("The number of runs to conduct.")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .short("j")
                .help("Dump results as JSON file."),
        )
        .arg(
            Arg::with_name("outdir")
                .long("outdir")
                .short("o")
                .help("Sets the output directory for JSON dumps")
                .takes_value(true)
                .default_value("results"),
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

    // generate the input data
    let (input, attacks) = generate_stream(flowcount, attack_percentage, max_packet_len, rng_seed);
    if !json_dump {
        println!(
            "[INFO] Generated flows containing an attack: {}",
            attacks.len()
        );
    }

    let mut results = Vec::with_capacity(runs);

    for r in 0..runs {
        // prepare the data for the run
        // let input_data = TVar::new(input.clone());
        let input_data = input.clone();

        // start the clock
        let start = PreciseTime::now();

        // run the algorithm
        #[ohua]
        let result = algos::split{_py_size_}_detector(input_data);

        // stop the clock
        let end = PreciseTime::now();
        let runtime_ms = start.to(end).num_milliseconds();

        if !json_dump {
            println!("[INFO] Routing run {} completed.", r + 1);
        }

        // verify correctness
        if result.len() != attacks.len() {
            println!("[ERROR] Output verification failed. An incorrect number of attacks has been found. ({}/{})", result.len(), attacks.len());
        } else {
            results.push(runtime_ms);
        }
    }

    // note time
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua_split{_py_size_}-n{}-p{}-s{}-pl{}-r{}_log.json",
            out_dir, flowcount, attack_percentage, rng_seed, max_packet_len, runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"ohua-split\",
    \"splits\": {_py_size_},
    \"flow_count\": {flows},
    \"attack_percentage\": {attack_perc},
    \"attack_count\": {attacks},
    \"runs\": {runs},
    \"prng_seed\": {seed},
    \"max_packet_len\": {packet_len},
    \"results\": {res:?}
}}",
            flows = flowcount,
            attack_perc = attack_percentage,
            attacks = attacks.len(),
            runs = runs,
            seed = rng_seed,
            packet_len = max_packet_len,
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
        println!("    Runs:                  {}", runs);
        println!("\nRuntime in ms: {:?}", results);
    }
}

/// Process the packet queue, returning a VecDeque of DecodedPackets
fn process(
    mut decoder_state: DecoderState,
    mut packets: VecDeque<Packet>,
) -> VecDeque<DecodedPacket> {
    packets
        .drain(..)
        .filter_map(|p| decode_packet(p, &mut decoder_state))
        .collect()
}

/// Function that analyzes the incoming packet stream. The "benchmark" itself.
/// Everything inside this function is being timed.
///
/// Returns a Vec of flow IDs that contained an attack for later check
fn analyze_stream(decoded_flow: DecodedPacket) -> (DecodedPacket, DetectorResult) {
    let detector_result = run_detector(&decoded_flow.data);
    (decoded_flow, detector_result)
}

fn statistics(mut packets: Vec<(DecodedPacket, DetectorResult)>) -> Vec<usize> {
    packets
        .drain(..)
        .filter_map(|(packet, res)| {
            if res == DetectorResult::SignatureMatch {
                Some(packet.flow_id)
            } else {
                None
            }
        })
        .collect()
}

fn init_state() -> DecoderState {
    DecoderState::new()
}

fn split(mut v: VecDeque<DecodedPacket>) -> ({_py_splitup_args_}) {
    let parts = {_py_size_};

    let l = v.len() / parts;
    let mut rest = v.len() % parts;

    let mut paths_to_map = Vec::new();

    for t_num in 0..parts {
        paths_to_map.push(VecDeque::with_capacity(l));
        if rest > 0 {
            paths_to_map[t_num] = v.split_off(v.len() - l - 1);
            rest -= 1;
        } else {
            if v.len() <= l {
                paths_to_map[t_num] = v.split_off(0);
            } else {
                paths_to_map[t_num] = v.split_off(v.len() - l);
            }
        }
    }

    ({_py_splitup_pops_})
}

fn combine(
    {_py_join_args_}
) -> Vec<(DecodedPacket, DetectorResult)> {
    {_py_join_appends_}
    // v1.append(&mut v2);
    v1
}
