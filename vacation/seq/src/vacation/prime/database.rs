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
