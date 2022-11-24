use crate::vacation::manager::{Admin, Manager, QueryInterface, ReservationInterface};
use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::reservation::ReservationType;
use std::sync::Arc;
use stm::atomically;

#[derive(Clone)]
pub struct Database {
    mngr: Arc<Manager>,
}

impl Database {
    pub fn new(mngr: Manager) -> Database {
        Database { mngr: Arc::new(mngr) }
    }

    pub(crate) fn issue(&self, q: Query) -> Response {
        if q.is_read() {
            self.issue_read(q)
        } else {
            self.issue_write(q)
        }
    }

    pub(crate) fn issue_write(&self, query: Query) -> Response {
        atomically(|trans| {
            match query {
                Query::AddPrice(ref t, asset_id, a, b) => Ok(Response::Success(match t {
                    ReservationType::Car => self.mngr.add_car(asset_id, a, b, trans)?,
                    ReservationType::Flight => self.mngr.add_flight(asset_id, a, b, trans)?,
                    ReservationType::Room => self.mngr.add_room(asset_id, a, b, trans)?,
                })),
                Query::DeleteCapacity(ref t, asset_id, a) => Ok(Response::Success(match t {
                    ReservationType::Car => self.mngr.delete_car(asset_id, a, trans)?,
                    ReservationType::Flight => self.mngr.delete_flight(asset_id, trans)?,
                    ReservationType::Room => self.mngr.delete_room(asset_id, a, trans)?,
                })),
                Query::Insert(customer_id) => Ok(Response::Success(self.mngr.add_customer(customer_id, trans)?)),
                Query::Delete(customer_id) => Ok(Response::Success(self.mngr.delete_customer(customer_id, trans)?)),
                Query::Reserve(ref t, customer_id, asset_id) => Ok(Response::Success(match t {
                    ReservationType::Car => self.mngr.reserve_car(customer_id, asset_id, trans)?,
                    ReservationType::Flight => self.mngr.reserve_flight(customer_id, asset_id, trans)?,
                    ReservationType::Room => self.mngr.reserve_room(customer_id, asset_id, trans)?,
                })),
                _ => panic!("Invariant broken: received read for doing a write"),
            }
        })
    }

    pub(crate) fn issue_read(&self, query: Query) -> Response {
        // FIXME(feliix42): This could be atomic reads? Or is there any type of assurance that
        // there won't be an overbooking? My concern:
        // - this now is broken apart into different tx. Before, the whole booking process was 1
        // transaction and that by design.
        // - what happens when I reserve a car and then a hotel, but the latter fails? Is there a
        // semantic correctness guarantee?
        atomically(|trans| {
            match query {
                Query::GetCapacity(ref t, asset_id) => Ok(Response::Capacity(match t {
                    ReservationType::Car => self.mngr.query_car(asset_id, trans)?,
                    ReservationType::Flight => self.mngr.query_flight(asset_id, trans)?,
                    ReservationType::Room => self.mngr.query_room(asset_id, trans)?,
                })),
                Query::GetPrice(ref t, asset_id) => Ok(Response::Price(match t {
                    ReservationType::Car => self.mngr.query_car_price(asset_id, trans)?,
                    ReservationType::Flight => self.mngr.query_flight_price(asset_id, trans)?,
                    ReservationType::Room => self.mngr.query_room_price(asset_id, trans)?,
                })),
                Query::GetBill(customer_id) => {
                    Ok(Response::Bill(self.mngr.query_customer_bill(customer_id, trans)?))
                }
                _ => panic!("Invariant broken: received write for doing a read"),
            }
        })
    }
}

/// This call consumes the Arc.
/// The Arc is dropped as soon as the call is done!
pub(crate) fn issue_read(db: Arc<Database>, q: Query) -> Response {
    db.issue_read(q)
}

pub(crate) fn seq_arc_unwrap<S, T>(a: Arc<S>, x: T) -> (S, T) {
    match Arc::<S>::try_unwrap(a) {
        Ok(ap) => (ap,x),
        _ => panic!("Failed to unwrap the Arc. Please make sure that the construction of `x` has destructed all previous Arcs.")
    }
}

pub(crate) fn split(batch: Vec<Query>) -> (Vec<Query>, Vec<Query>, Vec<Response>) {
    let mut reads = Vec::new();
    let mut writes = Vec::new();
    let responses = Vec::with_capacity(batch.len());

    for q in batch {
        if q.is_read() {
            reads.push(q);
        } else {
            writes.push(q);
        }
    }

    (reads, writes, responses)
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
            Query::GetCapacity(t0, asset_id0) => {
                for write in writes {
                    match &write.query {
                        Query::DeleteCapacity(t1, asset_id1, _)
                            if t0 == t1 && asset_id0 == asset_id1 =>
                        {
                            return true
                        }
                        Query::Reserve(t1, _, asset_id1) if t0 == t1 && asset_id0 == asset_id1 => {
                            return true
                        }
                        _ => (),
                    }
                }
            }
            Query::GetPrice(t0, asset_id0) => {
                for write in writes {
                    match &write.query {
                        Query::AddPrice(t1, asset_id1, _, _) if t0 == t1 && asset_id0 == asset_id1 => {
                            return true
                        }
                        Query::DeleteCapacity(t1, asset_id1, _)
                            if t0 == t1 && asset_id0 == asset_id1 =>
                        {
                            return true
                        }
                        _ => (),
                    }
                }
            }
            Query::GetBill(customer_id0) => {
                for write in writes {
                    match &write.query {
                        Query::Reserve(_, customer_id1, _) if customer_id0 == customer_id1 => {
                            return true
                        }
                        Query::Insert(customer_id1) if customer_id0 == customer_id1 => return true,
                        Query::Delete(customer_id1) if customer_id0 == customer_id1 => return true,
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

