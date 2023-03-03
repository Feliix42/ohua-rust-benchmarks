use blackscholes::{self, OptionData};
use clap::{App, Arg};
use cpu_time::ProcessTime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::ops::Range;
use std::thread::{self, JoinHandle};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let matches = App::new("Threaded blackscholes benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the blackscholes benchmark from the PARSEC collection, implemented using threads only")
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
            Arg::with_name("threadcount")
                .long("threads")
                .short("t")
                .help("Sets the number of threads to use for computation")
                .takes_value(true)
                .default_value("4")
        )
        .get_matches();

    // parse parameters
    let input_file = matches.value_of("INPUT").unwrap();
    let threadcount = usize::from_str(matches.value_of("threadcount").unwrap())
        .expect("Could not parse thread count");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let verify = matches.is_present("verify");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    // read and parse input data
    let input_data = Arc::new(OptionData::load_from_file(input_file).unwrap());

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
        let options = input_data.clone();
        // let options = input_data.clone();

        // start the clock
        let cpu_start = ProcessTime::now();
        let start = Instant::now();

        // run the algorithm
        let res = run_blackcholes(options, threadcount);

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
            let err_count = blackscholes::verify_all_results(&input_data, &res);
            if err_count != 0 {
                eprintln!("[error] Encountered {} errors in calculation.", err_count);
            }
        }
    }

    // write output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!("{}/threaded_opt-{}opt-t{}-r{}_log.json", out_dir, input_data.len(), threadcount, runs);
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"threaded-opt\",
    \"options\": {opt},
    \"threadcount\": {threadcount},
    \"runs\": {runs},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            opt = input_data.len(),
            threadcount = threadcount,
            runs = runs,
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

fn run_blackcholes(data: Arc<Vec<OptionData>>, threadcount: usize) -> Vec<f32> {
    let ranges = splitup(&data, threadcount);
    let mut handles: Vec<JoinHandle<Vec<f32>>> = Vec::with_capacity(threadcount);

    for range in ranges {
        let dat = data.clone();
        handles.push(
            thread::spawn(move || {
                dat[range]
                    .iter()
                    .map(OptionData::calculate_black_scholes)
                    .collect()
            })
        );
    }

    handles
        .drain(..)
        .map(|h| h.join().unwrap())
        .flatten()
        .collect()
}

//fn splitup(mut to_split: Vec<OptionData>, split_size: usize) -> Vec<Vec<OptionData>> {
    //// TODO: Is this the new optimized implementation?
    //let l = to_split.len() / split_size;
    //let mut rest = to_split.len() % split_size;

    //let mut splitted = Vec::new();

    //for t_num in 0..split_size {
        //splitted.push(Vec::with_capacity(l));
        //if rest > 0 {
            //splitted[t_num] = to_split.split_off(to_split.len() - l - 1);
            //rest -= 1;
        //} else {
            //if to_split.len() <= l {
                //splitted[t_num] = to_split.split_off(0);
            //} else {
                //splitted[t_num] = to_split.split_off(to_split.len() - l);
            //}
        //}
    //}

    //splitted
//}

/// Splits the input vector into evenly sized ranges for `split_size` workers.
fn splitup<T>(vec: &Vec<T>, split_size: usize) -> Vec<Range<usize>>
{
    let size = split_size;
    let element_count = vec.len();
    let mut rest = element_count % size;
    let window_len: usize = element_count / size;

    let mut res = Vec::with_capacity(size);

    let mut start = 0;
    for _ in 0..size {
        // calculate the length of the window (for even distribution of the `rest` elements)
        let len = if rest > 0 {
            rest -= 1;
            window_len + 1
        } else {
            window_len
        };

        let dst = start + len;
        res.push(start..dst);

        start = dst;
    }

    return res;
}

