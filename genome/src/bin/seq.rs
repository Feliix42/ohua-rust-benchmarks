use clap::{App, Arg};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::str::FromStr;
use time::PreciseTime;
use genome::gene::Gene;
use genome::segments::Segments;

fn main() {
    let matches = App::new("Sequential genome benchmark")
        .version("1.0")
        .author("Felix Wittwer <dev@felixwittwer.de>")
        .about("A Rut port of the genome benchmark from the STAMP collection, implemented in a sequential manner.")
        .arg(
            Arg::with_name("genelength")
                .long("gene-length")
                .short("g")
                .help("Length of the gene")
                .takes_value(true)
                .default_value("16384")
        )
        .arg(
            Arg::with_name("minnumber")
                .long("min-number")
                .short("n")
                .help("The minimal number of segments")
                .takes_value(true)
                .default_value("4194304")    
        )
        .arg(
            Arg::with_name("seglength")
                .long("segment-length")
                .short("s")
                .help("Length of a gene segment")
                .takes_value(true)
                .default_value("64")
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

    // generate the gene and its segments
    let mut rng = ChaCha12Rng::seed_from_u64(0);

    let gene = Gene::create(gene_length, &mut rng);
    let segments = Segments::create(segment_length, min_number, &mut gene, &mut rng);
    
}
