use crate::generated::*;
use crate::types::*;
use clap::{App, Arg};
use cpu_time::ProcessTime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use time::PreciseTime;

mod generated;
mod original;
mod types;

fn main() {
    let matches = App::new("Future-based ohua kmeans benchmark using the new Ohua compiler")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the kmeans benchmark from the STAMP collection, implemented using a future-based Ohua algorithm with the new Ohua compiler.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file to describe the grid and paths.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("clusters-count")
                .long("clusters")
                .short("n")
                .help("Number of clusters")
                .takes_value(true)
                .default_value("40")
        )
        .arg(
            Arg::with_name("threshold")
                .long("threshold")
                .short("t")
                .help("Threshold below which a convergence is assumed. (Defined as maximum percentage of values that may change cluster in a single iteration)")
                .takes_value(true).default_value("0.05")
        )
        .arg(
            Arg::with_name("no_zscore")
                .short("z")
                .help("Don't perform zscore transformations on the data")
                .takes_value(false)
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
            Arg::with_name("sequential")
                .long("seq")
                .short("s")
                .help("Run the sequential ohua algorithm (bare)")
        )
        .get_matches();

    // parse benchmark parameters
    let input_path = matches.value_of("INPUT").unwrap();
    let cluster_count = usize::from_str(matches.value_of("clusters-count").unwrap())
        .expect("Provided invalid value for `clusters`. Must be an uint.");
    let threshold = f32::from_str(matches.value_of("threshold").unwrap()).expect(
        "Provided invalid value for `threshold`. Must be a non-negative floatin gpoint number",
    );
    let dont_use_zscore = matches.is_present("no_zscore");
    let sequential = matches.is_present("sequential");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    let threadcount = THREADCOUNT;

    // read and prepare the input data
    let mut clusters =
        Value::load_from_text_file(input_path).expect("Failed to load input from file");
    // apply zscore transformation
    if !dont_use_zscore {
        apply_zscore_transform(&mut clusters);
    }

    let centroids = Arc::new(Centroid::randomly_generate(&clusters, cluster_count));

    // run benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_results = Vec::with_capacity(runs);
    let mut convergence_after = Vec::with_capacity(runs);
    let mut computations = Vec::with_capacity(runs);

    for r in 0..runs {
        // prepare the data for the run
        let input_data = splitup(clusters.clone(), threadcount);
        let initial_centers = centroids.clone();

        // start the clock
        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        // run the algorithm
        let runs_necessary = if sequential {
            original::calculate(input_data, initial_centers, threshold, 0)
        } else {
            let (r, comps) = calculate(input_data, initial_centers, threshold, 0);
            computations.push(comps);
            r
        };

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = PreciseTime::now();
        let runtime_ms = start.to(end).num_milliseconds();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        if !json_dump {
            println!(
                "[INFO] kmeans run {} completed ({} iterations).",
                r + 1,
                runs_necessary
            );
        }

        results.push(runtime_ms);
        cpu_results.push(cpu_runtime_ms);
        convergence_after.push(runs_necessary);
    }

    // generate output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua_futures-n{}-t{}-p{}-r{}_log.json",
            out_dir, cluster_count, threshold, threadcount, runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"ohua-futures\",
    \"threadcount\": {threadcount},
    \"cluster-count\": {cluster_count},
    \"threshold\": {threshold},
    \"input\": \"{input_path}\",
    \"values-count\": {value_count},
    \"runs\": {runs},
    \"sequential\": {seq},
    \"converged_after\": {conv:?},
    \"computations\": {comps:?},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            cluster_count = cluster_count,
            threadcount = threadcount,
            threshold = threshold,
            input_path = input_path,
            value_count = clusters.len(),
            runs = runs,
            seq = sequential,
            conv = convergence_after,
            comps = computations,
            cpu = cpu_results,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] All runs completed successfully.");
        println!("\nStatistics:");
        println!("    Number of clusters:          {}", cluster_count);
        println!("    Threshold for conversion:    {}", threshold);
        println!("    Input file used:             {}", input_path);
        println!("    Number of values from input: {}", clusters.len());
        println!("    Threads used:                {}", threadcount);
        println!("    Runs:                        {}", runs);
        println!("\nConvergence after: {:?}", convergence_after);
        println!("Computations: {:?}", computations);
        println!("CPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}

fn splitup<T>(vec: Vec<T>, split_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let size = split_size * 2;
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
