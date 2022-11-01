use clap::Parser;
use cpu_time::ProcessTime;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::cell::RefCell;
use std::fs::create_dir_all;
use std::rc::Rc;
use std::time::Instant;
use vacation::{client::Client, manager::Manager, Parameters};

mod vacation;

fn main() {
    let params = Parameters::parse();

    if params.clients != 1 {
        println!(
            "[WARN] setting client count to anything else than 1 makes no sense and is discouraged"
        );
    }

    let mut results = Vec::with_capacity(params.runs);
    let mut cpu_results = Vec::with_capacity(params.runs);

    for _ in 0..params.runs {
        // setup
        let mut manager = Manager::new();
        let mut rng = ChaCha12Rng::seed_from_u64(0);
        manager.initialize(&mut rng, params.num_relations);
        let mgr = Rc::new(RefCell::new(manager));

        let mut clients = vacation::initialize_clients(mgr.clone(), &params);

        // run benchmark
        let start = Instant::now();
        let cpu_start = ProcessTime::now();

        run(&mut clients);

        let cpu_stop = ProcessTime::now();
        let stop = Instant::now();

        results.push(stop.duration_since(start).as_millis());
        cpu_results.push(cpu_stop.duration_since(cpu_start).as_millis());

        // check results
        vacation::check_tables(mgr);
    }

    if params.json {
        create_dir_all(&params.outdir).unwrap();
        //let filename = format!("{}/seq-s{}-i{:.1}-u{:.1}-l{}-p{}-r{}_log.json",
        //params.outdir,
        //params.scale,
        //params.probability_interclique_edges,
        //params.probability_unidirectional,
        //params.subgraph_edge_length,
        //params.max_parallel_edges,
        //params.runs,
        //);

        //let mut f = File::create(&filename).unwrap();
        //f.write_fmt(format_args!(
        //"{{
        //\"application\": \"ssca2\",
        //\"algorithm\": \"sequential\",
        //\"threadcount\": 1,
        //\"scaling-heuristic\": {scale},
        //\"probability-interclique\": {prob_ie},
        //\"probability-unidirectional\": \"{prob_u}\",
        //\"max-path-length\": {sub_edge_len},
        //\"max-par-edges\": {mpe},
        //\"runs\": {runs},
        //\"cpu_time\": {cpu:?},
        //\"results\": {res:?}
        //}}",
        //scale = params.scale,
        //prob_ie = params.probability_interclique_edges,
        //prob_u = params.probability_unidirectional,
        //sub_edge_len = params.subgraph_edge_length,
        //mpe = params.max_parallel_edges,
        //runs = params.runs,
        //cpu = cpu_results,
        //res = results
        //))
        //.unwrap();
    } else {
        println!("[INFO] All runs completed successfully.");
        println!("\nStatistics:");
        println!("    [...]");
        println!("    Runs: {}", params.runs);
        println!("\nCPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}

fn run(clients: &mut Vec<Client<ChaCha12Rng>>) {
    for c in clients {
        c.run();
    }
}
