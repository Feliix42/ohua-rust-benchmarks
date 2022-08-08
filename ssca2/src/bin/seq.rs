use ssca2::parameters::Parameters;
use clap::Parser;
use ssca2::generator::GraphSDG;

fn main() {
    let mut params = Parameters::parse();
    params.process();

    let sdg_data = GraphSDG::generate(params);
}
