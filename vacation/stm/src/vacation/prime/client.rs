use crate::vacation::action::Action;
use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::prime::database as db;
use crate::vacation::prime::server;
use crate::vacation::reservation::ReservationType;
use crate::vacation::Parameters;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;

pub struct Client<T: RngCore + SeedableRng + Clone> {
    random: T,
    num_operation: usize,
    num_query_per_transaction: usize,
    query_range: u64,
    percent_user: i64,

    // state
    op: usize,
}

pub fn initialize_clients(params: &Parameters) -> Vec<Client<ChaCha12Rng>> {
    let mut clients = Vec::with_capacity(params.clients);

    let num_tx_per_client = (params.num_transactions as f64 / params.clients as f64 + 0.5) as usize;
    let query_range =
        (params.percentage_queried as f64 / 100_f64 * params.num_relations as f64 + 0.5) as usize;

    for _ in 0..params.clients {
        clients.push(Client::new(
            num_tx_per_client,
            params.num_queries,
            query_range as u64,
            params.percentage_user_tx as i64,
        ));
    }

    clients
}
impl<T: 'static + RngCore + SeedableRng + Clone> Client<T> {
    pub fn new(
        num_operation: usize,
        num_query_per_transaction: usize,
        query_range: u64,
        percent_user: i64,
    ) -> Self {
        Client {
            random: <T as SeedableRng>::seed_from_u64(1),
            num_operation,
            num_query_per_transaction,
            query_range,
            percent_user,
            op: 0,
        }
    }

    fn next_program(&mut self) -> Option<Box<dyn Program>> {
        if self.op < self.num_operation {
            self.op += 1;
            let r = self.random.gen::<i64>() % 100;
            let action = select_action(r, self.percent_user);
            Some(match action {
                Action::MakeReservation => Box::new(MkReservation::new(
                    self.random.clone(),
                    self.query_range,
                    self.random.gen::<usize>() % self.num_query_per_transaction + 1,
                    self.random.gen::<u64>() % self.query_range + 1,
                )),
                Action::DeleteCustomer => Box::new(DeleteCustomer::new(
                    self.random.gen::<u64>() % self.query_range + 1,
                )),
                Action::UpdateTables => Box::new(UpdateTables::new(
                    self.random.clone(),
                    self.query_range,
                    self.random.gen::<usize>() % self.num_query_per_transaction + 1,
                )),
            })
        } else {
            // done
            None
        }
    }
}

/// Program abstraction

trait Program {
    /// Initialization
    fn prepare_initial_query(&mut self) -> Vec<Query>;

    /// Typical client event dispatch
    fn handle_response(&mut self, req: Query, resp: Response) -> Option<Query>;
}

/// Possible programs:

struct MkReservation<T: RngCore + SeedableRng> {
    random: T,
    query_range: u64,
    max_prices: Vec<Option<u64>>,
    max_ids: Vec<Option<u64>>,
    num_queries: usize,
    query_id: usize,
    customer_id: u64,
}

impl<T: RngCore + SeedableRng> MkReservation<T> {
    fn new(random: T, query_range: u64, num_queries: usize, customer_id: u64) -> Self {
        MkReservation {
            random,
            query_range,
            max_prices: vec![None, None, None],
            max_ids: vec![None, None, None],
            num_queries,
            query_id: 0,
            customer_id,
        }
    }
    fn prepare_capacity_queries(&mut self) -> Vec<Query> {
        let mut queries = Vec::with_capacity(self.num_queries);

        for _ in 0..self.num_queries {
            let t = self.random.gen::<ReservationType>();
            let id = (self.random.gen::<u64>() % self.query_range) + 1;
            queries.push(Query::GetCapacity(t, id));
        }

        // just a formality
        self.query_id = self.num_queries;

        queries
    }
}

impl<T: RngCore + SeedableRng> Program for MkReservation<T> {
    fn prepare_initial_query(&mut self) -> Vec<Query> {
        self.prepare_capacity_queries()
    }

    // typical client event dispatch
    fn handle_response(&mut self, req: Query, resp: Response) -> Option<Query> {
        match req {
            Query::GetCapacity(t, id) => match resp {
                // Note this query does not make any sense.
                // Normally one would directly query for price!
                Response::Capacity(Some(_)) => Some(Query::GetPrice(t, id)),
                _ => panic!("Communication logic inconsistency."),
            },
            Query::GetPrice(t, id) => match resp {
                Response::Price(price) => {
                    let idx = t as usize;
                    if price > self.max_prices[idx] {
                        self.max_prices[idx] = price;
                        self.max_ids[idx] = Some(id);
                    } else {
                        // nothing
                    }

                    if self.query_id < self.num_queries {
                        panic!("cannot possibly reach self.query_id < self.num_queries when handling an update response");
                        // continue to issue capacity queries
                        //self.query_id += 1;
                        //Some(self.prepare_capacity_query())
                    } else {
                        // we are done with the capacity queries.
                        // do the reservation

                        // create the customer first
                        Some(Query::Insert(self.customer_id))
                    }
                }
                _ => panic!("Communication logic inconsistency."),
            },
            Query::Insert(customer_id) => match self.max_ids[ReservationType::Car as usize] {
                Some(id) => Some(Query::Reserve(ReservationType::Car, customer_id, id)),
                _ => None, //panic!("Impossible: we never issued any read query."),
            },
            Query::Reserve(t, customer_id, _) =>
            // note: we do not care about the result of the reservation.
            // neither did the original code.
            {
                match t {
                    ReservationType::Car => match self.max_ids[ReservationType::Flight as usize] {
                        Some(id) => Some(Query::Reserve(ReservationType::Flight, customer_id, id)),
                        _ => None, //panic!("Impossible: we never issued any read query."),
                    },
                    ReservationType::Flight => match self.max_ids[ReservationType::Flight as usize]
                    {
                        Some(id) => Some(Query::Reserve(ReservationType::Room, customer_id, id)),
                        _ => None, //panic!("Impossible: we never issued any read query."),
                    },
                    ReservationType::Room => {
                        // done
                        None
                    }
                }
            }
            _ => panic!("Unexpected query: inconsistent program flow."),
        }
    }
}

struct DeleteCustomer {
    customer_id: u64,
}

impl DeleteCustomer {
    fn new(customer_id: u64) -> Self {
        DeleteCustomer { customer_id }
    }
}

impl Program for DeleteCustomer {
    fn prepare_initial_query(&mut self) -> Vec<Query> {
        vec![Query::GetBill(self.customer_id)]
    }

    fn handle_response(&mut self, req: Query, resp: Response) -> Option<Query> {
        match req {
            Query::GetBill(customer_id) => match resp {
                Response::Bill(obill) => {
                    if obill.is_some() {
                        // stiff the check
                        Some(Query::Delete(customer_id))
                    } else {
                        // customer did not exist
                        None
                    }
                }
                _ => panic!("Impossible: we never issued any other query than GetBill."),
            },
            _ => None, // done
        }
    }
}

struct UpdateTables<T: RngCore + SeedableRng> {
    random: T,
    query_range: u64,
    num_updates: usize,
    update_id: usize,
}

impl<T: RngCore + SeedableRng> UpdateTables<T> {
    fn new(random: T, query_range: u64, num_updates: usize) -> Self {
        UpdateTables {
            random,
            query_range,
            num_updates,
            update_id: 0,
        }
    }

    fn prepare_update_queries(&mut self) -> Vec<Query> {
        let mut update_queries = Vec::with_capacity(self.num_updates);

        for _ in 0..self.num_updates {
            let t = self.random.gen::<ReservationType>();
            let id = (self.random.gen::<u64>() % self.query_range) + 1;
            let tmp = self.random.gen::<bool>();
            let new_price0 = if tmp {
                Some(((self.random.gen::<u64>() % 5) * 10) + 50)
            } else {
                None
            };
            match new_price0 {
                Some(new_price) => update_queries.push(Query::AddPrice(t, id, 100, new_price)),
                None => update_queries.push(Query::DeleteCapacity(t, id, 100)),
            }
        }

        self.update_id = self.num_updates;

        update_queries
    }
}

impl<T: RngCore + SeedableRng> Program for UpdateTables<T> {
    fn prepare_initial_query(&mut self) -> Vec<Query> {
        self.prepare_update_queries()
    }

    fn handle_response(&mut self, _req: Query, _resp: Response) -> Option<Query> {
        // Note, the original code again just did not care about the response.
        if self.update_id < self.num_updates {
            panic!("cannot possibly reach self.update_id < self.num_updates when handling an update response");
            //self.update_id += 1;
            //Some(self.prepare_update_query())
        } else {
            // done
            None
        }
    }
}

/*

// Algo to parallelize:
// Note, this algo penalizes clients whose queries were successfull.
// We use the YCSB benchmark to show how those can be responded to and the failed ones are merged
// with the next set of queries. The failed queries being at the front of the ones worked.
// This requires showing latency metrics!

fn server(db, queries) {
    let resps = Responses::new();
    serve(db, queries, resps)
}

fn serve(db, queries, resps) {
   let shared = Arc::new(db);
   //let resps = Responses::new();
   for query in queries {
       let db_ro = shared.clone();
       let update = compute_update(db_ro, query);
       let response = db.apply(update);
       resps.push(response);
   }
   let failed = resps.get_failed();
   let result = (db, resps);
   if failed.is_some() {
       server(db, failed, resps)
   } else {
       result
   }
}

*/

/// Just don't parallelize the client-side. as it makes no sense!
/// The whole point of the benchmark is the parallelism in the server, not the client!

/// Issues the request directly against the database.
#[allow(unused_variables, unused_mut)]
pub fn run_client<T: 'static + RngCore + SeedableRng + Clone>(
    mut client: Client<T>,
    mut db: db::Database,
) {
    let mut cprogram = client.next_program();
    while let Some(mut program) = cprogram {
        let mut cquery = program.prepare_initial_query();
        // FIXME: Theoretically, pop from the front
        while let Some(query) = cquery.pop() {
            let response = db.issue(query.clone());
            cquery.extend(program.handle_response(query, response).into_iter());
        }

        cprogram = client.next_program();
    }
}

/// Computes one request from each of the clients and then submits this batch to the database.
pub fn run_clients<T: 'static + RngCore + SeedableRng + Clone + Send>(
    clients: Vec<Client<T>>,
    db: db::Database,
    //serve: server::Server
) {
    let mut handles = Vec::new();

    for c in clients {
        let d = db.clone();
        handles.push(std::thread::spawn(move || run_client(c, d)));
    }

    handles.into_iter().map(std::thread::JoinHandle::join).for_each(Result::unwrap)
    //let mut cdb = db;

    //// create the initial batch of requests
    //let mut cpq = Vec::with_capacity(clients.len());
    //for mut client in clients.into_iter() {
        //let cprogram = client.next_program();
        //if let Some(mut program) = cprogram {
            //let queries = program.prepare_initial_query();
            //cpq.push((client, program, queries));
        //}
    //}

    //// loop until all clients are done
    //while !cpq.is_empty() {
        //// process the batch
        //let batch = cpq.iter().map(|(_, _, q)| q.clone()).flatten().collect();
        //let (dbp, mut responses) = serve(cdb, batch);
        //cdb = dbp;

        //// handle the responses
        //let mut cpq_p = Vec::new();
        ////for ((mut client, mut program, queries), response) in cpq.drain(..).zip(responses) {
        //for (mut client, mut program, queries) in cpq {
            //let mut local_queries = Vec::new();
            //let bound = queries.len();

            //for (query, response) in queries.into_iter().zip(responses.drain(..bound)) {
                //let nq = program.handle_response(query, response);
                //if let Some(q) = nq {
                    //local_queries.push(q);
                //}
            //}

            //if !local_queries.is_empty() {
                //cpq_p.push((client, program, local_queries));
            //} else {
                //let np = client.next_program();
                //if let Some(mut p) = np {
                    //let q = p.prepare_initial_query();
                    //cpq_p.push((client, p, q));
                //} else {
                    //// client is done
                //}
            //}
        //}
        //cpq = cpq_p;
    //}

    //cdb
}

/* =============================================================================
 * select_action
 * =============================================================================
 */
fn select_action(r: i64, percent_user: i64) -> Action {
    if r < percent_user {
        Action::MakeReservation
    } else if r & 1 != 0 {
        // FIXME check this again. the original code was just `if r&1 {`.
        Action::DeleteCustomer
    } else {
        Action::UpdateTables
    }
}
