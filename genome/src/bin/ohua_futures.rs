#![feature(proc_macro_hygiene)]
use clap::{App, Arg};
use cpu_time::ProcessTime;
use genome::gene::Gene;
use genome::ohua_sequencer::{self, SequencerItem};
use genome::segments::Segments;
use ohua_codegen::ohua;
use ohua_runtime;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver};
use time::PreciseTime;
use tokio::runtime::{Builder, Runtime};

fn main() {
    let matches = App::new("Ohua genome benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about(
            "A Rust port of the genome benchmark from the STAMP collection, implemented in Ohua with futures.",
        )
        .arg(
            Arg::with_name("genelength")
                .long("gene-length")
                .short("g")
                .help("Length of the gene")
                .takes_value(true)
                .default_value("16384"),
        )
        .arg(
            Arg::with_name("minnumber")
                .long("min-number")
                .short("n")
                .help("The minimal number of segments")
                .takes_value(true)
                .default_value("4194304"),
        )
        .arg(
            Arg::with_name("seglength")
                .long("segment-length")
                .short("s")
                .help("Length of a gene segment")
                .takes_value(true)
                .default_value("64"),
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
    let gene_length = usize::from_str(matches.value_of("genelength").unwrap())
        .expect("provided invalid value for `gene-length`");
    let min_number = usize::from_str(matches.value_of("minnumber").unwrap())
        .expect("provided invalid value for `min-number`");
    let segment_length = usize::from_str(matches.value_of("seglength").unwrap())
        .expect("provided invalid value for `segment-length`");

    // parse runtime parameters
    let runs =
        usize::from_str(matches.value_of("runs").unwrap()).expect("Could not parse number of runs");
    let json_dump = matches.is_present("json");
    let out_dir = matches.value_of("outdir").unwrap();

    let threadcount = usize::from_str(matches.value_of("threads").unwrap())
        .expect("Could not parse number of threads");

    // generate the gene and its segments
    let mut rng = ChaCha12Rng::seed_from_u64(0);

    let mut gene = Gene::create(gene_length, &mut rng);
    let segments = Segments::create(segment_length, min_number, &mut gene, &mut rng);
    if !json_dump {
        println!(
            "[INFO] Generated {} gene segments.",
            segments.contents.len()
        );
    }

    let mut results = Vec::with_capacity(runs);
    let mut cpu_results = Vec::with_capacity(runs);

    for r in 0..runs {
        // prepare the data for the run
        let input_data = segments.clone();
        let initial_overlap = input_data.length - 1;

        // start the clock
        let start = PreciseTime::now();
        let cpu_start = ProcessTime::now();

        // run the algorithm
        #[ohua]
        let result = algos::future_sequencer(input_data, initial_overlap, threadcount);

        // stop the clock
        let cpu_end = ProcessTime::now();
        let end = PreciseTime::now();
        let runtime_ms = start.to(end).num_milliseconds();
        let cpu_runtime_ms = cpu_end.duration_since(cpu_start).as_millis();

        if !json_dump {
            println!("[INFO] Genome sequencing run {} completed.", r + 1);
        }

        if result.len() != gene.contents.len() {
            eprintln!("[ERROR] Output verification failed. An error occured during genome sequencing. Sequenced genome length deviated from the original genome size ({}/{})", result.len(), gene.contents.len());
        } else {
            results.push(runtime_ms);
            cpu_results.push(cpu_runtime_ms);
        }
    }

    // generate output
    if json_dump {
        create_dir_all(out_dir).unwrap();
        let filename = format!(
            "{}/ohua_futures-g{}-n{}-s{}-t{}-r{}_log.json",
            out_dir, gene_length, min_number, segment_length, threadcount, runs
        );
        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
    \"algorithm\": \"ohua-futures\",
    \"threadcount\": {threadcount},
    \"gene_length\": {gene_len},
    \"min_segment_count\": {min_segment},
    \"segment_length\": {seg_len},
    \"runs\": {runs},
    \"cpu_time\": {cpu:?},
    \"results\": {res:?}
}}",
            threadcount = threadcount,
            gene_len = gene_length,
            min_segment = min_number,
            seg_len = segment_length,
            runs = runs,
            cpu = cpu_results,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] All runs completed successfully.");
        println!("\nStatistics:");
        println!("    Length of the generated gene: {}", gene_length);
        println!("    Minimal number of segments:   {}", min_number);
        println!("    Length of a gene segment:     {}", segment_length);
        println!("    Threads used:                 {}", threadcount);
        println!("    Runs:                         {}", runs);
        println!("\nCPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}

fn generate_iterator_indices(seq: Vec<SequencerItem>, threadcount: usize) -> Vec<Vec<usize>> {
    let v = seq.iter().enumerate().rev().map(|(i, _)| i).collect();
    split_evenly(v, threadcount)
}

/// Splits the input vector into evenly sized vectors for `split_size` workers.
fn split_evenly(mut to_split: Vec<usize>, split_size: usize) -> Vec<Vec<usize>> {
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

fn get_overlap(cur: usize) -> (usize, usize) {
    (cur, cur - 1)
}

fn remaining_computations(overlap: usize) -> bool {
    overlap > 0
}

// tokio-specific stuff
// TODO: Borrow here would be cool i guess
fn check_items(
    mut indices: Vec<usize>,
    overlap: usize,
    segments: Vec<SequencerItem>,
) -> Vec<Option<(usize, usize)>> {
    indices
        .drain(..)
        .map(|idx| ohua_sequencer::search_match(segments.clone(), overlap, idx))
        .collect()
}

fn spawn_onto_pool(
    mut indices: Vec<Vec<usize>>,
    overlap: usize,
    segments: Vec<SequencerItem>,
    threadcount: usize,
) -> (Runtime, Vec<Receiver<Vec<Option<(usize, usize)>>>>) {
    let rt = Builder::new()
        .threaded_scheduler()
        .num_threads(threadcount)
        .build()
        .unwrap();
    let mut handles = Vec::with_capacity(indices.len());

    for lst in indices.drain(..) {
        let (sx, rx) = mpsc::channel();
        let seg = segments.clone();

        rt.spawn(async move { sx.send(check_items(lst, overlap, seg)).unwrap() });

        handles.push(rx);
    }

    (rt, handles)
}

fn collect_and_shutdown(
    tokio_data: (Runtime, Vec<Receiver<Vec<Option<(usize, usize)>>>>),
) -> Vec<Option<(usize, usize)>> {
    let (_rt, mut handles) = tokio_data;

    let results = handles
        .drain(..)
        .map(|h| h.recv().unwrap())
        .flatten()
        .collect();

    results
}
