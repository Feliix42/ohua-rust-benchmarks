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


