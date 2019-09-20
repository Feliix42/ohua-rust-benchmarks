use clap::{App, Arg};
use intruder::*;
use std::collections::VecDeque;
use std::str::FromStr;
use time::PreciseTime;

fn main() {
    let matches = App::new("Sequential Intruder Benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the intruder benchmark from the STAMP collection, implemented in a sequential manner.")
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
                .help("Maximum length of a packet in bytes.")
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

    // generate the input data
    let (input, attacks) = generate_stream(flowcount, attack_percentage, max_packet_len, rng_seed);
    println!(
        "[INFO] Generated flows containing an attack: {}",
        attacks.len()
    );

    // start the clock
    let start = PreciseTime::now();

    // run the algorithm
    let result = analyze_stream(input);

    // stop the clock
    let end = PreciseTime::now();
    let runtime_ms = start.to(end).num_milliseconds();

    // verify correctness
    // note time
}

/// Function that analyzes the incoming packet stream. The "benchmark" itself.
/// Everything inside this function is being timed.
///
/// Returns a Vec of flow IDs that contained an attack for later check
fn analyze_stream(mut packets: VecDeque<Packet>) -> Vec<usize> {
    unimplemented!()
    // get a single packet from the input stream (here a loop?)
    // decode the data (stateful!)
    // process the output (run the detector)
}
