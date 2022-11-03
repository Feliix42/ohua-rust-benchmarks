use clap::{App, Arg};
use cpu_time::ProcessTime;
use kmeans::stm_centroid::ComputeCentroid;
use kmeans::{self, Centroid, Value};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use stm::{atomically, det_atomically, dtm, freeze, DTMHandle, TVar};
use time::PreciseTime;

fn main() {
    let matches = App::new("DSTM kmeans benchmark")
        .version("1.0")
        .author("Felix Suchert <dev@felixsuchert.de>")
        .about("A Rust port of the kmeans benchmark from the STAMP collection, implemented using STM")
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
                .help("The number of threads to use for computation")
                .takes_value(true)
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
    let mut convergence_after = Vec::with_capacity(runs);
    #[allow(unused_mut)]
    let mut computations: Vec<usize> = Vec::with_capacity(runs);

    for r in 0..runs {
        // prepare the data for the run
        let input_data = clusters.clone();
        let initial_centers = centroids.clone();

        // start the clock
        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        // run the algorithm
        let iterations = run_kmeans(input_data, initial_centers, threshold, threadcount);

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = PreciseTime::now();
        let runtime_ms = start.to(end).num_milliseconds();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        if !json_dump {
            println!("[INFO] kmeans run {} completed.", r + 1);
        }

        results.push(runtime_ms);
        cpu_results.push(cpu_runtime_ms);
        convergence_after.push(iterations);
        //computations.push(comps);
    }

    // generate output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/dstm-n{}-t{}-p{}-r{}_log.json",
            out_dir, cluster_count, threshold, threadcount, runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"rust-dstm\",
    \"cluster-count\": {cluster_count},
    \"threshold\": {threshold},
    \"input\": \"{input_path}\",
    \"values-count\": {value_count},
    \"threadcount\": {threads},
    \"runs\": {runs},
    \"converged_after\": {conv:?},
    \"computations\": {comps:?},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            cluster_count = cluster_count,
            threshold = threshold,
            input_path = input_path,
            value_count = clusters.len(),
            threads = threadcount,
            runs = runs,
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
        println!("    Number of threads used:      {}", threadcount);
        println!("    Runs:                        {}", runs);
        println!("\nConvergence after: {:?}", convergence_after);
        println!("Computations: {:?}", computations);
        println!("CPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}

fn run_kmeans(
    mut values: Vec<Value>,
    mut centroids: Arc<Vec<Centroid>>,
    threshold: f32,
    threadcount: usize,
) -> usize {
    let mut runs = 0;
    let delta = TVar::new(std::f32::MAX);
    let new_centroids = ComputeCentroid::new_empty(values[0].values.len(), centroids.len());
    //let mut total_computations: usize = 0;

    // exit conditions: either we are below our self-set threshold or 500 iterations have passed
    while runs < 500 && delta.read_atomic() > threshold {
        runs += 1;
        atomically(|trans| delta.write(trans, 0f32));

        let mut new_worklist = Vec::with_capacity(values.len());
        for chunk in values.chunks(threadcount) {
            // create the DTM handles
            let mut dtm = dtm();
            let work: Vec<(Value, DTMHandle)> = chunk.into_iter().map(std::borrow::ToOwned::to_owned).map(|item| (item, dtm.register())).collect();
            freeze(dtm);

            let mut handles: Vec<JoinHandle<Value>> = Vec::with_capacity(threadcount);
            for item in work {
                let local_delta = delta.clone();
                let local_centroids = centroids.clone();
                let local_new_centroids = new_centroids.clone();

                handles.push(thread::spawn(move || {
                    let (mut val, dtm_handle) = item;

                    let new_cluster = val.find_nearest_centroid(&local_centroids);
                    det_atomically(dtm_handle, |trans| {
                        if new_cluster != val.associated_cluster {
                            local_delta.modify(trans, |d| d + 1.0)?;
                        }
                        local_new_centroids[new_cluster].modify(trans, |mut ctr| {
                            ctr.add_value(&val);
                            ctr
                        })
                    });

                    if new_cluster != val.associated_cluster {
                        val.associated_cluster = new_cluster;
                    }

                    val
                }));
            }

            new_worklist.extend(handles.into_iter().map(JoinHandle::join).map(Result::unwrap));
        }

        values = new_worklist;

        // calculate the definite delta
        atomically(|trans| delta.modify(trans, |d| d / values.len() as f32));

        // Step 2: Calculate new centroids
        let c = atomically(|trans| {
            // calculate and assign the new centroids
            let c = new_centroids
                .iter()
                .map(TVar::read_atomic)
                .map(Centroid::from)
                .collect();

            // prepare the centroids for the next round
            for centroid in &new_centroids {
                centroid.modify(trans, |mut ctr| {
                    ctr.clear();
                    ctr
                })?;
            }

            Ok(Arc::new(c))
        });

        centroids = c;
    }

    runs
}
