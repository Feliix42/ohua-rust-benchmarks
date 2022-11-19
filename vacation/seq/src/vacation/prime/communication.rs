use crate::vacation::reservation::ReservationType;

type CustomerID = u64;
type AssetID = u64;

enum Query {
    GetCapacity(ReservationType, AssetID),
    GetPrice(ReservationType, AssetID),
    AddPrice(ReservationType, AssetID, u64, u64),
    Delete(ReservationType, AssetID, u64),

    Insert(CustomerID),
    Delete(CustomerID),
    GetBill(CustomerID),

    Reserve(ReservationType, CustomerID, AssetID)
}

enum Response {
    Capacity(Option<u64>),
    Price(Option<u64>),
    Bill(Option<u64>),
    Success(bool),
}


struct Server {
    db: Manager
}

impl Server{
    fn issue(self, query: Query) -> Response {
        match query {
            Query::GetCapacity(t, asset_id) => match t {
                ReservationType::Car => self.db.query_car(asset_id),
                ReservationType::Flight => self.db.query_flight(asset_id),
                ReservationType::Room => self.db.query_room(asset_id),
            },
            Query::GetPrice(t, asset_id) => match t {
                ReservationType::Car => self.db.query_car_price(asset_id),
                ReservationType::Flight => self.db.query_flight_price(asset_id),
                ReservationType::Room => self.db.query_room_price(asset_id),
            },
            Query::AddPrice(t, asset_id, a, b) => match t {
                ReservationType::Car => self.db.add_car(asset_id, a, b),
                ReservationType::Flight => self.db.add_flight(asset_id, a, b),
                ReservationType::Room => self.db.add_room(asset_id, a, b),
            },
            Query::Delete(t, asset_id, a) => match t {
                ReservationType::Car => self.db.delete_car(asset_id, a),
                ReservationType::Flight => self.db.delete_flight(asset_id, a),
                ReservationType::Room => self.db.delete_room(asset_id, a),
            },
            Query::Insert(customer_id) => self.db.add_customer(customer_id),
            Query::Delete(customer_id) => self.db.delete_customer(customer_id),
            Query::GetBill(customer_id) => self.db.query_customer_bill(customer_id),
            Query::Reserve(t, customer_id, asset_id) => match t {
                ReservationType::Car => self.db.reserve_car(customer_id, asset_id),
                ReservationType::Flight => self.db.reserve_flight(customer_id, asset_id),
                ReservationType::Room => self.db.reserve_room(customer_id, asset_id),
            },        }
    }
}
