use ssca2::parameters::Parameters;
use clap::Parser;
use ssca2::generator::GraphSDG;
use ssca2::graph::Graph;

fn main() {
    let mut params = Parameters::parse();
    params.process();

    let sdg_data = GraphSDG::generate(params);


    let start = std::time::Instant::now();
    let graph = Graph::compute(sdg_data);
    let end = std::time::Instant::now();

    println!("Time taken for kernel 1: {} ms", end.duration_since(start).as_millis());

}
