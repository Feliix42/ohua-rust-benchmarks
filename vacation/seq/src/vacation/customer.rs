use crate::vacation::reservation::{ReservationInfo, ReservationType};
use std::cmp::Ordering;

#[derive(Clone)]
pub(crate) struct Customer {
    id: u64,
    pub(crate) reservation_info_list: Vec<ReservationInfo>,
}

impl Eq for Customer {}

impl Ord for Customer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Customer {
    pub(crate) fn new(id: u64) -> Self {
        Customer {
            id,
            reservation_info_list: Vec::new(),
        }
    }

    /* =============================================================================
     * customer_addReservationInfo
     * =============================================================================
     */
    pub(crate) fn add_reservation_info(&mut self, typ: ReservationType, id: u64, price: u64) {
        let reservation_info = ReservationInfo::new(typ, id, price);
        let a = self.reservation_info_list.binary_search(&reservation_info);
        match a {
            Ok(idx) => self.reservation_info_list.insert(idx, reservation_info),
            // this insert operation is certainly expensive!
            Err(idx) => self.reservation_info_list.insert(idx, reservation_info),
        };
    }

    /* =============================================================================
     * customer_removeReservationInfo
     * =============================================================================
     */
    pub(crate) fn remove_reservation_info(&mut self, typ: ReservationType, id: u64) -> bool {
        let find_reservation_info = ReservationInfo::new(typ, id, 0);
        /* price not used to compare reservation infos */

        let a = self
            .reservation_info_list
            .binary_search(&find_reservation_info);
        match a {
            Ok(idx) => {
                // again a quite expensive operation
                self.reservation_info_list.remove(idx);
                true
            }
            _ => false,
        }
    }

    /* =============================================================================
     * customer_getBill
     * -- Returns total cost of reservations
     * =============================================================================
     */
    pub(crate) fn get_bill(&self) -> u64 {
        let mut bill = 0;
        for ri in &self.reservation_info_list {
            bill += ri.price;
        }
        bill
    }
}
