use clap::Parser;
use client::Client;
use manager::Manager;
use rand_chacha::ChaCha12Rng;
use std::cell::RefCell;
use std::rc::Rc;

mod action;
pub mod client;
mod customer;
pub mod manager;
mod operation;
mod reservation;

#[derive(Parser, Clone, Debug)]
#[clap(author = "Felix Suchert, Sebastian Ertel", version = "1.0", about = "A Rust port ot the vacation benchmark from the STAMP collection", long_about = None)]
pub struct Parameters {
    #[clap(long = "runs", default_value_t = 1)]
    pub runs: usize,
    #[clap(long = "json")]
    pub json: bool,
    #[clap(long, default_value = "results")]
    pub outdir: String,
    //#[clap(long = "threads", short = 't', default_value_t = 1)]
    //pub threads: usize,
    /// The number of clients
    #[clap(long = "clients", short = 'c', default_value_t = 1)]
    pub clients: usize,
    /// Number of user queries/transaction
    #[clap(long = "num_queries", short = 'n', default_value_t = 10)]
    pub num_queries: usize,
    /// Percentage of relations queried
    #[clap(long = "queried", short = 'q', default_value_t = 90)]
    pub percentage_queried: u8,
    /// Percentage of user transactions
    #[clap(long = "user", short = 'u', default_value_t = 80)]
    pub percentage_user_tx: u8,
    /// Number of possible relations
    #[clap(long = "relations", short = 'r', default_value_t = 1 << 16)]
    pub num_relations: usize,
    /// Number of transactions
    #[clap(long = "num_tx", short = 't', default_value_t = 1 << 26)]
    pub num_transactions: usize,
}

pub fn initialize_clients(manager: Rc<RefCell<Manager>>, params: &Parameters) -> Vec<Client<ChaCha12Rng>> {
    let mut clients = Vec::with_capacity(params.clients);

    let num_tx_per_client = (params.num_transactions as f64 / params.clients as f64 + 0.5) as usize;
    let query_range =
        (params.percentage_queried as f64 / 100_f64 * params.num_relations as f64 + 0.5) as usize;

    for _ in 0..params.clients {
        clients.push(Client::new(
            manager.clone(),
            num_tx_per_client,
            params.num_queries,
            query_range as u64,
            params.percentage_user_tx as i64,
        ));
    }

    clients
}

pub fn check_tables(_manager: Rc<RefCell<Manager>>) {
    // TODO(feliix42): implement
}
