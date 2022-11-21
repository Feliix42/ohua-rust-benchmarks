use crate::vacation::manager::{Admin, Manager, QueryInterface, ReservationInterface};
use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::reservation::ReservationType;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    mngr: Manager,
}

impl Database {
    pub fn new(mngr: Manager) -> Database {
        Database { mngr }
    }

    pub(crate) fn issue(&mut self, q: Query) -> Response {
        if q.is_read() {
            self.issue_read(q)
        } else {
            self.issue_write(q)
        }
    }

    pub(crate) fn issue_write(&mut self, query: Query) -> Response {
        match query {
            Query::AddPrice(t, asset_id, a, b) => Response::Success(match t {
                ReservationType::Car => self.mngr.add_car(asset_id, a, b),
                ReservationType::Flight => self.mngr.add_flight(asset_id, a, b),
                ReservationType::Room => self.mngr.add_room(asset_id, a, b),
            }),
            Query::DeleteCapacity(t, asset_id, a) => Response::Success(match t {
                ReservationType::Car => self.mngr.delete_car(asset_id, a),
                ReservationType::Flight => self.mngr.delete_flight(asset_id),
                ReservationType::Room => self.mngr.delete_room(asset_id, a),
            }),
            Query::Insert(customer_id) => Response::Success(self.mngr.add_customer(customer_id)),
            Query::Delete(customer_id) => Response::Success(self.mngr.delete_customer(customer_id)),
            Query::Reserve(t, customer_id, asset_id) => Response::Success(match t {
                ReservationType::Car => self.mngr.reserve_car(customer_id, asset_id),
                ReservationType::Flight => self.mngr.reserve_flight(customer_id, asset_id),
                ReservationType::Room => self.mngr.reserve_room(customer_id, asset_id),
            }),
            _ => panic!("Invariant broken: received read for doing a write"),
        }
    }

    pub(crate) fn issue_read(&self, query: Query) -> Response {
        match query {
            Query::GetCapacity(t, asset_id) => Response::Capacity(match t {
                ReservationType::Car => self.mngr.query_car(asset_id),
                ReservationType::Flight => self.mngr.query_flight(asset_id),
                ReservationType::Room => self.mngr.query_room(asset_id),
            }),
            Query::GetPrice(t, asset_id) => Response::Price(match t {
                ReservationType::Car => self.mngr.query_car_price(asset_id),
                ReservationType::Flight => self.mngr.query_flight_price(asset_id),
                ReservationType::Room => self.mngr.query_room_price(asset_id),
            }),
            Query::GetBill(customer_id) => {
                Response::Bill(self.mngr.query_customer_bill(customer_id))
            }
            _ => panic!("Invariant broken: received write for doing a read"),
        }
    }
}

pub(crate) fn issue_read(db: &Database, q: Query) -> Response {
    db.issue_read(q)
}

pub(crate) fn split(batch: Vec<Query>) -> (Vec<Query>, Vec<Query>) {
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
#[derive(Clone)]
pub(crate) struct IndexedQuery {
    query: Query,
    idx: usize,
}

impl IndexedQuery {
    fn is_collision(&self, writes: &Vec<IndexedQuery>) -> bool {
        match &self.query {
            Query::GetCapacity(t0, assetId0) => {
                for write in writes {
                    match &write.query {
                        Query::DeleteCapacity(t1, assetId1, _)
                            if t0 == t1 && assetId0 == assetId1 =>
                        {
                            return true
                        }
                        Query::Reserve(t1, _, assetId1) if t0 == t1 && assetId0 == assetId1 => {
                            return true
                        }
                        _ => (),
                    }
                }
            }
            Query::GetPrice(t0, assetId0) => {
                for write in writes {
                    match &write.query {
                        Query::AddPrice(t1, assetId1, _, _) if t0 == t1 && assetId0 == assetId1 => {
                            return true
                        }
                        Query::DeleteCapacity(t1, assetId1, _)
                            if t0 == t1 && assetId0 == assetId1 =>
                        {
                            return true
                        }
                        _ => (),
                    }
                }
            }
            Query::GetBill(customerId0) => {
                for write in writes {
                    match &write.query {
                        Query::Reserve(_, customerId1, _) if customerId0 == customerId1 => {
                            return true
                        }
                        Query::Insert(customerId1) if customerId0 == customerId1 => return true,
                        Query::Delete(customerId1) if customerId0 == customerId1 => return true,
                        _ => (),
                    }
                }
            }
            _ => (), // write query
        }
        false
    }
}

pub(crate) fn index_queries_and_responses(
    batch: Vec<Query>,
) -> (Vec<IndexedQuery>, Vec<Option<Response>>) {
    let mut indexed = Vec::with_capacity(batch.len());
    let mut responses = Vec::with_capacity(batch.len());
    let mut idx = 0;
    for query in batch {
        indexed.push(IndexedQuery { query, idx });
        responses.push(None);
        idx += 1;
    }
    (indexed, responses)
}

pub(crate) fn unwrap_responses(mut responses: Vec<Option<Response>>) -> Vec<Response> {
    responses
        .drain(..)
        .map(|oresp| match oresp {
            Some(resp) => resp,
            None => panic!("Ivariant broken: the server did not process all queries."),
        })
        .collect()
}

pub(crate) fn compute(db: Arc<Database>, query: IndexedQuery) -> (IndexedQuery, Option<Response>) {
    let resp = if query.query.is_read() {
        Some(db.issue_read(query.query.clone()))
    } else {
        None
    };

    (query, resp)
}

pub(crate) trait Delta {
    fn apply_delta(
        &mut self,
        delta: Vec<(IndexedQuery, Option<Response>)>,
    ) -> (Vec<IndexedQuery>, Vec<(usize, Response)>);
}

impl Delta for Database {
    /// This implementation redoes reads that read an old value.
    /// Note that this version is only here for baseline comparison with the old STM version.
    /// It does not make any sense, because there is no notion of consistency for single query transactions!
    /// (The old STM implementation was simulating database transactions with software transactions.)
    fn apply_delta(
        &mut self,
        mut delta: Vec<(IndexedQuery, Option<Response>)>,
    ) -> (Vec<IndexedQuery>, Vec<(usize, Response)>) {
        let mut redos = Vec::new();
        let mut responses = Vec::new();
        let writes = delta
            .iter()
            .filter(|(_, o)| o.is_none())
            .map(|(a, _)| a.clone())
            .collect();
        for (query, resp) in delta.drain(..) {
            match resp {
                Some(r) => {
                    if query.is_collision(&writes) {
                        redos.push(query)
                    } else {
                        responses.push((query.idx, r));
                    }
                }
                None => {
                    assert!(!query.query.is_read());
                    responses.push((query.idx, self.issue_write(query.query)));
                }
            }
        }
        (redos, responses)
    }
}

pub(crate) fn insert_at_index(
    mut responses: Vec<Option<Response>>,
    new: Vec<(usize, Response)>,
) -> Vec<Option<Response>> {
    for (idx, resp) in new {
        responses[idx] = Some(resp);
    }
    responses
}

pub(crate) trait NotEmpty {
    fn not_empty(&self) -> bool;
}

impl<T> NotEmpty for Vec<T> {
    fn not_empty(&self) -> bool {
        !self.is_empty()
    }
}
