use crate::vacation::action::Action;
use crate::vacation::prime::manager::{Admin, Manager, QueryInterface, ReservationInterface};
use crate::vacation::reservation::ReservationType;
use rand::{Rng, RngCore, SeedableRng};
use crate::vacation::prime::communication::{Query, Response};

pub struct Client<T: RngCore + SeedableRng> {
    random: T,
    num_operation: usize,
    num_query_per_transaction: usize,
    query_range: u64,
    percent_user: i64,

    // state
    op: usize,
}


impl Client {

    fn next_program() -> Option<Program> {
        if self.op < self.num_operation {
            let r = self.random.gen::<i64>() % 100;
            let action = select_action(r, self.percent_user);
            match action {
                Action::MakeReservation =>
                    MkReservation::new(
                        self.random.gen::<usize>() % self.num_query_per_transaction + 1,
                        self.random.gen::<u64>() % self.query_range + 1
                    ),
                Action::DeleteCustomer =>
                    DeleteCustomer::new(
                        self.random.gen::<u64>() % self.query_range + 1
                    ),
                Action::UpdateTables =>
                    UpdatesTables::new(
                        self.random.gen::<usize>() % self.num_query_per_transaction + 1
                    )
            }
        } else {
            // done
            Nothing
        }
    }
}


/// Program abstraction

trait Program {
    /// Initialization
    fn prepare_initial_query(&mut self) -> Query;

    /// Typical client event dispatch
    fn handle_response(&mut self, req: Query, resp: Response) -> Option<Query>;

/// Possible programs:

struct MkReservation {
    max_prices: Vec<_>,
    max_ids: Vec<_>,
    num_queries: usize,
    query_id: usize,
    customer_id: u64,
}

impl MkReservation {
    fn new(num_query: usize, customer_id: u64) -> Self {
        MkReservation{
            max_prices : vec![None, None, None],
            max_ids : vec![None, None, None],
            num_query, // TODO self.random.gen::<usize>() % self.num_query_per_transaction + 1,
            query_id : 0,
            customer_id, // TODO  self.random.gen::<u64>() % self.query_range + 1,
        }
    }

    fn prepare_initial_query(&mut self) -> Query {
        self.prepare_capacity_query()
    }

    fn prepare_capacity_query(&mut self) -> Query {
        let t = self.random.gen::<ReservationType>();
        let id = (self.random.gen::<u64>() % self.query_range) + 1;
        Query::GetCapacity(t, id)
    }

    // typical client event dispatch
    fn handle_response(&mut self, req: Query, resp: Response) -> Option<Query> {
        match req {
            Query::GetCapacity(t, id) =>
                match resp {
                    // Note this query does not make any sense.
                    // Normally one would directly query for price!
                    Capacity(Some(_)) => {
                        Some(Query::GetPrice(t,id))
                    },
                    _ => panic!("Communication logic inconsistency.")
                },
            Query::GetPrice(t, id) =>
                match resp {
                    Price(price) => {
                        let idx = t as usize;
                        if price > max_prices[idx] {
                            self.max_prices[idx] = price;
                            self.max_ids[idx] = Some(id);
                        } else {
                            // nothing
                        }


                        if self.query_id < self.num_queries {
                            // continue to issue capacity queries
                            self.query_id += 1;
                            self.query = self.prepare_capacity_query();
                        } else {
                            // we are done with the capacity queries.
                            // do the reservation

                            // create the customer first
                            self.query = Query::Insert(self.customer_id);
                        }
                    },
                    _ => panic!("Communication logic inconsistency.")
                },
            Query::Insert(customer_id) => {
                match self.max_ids[ReservationType::Car as usize] {
                    Some(id) => {
                        Some(Query::Reserve(ReservationType::Car, customer_id, id))
                    }
                    _ => panic!("Impossible: we never issued any read query."),
                }
            },
            Query::Reserve(t,customer_id,_) =>
            // note: we do not care about the result of the reservation.
            // neither did the original code.
                match t {
                    ReservationType::Car =>
                        match self.max_ids[ReservationType::Flight as usize] {
                            Some(id) => {
                                Some(Query::Reserve(ReservationType::Flight, customer_id, id))
                            }
                            _ => panic!("Impossible: we never issued any read query."),
                        },
                    ReservationType::Flight =>
                        match self.max_ids[ReservationType::Flight as usize] {
                            Some(id) => {
                                Some(Query::Reserve(ReservationType::Room, customer_id, id))
                            }
                            _ => panic!("Impossible: we never issued any read query."),
                        },
                    ReservationType::Room => {
                        // done
                        Nothing
                    }
                },
            _ => panic!("Unexpected query: inconsistent program flow.")
        }
    }
}

struct DeleteCustomer {
    customer_id: u64
}

impl DeleteCustomer {

    fn new(customer_id: u64) -> Self {
        DeleteCustomer{ customer_id }
    }

    fn prepare_initial_query(&self) {
        Query::GetBill(self.customer_id)
    }

    fn handle_response(&mut self, req: Query, resp: Response) -> Option<Query> {
        match req {
            Query::GetBill(customer_id) =>
                match resp {
                    Bill(oBill) =>
                        if oBill.is_some() {
                            // stiff the check
                            Some(Query::Delete(customer_id))
                        } else {
                            // customer did not exist
                            Nothing
                        }
                },
            _ => Nothing // done
        }
    }
}

struct UpdateTables {
    num_update: usize,
    update_id: usize
}


impl UpdateTables {

    fn new(num_updates: usize) -> Self {
        UpdateTables { num_updates, update_id : 0 }
    }

    fn prepare_initial_query(&self) -> Query {
        self.prepare_update_query()
    }

    fn prepare_update_query(&self) -> Query {
        let t = self.random.gen::<ReservationType>();
        let id = (self.random.gen::<u64>() % self.query_range) + 1;
        let tmp = self.random.gen::<bool>();
        let new_price0 = if tmp {
            Some(((self.random.gen::<u64>() % 5) * 10) + 50)
        } else {
            None
        };
        match new_price0 {
            Some(new_price) => Query::AddPrice(t, id, 100, new_price),
            Nothing => Query::Delete(t, id, 100)
        }
    }

    fn handle_response(&mut self, req: Query, _resp: Response) -> Option<Query> {
        // Note, the original code again just did not care about the response.
        if self.update_id < self.num_update {
            self.update_id += 1;
            Some(self.prepare_update_query())
        } else {
            // done
            Nothing
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

// Just don't parallelize this. as it makes no sense!
// The whole point of the benchmark is the parallelism in the server, not the client!
fn run(client: Client, db: Database) {

    let mut cprogram = client.next_program();
    while let Some(program) = cprogram {
        let mut cquery = Some(program.prepare_initial_query());
        while let Some(query) = current {
            let (db', responses) = server(db, query.clone());
            current = client.handle_response(query, response);
        }
        cprogram = client.next_program();
    }
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
