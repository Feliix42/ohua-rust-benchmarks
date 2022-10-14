use crate::vacation::customer::Customer;
use crate::vacation::reservation::{Reservation, ReservationType, TotalUpdate};
use std::collections::HashMap;

// The benchmark implements tables as hashmaps.
// That is reasonable for point access but any range read
// will be a big performance burden.
struct Manager {
    carTable: HashMap<u64, Reservation>,
    roomTable: HashMap<u64, Reservation>,
    flightTable: HashMap<u64, Reservation>,
    customerTable: HashMap<u64, Customer>, // a customer should at least have a name etc.
}

fn upsert_reservation(
    table: &mut HashMap<u64, Reservation>,
    id: u64,
    num: TotalUpdate,
    price: Option<u64>,
) -> bool {
    match price {
        None => false,
        Some(p) => {
            let (changed, remove) = match table.get_mut(&id) {
                None =>
                /* Create new reservation */
                {
                    match num {
                        TotalUpdate::Subtract(_) => (false, false),
                        TotalUpdate::Add(n) => {
                            table.insert(id, Reservation::new(id, n, p));
                            (true, false)
                        }
                    }
                }
                Some(mut res) => {
                    /* Update existing reservation */
                    let num_total = res.update_total(num);

                    if num_total == 0 {
                        (true, true)
                    } else {
                        res.update_price(p);
                        (true, false)
                    }
                }
            };
            if remove {
                table.remove(&id);
                changed
            } else {
                changed
            }
        }
    }
}

impl Manager {
    /* =============================================================================
     * manager_addCar
     * -- Add cars to a city
     * -- Adding to an existing car overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */

    fn addCar(&mut self, carId: u64, numCars: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.carTable,
            carId,
            TotalUpdate::Add(numCars),
            Some(price),
        )
    }

    /* =============================================================================
     * manager_deleteCar
     * -- Delete cars from a city
     * -- Decreases available car count (those not allocated to a customer)
     * -- Fails if would make available car count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteCar(&mut self, carId: u64, numCar: u64) -> bool {
        /* None keeps old price */
        upsert_reservation(
            &mut self.carTable,
            carId,
            TotalUpdate::Subtract(numCar),
            None,
        )
    }

    /* =============================================================================
     * manager_addRoom
     * -- Add rooms to a city
     * -- Adding to an existing room overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn addRoom(&mut self, roomId: u64, numRoom: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.roomTable,
            roomId,
            TotalUpdate::Add(numRoom),
            Some(price),
        )
    }

    /* =============================================================================
     * manager_deleteRoom
     * -- Delete rooms from a city
     * -- Decreases available room count (those not allocated to a customer)
     * -- Fails if would make available room count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteRoom(&mut self, roomId: u64, numRoom: u64) -> bool {
        /* None keeps old price */
        upsert_reservation(
            &mut self.roomTable,
            roomId,
            TotalUpdate::Subtract(numRoom),
            None,
        )
    }

    /* =============================================================================
     * manager_addFlight
     * -- Add seats to a flight
     * -- Adding to an existing flight overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, FALSE on failure
     * =============================================================================
     */
    fn addFlight(&mut self, flightId: u64, numSeat: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.flightTable,
            flightId,
            TotalUpdate::Add(numSeat),
            Some(price)
        )
    }

    /* =============================================================================
     * manager_deleteFlight
     * -- Delete an entire flight
     * -- Fails if customer has reservation on this flight
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteFlight(&mut self, flightId: u64) -> bool {
        let reservation = self.flightTable.get(&flightId);
        let delete = match reservation {
            None => None,
            Some(res) => {
                if res.num_used > 0 {
                    None
                } else {
                    Some(res.num_total)
                }
            }
        };
        match delete {
            Some(numTotal) => upsert_reservation(
                &mut self.flightTable,
                flightId,
                TotalUpdate::Subtract(numTotal),
                None,
            ),
            None => false,
        }
    }

    /* =============================================================================
     * manager_addCustomer
     * -- If customer already exists, returns failure
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn addCustomer(&mut self, customerId: u64) -> bool {
        let customer_contained = self.customerTable.contains_key(&customerId);
        if customer_contained {
            false
        } else {
            self.customerTable.insert(customerId, Customer::new(customerId));
            true
        }
    }

    /* =============================================================================
     * manager_deleteCustomer
     * -- Delete this customer and associated reservations
     * -- If customer does not exist, returns success
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteCustomer(&mut self, customerId: u64) -> bool {
        let customer = self.customerTable.remove(&customerId);
        match customer {
            None => false,
            Some(cus) => {
                /* Cancel this customer's reservations */
                for reservation in cus.reservation_info_list {
                    let tbl = match reservation.typ {
                        ReservationType::Car => &mut self.carTable,
                        ReservationType::Flight => &mut self.flightTable,
                        ReservationType::Room => &mut self.roomTable,
                    };
                    let mut e = tbl.get_mut(&reservation.id);
                    e.map(|e0| e0.cancel());
                }
                true
            }
        }
    }
}
