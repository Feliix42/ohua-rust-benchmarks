use crate::vacation::prime::manager::Manager;
use crate::vacation::reservation::ReservationType;
use crate::vacation::prime::communication::{Query, Response};


struct Database {
    mngr: Manager
}

impl Database {
    fn new(mngr:Manager) -> Database {
        Database { mngr }
    }

    fn issue(&mut self, q:Query) -> Response {
        if q.is_read() {
            self.issue_read(q)
        } else {
            self.issue_write(q)
        }
    }

    fn issue_write(&mut self, q:Query) -> Response {
        match query {
            Query::AddPrice(t, asset_id, a, b) =>
                Response::Price(
                    match t {
                        ReservationType::Car => self.mngr.add_car(asset_id, a, b),
                        ReservationType::Flight => self.mngr.add_flight(asset_id, a, b),
                        ReservationType::Room => self.mngr.add_room(asset_id, a, b),
                    }),
            Query::DeleteCapacity(t, asset_id, a) =>
                Response::Capacity(
                    match t {
                        ReservationType::Car => self.mngr.delete_car(asset_id, a),
                        ReservationType::Flight => self.mngr.delete_flight(asset_id, a),
                        ReservationType::Room => self.mngr.delete_room(asset_id, a),
                    }),
            Query::Insert(customer_id) =>
                Response::Success(self.mngr.add_customer(customer_id)),
            Query::Delete(customer_id) =>
                Response::Success(self.mngr.delete_customer(customer_id)),
            Query::Reserve(t, customer_id, asset_id) =>
                Response::Success(
                    match t {
                        ReservationType::Car => self.mngr.reserve_car(customer_id, asset_id),
                        ReservationType::Flight => self.mngr.reserve_flight(customer_id, asset_id),
                        ReservationType::Room => self.mngr.reserve_room(customer_id, asset_id),
                    }),
            _ => panic!("Invariant broken: received read for doing a write")
        }
    }

    fn issue_read(&self, q:Query) -> Response {
        match query {
            Query::GetCapacity(t, asset_id) =>
                Response::Capacity(
                    match t {
                        ReservationType::Car => self.mngr.query_car(asset_id),
                        ReservationType::Flight => self.mngr.query_flight(asset_id),
                        ReservationType::Room => self.mngr.query_room(asset_id),
                    }),
            Query::GetPrice(t, asset_id) =>
                Response::Price(
                    match t {
                        ReservationType::Car => self.mngr.query_car_price(asset_id),
                        ReservationType::Flight => self.mngr.query_flight_price(asset_id),
                        ReservationType::Room => self.mngr.query_room_price(asset_id),
                    }),
            Query::GetBill(customer_id) =>
                Response::Bill(self.mngr.query_customer_bill(customer_id)),
            _ => panic!("Invariant broken: received write for doing a read")
        }
    }
}

pub(crate) fn issue_read(db:Arc<Database>, q:Query) -> Response {
   db.issue_read(q)
}

pub(crate) fn split(batch: Vec<Query>) -> (Vec<Query>,Vec<Query>) {
    let mut reads = Vec::new();
    let mut writes = Vec::new();

    for q in batch {
        if q.is_read() {
            reads.push(q);
        } else {
            writes.push(q);
        }
    }

    (reads, writes)
}


/// Code for the naive server version.

struct IndexedQuery {
    query: Query,
    idx: usize
}


pub(crate) fn index_queries(batch: Vec<Query>) -> Vec<IndexedQuery> {
    let mut indexed = Vec::with_capacity(batch.len());
    let mut idx = 0;
    for query in batch {
        indexed.push(IndexedQuery { query, idx } );
        idx += 1;
    }
    indexed
}

pub(crate) fn compute(db:Arc<Database>, query: IndexedQuery) -> (IndexedQuery, Option<Response>) {
    (query,
     if query.query.is_read() {
        Some(db.issue_read(query.query.clone()))
     } else {
        None
     })
}

trait Delta {
    fn apply_delta(&mut self, (query, resp): (IndexedQuery, Option<Response>)) -> Option<Response>;
}

enum Either<S,T> {
    Left(S),
    Right(T)
}

impl Delta for Database {
    /// This implemantation redoes reads that read an old value.
    fn apply_delta(&mut self, (query, resp): (IndexedQuery, Option<Response>)) -> Either<Query, (usize,Response)> {
        // FIXME this needs to be done on the whole result set.
        match resp {
            Some(_) => {
                // TODO implement the collision check!
                (query.idx, resp)
            },
            None => {
                assert!(!query.query.is_read());
                Some(self.issue_write(query.query))
            }
        }
    }
}

pub(crate) fn resolve(
    results: Vec<Either<IndexedQuery, (usize,Response)>>,
    responses: Vec<Response>
) -> (Vec<Response>,Vec<IndexedQuery>) {
    let mut redo = Vec::new();
    for r in results {
        match r {
            Either::Left(q) => redo.push(q),
            Either::Right((idx, resp)) => responses.insert(idx, resp) // TODO check this operation again
        }
    }
    (responses,redo)
}
