#![feature(proc_macro_hygiene)]
use clap::{App, Arg};
use cpu_time::ProcessTime;
use kmeans::{self, Centroid, Value};
use ohua_codegen::ohua;
use ohua_runtime;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use time::PreciseTime;
use tokio::runtime::{Builder, Runtime};

static TASKS_PER_THREAD: usize = 2;

fn main() {
    let matches = App::new("Future-based ohua kmeans benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rust port of the kmeans benchmark from the STAMP collection, implemented using a future-based Ohua algorithm.")
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
            Arg::with_name("threads")
                .long("threads")
                .short("p")
                .takes_value(true)
                .help("The number of threads to use in the futures threadpool")
                .default_value("4")
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

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    let threadcount = usize::from_str(matches.value_of("threads").unwrap())
        .expect("Provided invalid value for `threads`. Must be an uint.");

    // read and prepare the input data
    let mut clusters =
        Value::load_from_text_file(input_path).expect("Failed to load input from file");
    // apply zscore transformation
    if !dont_use_zscore {
        kmeans::apply_zscore_transform(&mut clusters);
    }

    let centroids = Arc::new(Centroid::randomly_generate(&clusters, cluster_count));

    // run benchmark itself
    let mut results = Vec::with_capacity(runs);
    let mut cpu_results = Vec::with_capacity(runs);

    for r in 0..runs {
        // prepare the data for the run
        let input_data = clusters.clone();
        let initial_centers = centroids.clone();

        // start the clock
        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        // run the algorithm
        #[ohua]
        let runs_necessary =
            algos::kmeans_future(input_data, initial_centers, threshold, threadcount);

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
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            cluster_count = cluster_count,
            threadcount = threadcount,
            threshold = threshold,
            input_path = input_path,
            value_count = clusters.len(),
            runs = runs,
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
        println!("\nCPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}

#[inline(always)]
fn calc_centroids(values: Vec<Value>, old_centroids: Arc<Vec<Centroid>>) -> Arc<Vec<Centroid>> {
    Arc::new(Centroid::from_assignments(&values, old_centroids.len()))
}

#[inline(always)]
fn inc(run_no: u32) -> u32 {
    run_no + 1
}

fn reassign_values(mut values: Vec<Value>, centroids: Arc<Vec<Centroid>>) -> Vec<(Value, f32)> {
    values
        .drain(..)
        .map(|mut value| {
            let new_cluster = value.find_nearest_centroid(&centroids);
            if new_cluster != value.associated_cluster {
                value.associated_cluster = new_cluster;
                (value, 1.0)
            } else {
                (value, 0.0)
            }
        })
        .collect()
}

#[inline(always)]
fn should_continue(current_delta: f32, threshold: f32, runs: u32) -> bool {
    current_delta > threshold && runs < 500
}

fn unpack_updates(mut values: Vec<(Value, f32)>) -> (Vec<Value>, f32) {
    let (new_values, mut deltas): (Vec<Value>, Vec<f32>) = values.drain(..).unzip();

    let current_delta = deltas.drain(..).sum::<f32>() / new_values.len() as f32;

    (new_values, current_delta)
}

/// Splits the input vector into evenly sized vectors for `split_size` workers.
fn splitup(mut to_split: Vec<Value>, split_size: usize) -> Vec<Vec<Value>> {
    let split_size = split_size * TASKS_PER_THREAD;
    let l = to_split.len() / split_size;
    let mut rest = to_split.len() % split_size;

    let mut splitted = Vec::new();

    for t_num in 0..split_size {
        splitted.push(Vec::with_capacity(l));
        if rest > 0 {
            splitted[t_num] = to_split.split_off(to_split.len() - l - 1);
            rest -= 1;
        } else {
            if to_split.len() <= l {
                splitted[t_num] = to_split.split_off(0);
            } else {
                splitted[t_num] = to_split.split_off(to_split.len() - l);
            }
        }
    }

    splitted
}

fn spawn_onto_pool(
    mut values: Vec<Vec<Value>>,
    centroids: Arc<Vec<Centroid>>,
    rt: Arc<Runtime>,
) -> Vec<Receiver<Vec<(Value, f32)>>> {
    let mut handles = Vec::with_capacity(values.len());

    for lst in values.drain(..) {
        let (sx, rx) = mpsc::channel();
        let ctr = centroids.clone();

        rt.spawn(async move { sx.send(reassign_values(lst, ctr)).unwrap() });

        handles.push(rx);
    }

    handles
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

fn collect_work<T>(mut receivers: Vec<Receiver<Vec<T>>>) -> Vec<T> {
    receivers
        .drain(..)
        .map(|h| h.recv().unwrap())
        .flatten()
        .collect()
}
