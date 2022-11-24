use clap::Parser;
use cpu_time::ProcessTime;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Instant;
use vacation::{client::Client, manager::Manager, prime, Parameters, Version};

mod vacation;

fn main() {
    let params = Parameters::parse();

    let mut results = Vec::with_capacity(params.runs);
    let mut cpu_results = Vec::with_capacity(params.runs);

    for _ in 0..params.runs {
        // setup
        let mut rng = ChaCha12Rng::seed_from_u64(0);
        let manager = Manager::initialize(&mut rng, params.num_relations, params.clients * 32);

        let (start, cpu_start) = match params.version {
            Version::Naive => {
                let mgr = Arc::new(manager);
                let clients = vacation::initialize_clients(mgr.clone(), &params);

                // run benchmark
                let start = Instant::now();
                let cpu_start = ProcessTime::now();
                run(clients);

                (start, cpu_start)
            }
            Version::Prime => {
                let clients = prime::client::initialize_clients(&params);

                // run benchmark
                let start = Instant::now();
                let cpu_start = ProcessTime::now();

                prime::client::run_clients(
                    clients,
                    prime::database::Database::new(manager),
                    //prime::server::writes_before_reads,
                );

                (start, cpu_start)
            }
        };

        let cpu_stop = ProcessTime::now();
        let stop = Instant::now();

        results.push(stop.duration_since(start).as_millis());
        cpu_results.push(cpu_stop.duration_since(cpu_start).as_millis());

        // check results
        //vacation::check_tables(mgr);
    }

    if params.json {
        create_dir_all(&params.outdir).unwrap();
        let filename = format!(
            "{}/stm-{}-c{}-n{}-q{}-u{}-r{}-t{}-runs{}_log.json",
            params.outdir,
            params.version,
            params.clients,
            params.num_queries,
            params.percentage_queried,
            params.percentage_user_tx,
            params.num_relations,
            params.num_transactions,
            params.runs,
        );

        let mut f = File::create(&filename).unwrap();
        f.write_fmt(format_args!(
            "{{
        \"application\": \"vacation\",
        \"algorithm\": \"rust-stm\",
        \"variant\": \"{ver}\",
        \"threadcount\": {clients},
        \"clients\": {clients},
        \"num_queries\": {queries},
        \"percentage_queried\": {queried},
        \"percentage_user_tx\": {user_tx},
        \"num_relations\": {num_rel},
        \"num_transactions\": {num_tx},
        \"runs\": {runs},
        \"cpu_time\": {cpu:?},
        \"results\": {res:?}
        }}",
            ver = params.version,
            clients = params.clients,
            queries = params.num_queries,
            queried = params.percentage_queried,
            user_tx = params.percentage_user_tx,
            num_rel = params.num_relations,
            num_tx = params.num_transactions,
            runs = params.runs,
            cpu = cpu_results,
            res = results
        ))
        .unwrap();
    } else {
        println!("[INFO] All runs completed successfully.");
        println!("\nStatistics:");
        println!("    [...]");
        println!("    Clients/Threads: {}", params.clients);
        println!("    Runs: {}", params.runs);
        println!("\nCPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}

fn run(clients: Vec<Client<ChaCha12Rng>>) {
    let mut handles = Vec::with_capacity(clients.len());

    for mut c in clients {
        handles.push(thread::spawn(move || c.run()));
    }

    handles
        .into_iter()
        .map(JoinHandle::join)
        .for_each(Result::unwrap);
}
