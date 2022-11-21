use clap::Parser;
use strum_macros::{Display, EnumString};

mod action;
mod customer;
pub(crate) mod manager;
pub(crate) mod original;
pub(crate) mod prime;
mod reservation;

#[derive(Parser, Clone, Debug, EnumString, Display)]
pub enum Version {
    Original,
    Prime, // TODO add Ohua
}


#[derive(Parser, Clone, Debug)]
#[clap(author = "Sebastian Ertel, Felix Suchert", version = "1.0", about = "A Rust port ot the vacation benchmark from the STAMP collection", long_about = None)]
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
    /// Version of the benchmark
    #[clap(long = "bench_version", short = 'b', default_value_t = Version::Original)]
    pub version: Version,
}

//pub fn check_tables(_manager: Rc<RefCell<Manager>>) {
// TODO(feliix42): implement
//}
