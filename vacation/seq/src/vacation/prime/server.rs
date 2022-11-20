use crate::vacation::prime::manager::Manager;
use crate::vacation::reservation::ReservationType;
use crate::vacation::prime::communication::{Query, Response};


pub(crate) fn issue(db: &mut Manager, query: Query) -> Response {
     match query {
         Query::GetCapacity(t, asset_id) => match t {
             ReservationType::Car => db.query_car(asset_id),
             ReservationType::Flight => db.query_flight(asset_id),
             ReservationType::Room => db.query_room(asset_id),
         },
         Query::GetPrice(t, asset_id) => match t {
             ReservationType::Car => db.query_car_price(asset_id),
             ReservationType::Flight => db.query_flight_price(asset_id),
             ReservationType::Room => db.query_room_price(asset_id),
         },
         Query::AddPrice(t, asset_id, a, b) => match t {
             ReservationType::Car => db.add_car(asset_id, a, b),
             ReservationType::Flight => db.add_flight(asset_id, a, b),
             ReservationType::Room => db.add_room(asset_id, a, b),
         },
         Query::DeleteCapacity(t, asset_id, a) => match t {
             ReservationType::Car => db.delete_car(asset_id, a),
             ReservationType::Flight => db.delete_flight(asset_id, a),
             ReservationType::Room => db.delete_room(asset_id, a),
         },
         Query::Insert(customer_id) => db.add_customer(customer_id),
         Query::Delete(customer_id) => db.delete_customer(customer_id),
         Query::GetBill(customer_id) => db.query_customer_bill(customer_id),
         Query::Reserve(t, customer_id, asset_id) => match t {
             ReservationType::Car => db.reserve_car(customer_id, asset_id),
             ReservationType::Flight => db.reserve_flight(customer_id, asset_id),
             ReservationType::Room => db.reserve_room(customer_id, asset_id),
         },
     }
 }


pub(crate) fn split(batch: Vec<Query>) -> (Vec<Query>,Vec<Query>) {
    let mut reads = Vec::new();
    let mut writes = Vec::new();

    for q in batch {
        match q {
            Query::AddPrice(_,_,_,_) |
            Query::DeleteCapacity(_,_,_) |
            Query::Insert(_) |
            Query::Delete(_) |
            Query::Reserve(_,_,_) => writes.push(q),
            _ => reads.push(q)
        }
    }

    (reads, writes)
}

struct Database {
    mngr: Manager
}

impl Database {
    fn new(mngr:Manager) -> Database {
        Database { mngr }
    }

    fn issue_write(&mut self, q:Query) -> Response {
        match query {
            Query::AddPrice(t, asset_id, a, b) =>
                Response::Price(
                    match t {
                        ReservationType::Car => db.add_car(asset_id, a, b),
                        ReservationType::Flight => db.add_flight(asset_id, a, b),
                        ReservationType::Room => db.add_room(asset_id, a, b),
                    }),
            Query::DeleteCapacity(t, asset_id, a) =>
                Response::Capacity(
                    match t {
                        ReservationType::Car => db.delete_car(asset_id, a),
                        ReservationType::Flight => db.delete_flight(asset_id, a),
                        ReservationType::Room => db.delete_room(asset_id, a),
                    }),
            Query::Insert(customer_id) =>
                Response::Success(db.add_customer(customer_id)),
            Query::Delete(customer_id) =>
                Response::Success(db.delete_customer(customer_id)),
            Query::Reserve(t, customer_id, asset_id) =>
                Response::Success(
                    match t {
                        ReservationType::Car => db.reserve_car(customer_id, asset_id),
                        ReservationType::Flight => db.reserve_flight(customer_id, asset_id),
                        ReservationType::Room => db.reserve_room(customer_id, asset_id),
                    }),
            _ => panic!("Invariant broken: received read for doing a write")
        }
    }

    fn issue_read(&self, q:Query) -> Response {
        match query {
            Query::GetCapacity(t, asset_id) =>
                Response::Capacity(
                    match t {
                        ReservationType::Car => db.query_car(asset_id),
                        ReservationType::Flight => db.query_flight(asset_id),
                        ReservationType::Room => db.query_room(asset_id),
                    }),
            Query::GetPrice(t, asset_id) =>
                Response::Price(
                    match t {
                        ReservationType::Car => db.query_car_price(asset_id),
                        ReservationType::Flight => db.query_flight_price(asset_id),
                        ReservationType::Room => db.query_room_price(asset_id),
                    }),
            Query::GetBill(customer_id) =>
                Response::Bill(db.query_customer_bill(customer_id)),
            _ => panic!("Invariant broken: received write for doing a read")
        }
    }
}

fn issue_read(db:Arc<Database>, q:Query) -> Response {
   db.issue_read(q)
}

/// This server algorithm uses batching and performs reordering of queries.
/// It applies writes before reads, so reads see the most up-to-date data.
pub(crate) fn server_writes_reads(db: Database, batch: Vec<Query>) -> (Manager,Vec<Response>) {
    let (reads, writes) = split(batch);
    let mut responses = Vec::with_capacity(batch.len());
    for write in writes {
        let resp = db.issue_write(write);
        responses.push(resp);
    }

    let shared = Arc::new(db);
    for read in reads {
        let own_db = shared.clone();
        let resp = issue_read(own_db, read);
        responses.push(resp);
    }

    (db, responses)
}

// Note that the nature of transactions changed now:
// Before the benchmark was actually spanning transactions across client calls.
// Now this is not possible anymore!
// The developer now has to handle overwrites and a client needs to restart its computation again.
// Effectively, the database system needs to either be extended to transactions or the
// client has to cope with consistency on the client side, for example via eventual consistency.
