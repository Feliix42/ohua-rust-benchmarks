use clap::{App, Arg};
use cpu_time::ProcessTime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;
use types::*;

mod generated;
mod original;
mod types;

fn main() {
    let matches = App::new("Ohua blackscholes benchmark for the new compiler")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the blackscholes benchmark from the PARSEC collection, implemented using the Ohua framework")
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
            Arg::with_name("verify")
                .long("verify")
                .short("v")
                .help("Verify the calculated prices")
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
                .takes_value(true)
                .default_value("1")
        )
       .get_matches();

    // parse parameters
    let input_file = matches.value_of("INPUT").unwrap();
    let threadcount = usize::from_str(matches.value_of("threadcount").unwrap()).expect("Could not parse thread count");
    let sequential = matches.is_present("sequential");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let verify = matches.is_present("verify");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // read and parse input data
    let input_data = OptionData::load_from_file(input_file).unwrap();

    if !json_dump {
        println!("[info] Loaded {} options.", input_data.len());
    }

    // run the benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_time = Vec::with_capacity(runs);

    if !json_dump {
        print!("[info] Running benchmark");
    }

    for _ in 0..runs {
        // clone the necessary data
        let options = splitup(input_data.clone(), threadcount);
        // let options = input_data.clone();

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        let res = if sequential {
            original::calculate(options)
        } else {
            generated::original::calculate(options)
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

        // optionally run the verification
        if verify {
            let err_count = verify_all_results(&input_data, &res);
            if err_count != 0 {
                eprintln!("[error] Encountered {} errors in calculation.", err_count);
            }
        }
    }

    // write output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!("{}/ohua_futures-{}opt-t{}-r{}_log.json", out_dir, input_data.len(), threadcount, runs);
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"ohua-futures\",
    \"options\": {opt},
    \"threadcount\": {threadcount},
    \"runs\": {runs},
    \"sequential\": {seq},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            opt = input_data.len(),
            threadcount = threadcount,
            runs = runs,
            seq = sequential,
            cpu = cpu_time,
            res = results
        ))
        .unwrap();
    } else {
        println!(" done!");

        println!("[info] All runs completed.");
        println!("\nStatistics:");
        println!("    Number of options: {}", input_data.len());
        println!("    Input file used:   {}", input_file);
        println!("    Threads:           {}", threadcount);
        println!("    Runs:              {}", runs);
        println!("\nCPU-time used (ms): {:?}", cpu_time);
        println!("Runtime (ms): {:?}", results);
    }
}

fn splitup<T>(vec: Vec<T>, split_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let size = split_size * 2; // magic number to oversaturate workers
    let element_count = vec.len();
    let mut rest = element_count % size;
    let window_len: usize = element_count / size;
    let per_vec = if rest != 0 {
        window_len + 1
    } else {
        window_len
    };

    let mut res = vec![Vec::with_capacity(per_vec); size];

    let mut start = 0;
    for i in 0..size {
        // calculate the length of the window (for even distribution of the `rest` elements)
        let len = if rest > 0 {
            rest -= 1;
            window_len + 1
        } else {
            window_len
        };

        let dst = start + len;

        res[i].extend_from_slice(&vec[start..dst]);

        start = dst;
    }

    return res;
}
