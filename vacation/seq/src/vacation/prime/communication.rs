use crate::vacation::reservation::ReservationType;

type CustomerID = u64;
type AssetID = u64;

#[derive(Clone)]
pub(crate) enum Query {
    GetCapacity(ReservationType, AssetID),
    GetPrice(ReservationType, AssetID),
    AddPrice(ReservationType, AssetID, u64, u64),
    DeleteCapacity(ReservationType, AssetID, u64),

    Insert(CustomerID),
    Delete(CustomerID),
    GetBill(CustomerID),

    Reserve(ReservationType, CustomerID, AssetID)
}

#[derive(Clone)]
pub(crate) enum Response {
    Capacity(Option<u64>),
    Price(Option<u64>),
    Bill(Option<u64>),
    Success(bool),
}


impl Query {
    pub(crate) fn is_read(&self) -> bool {
        match self {
            Query::AddPrice(_,_,_,_) |
            Query::DeleteCapacity(_,_,_) |
            Query::Insert(_) |
            Query::Delete(_) |
            Query::Reserve(_,_,_) => false,
            _ => true
        }
    }
}


