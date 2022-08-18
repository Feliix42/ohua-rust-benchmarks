use crate::parameters::Parameters;
use clap::Parser;
use crate::generator::GraphSDG;
use crate::graph::Graph;

mod graph;
mod generator;
mod parameters;

fn main() {
    let mut params = Parameters::parse();
    params.process();

    let sdg_data = GraphSDG::generate(params);


    let start = std::time::Instant::now();
    let graph = Graph::compute(sdg_data);
    let end = std::time::Instant::now();

    println!("Time taken for kernel 1: {} ms", end.duration_since(start).as_millis());

}
