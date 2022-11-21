use clap::Parser;
use cpu_time::ProcessTime;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::cell::RefCell;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::rc::Rc;
use std::time::Instant;
use vacation::{manager::Manager, Parameters};
use vacation::{original, prime, Version};

mod vacation;

fn main() {
    let mut params = Parameters::parse();

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

        let (start, cpu_start) = match params.version {
            Version::Original => {
                let mgr = Rc::new(RefCell::new(manager));
                let mut clients = original::client::initialize_clients(mgr.clone(), &params);

                // run benchmark
                let start = Instant::now();
                let cpu_start = ProcessTime::now();

                original::client::run(&mut clients);

                (start, cpu_start)
            }
            Version::PrimeSingleClient => {
                params.num_transactions = params.num_transactions * params.clients;
                params.clients = 1;
                let mut clients = prime::client::initialize_clients(&params);
                assert!(clients.len() == 1);
                let client = clients.pop().unwrap();

                // run benchmark
                let start = Instant::now();
                let cpu_start = ProcessTime::now();

                prime::client::run_client(client, prime::database::Database::new(manager));

                (start, cpu_start)
            }
            Version::PrimeNaive => {
                let clients = prime::client::initialize_clients(&params);

                // run benchmark
                let start = Instant::now();
                let cpu_start = ProcessTime::now();

                prime::client::run_clients(
                    clients,
                    prime::database::Database::new(manager),
                    prime::server::naive,
                );

                (start, cpu_start)
            }
            Version::PrimeSmart => {
                let clients = prime::client::initialize_clients(&params);

                // run benchmark
                let start = Instant::now();
                let cpu_start = ProcessTime::now();

                prime::client::run_clients(
                    clients,
                    prime::database::Database::new(manager),
                    prime::server::writes_before_reads
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
            "{}/seq-c{}-n{}-q{}-u{}-r{}-t{}-runs{}_log.json",
            params.outdir,
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
        \"algorithm\": \"sequential\",
        \"threadcount\": 1,
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
        println!("    Runs: {}", params.runs);
        println!("\nCPU-time used (ms): {:?}", cpu_results);
        println!("Runtime in ms: {:?}", results);
    }
}
