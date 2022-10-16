use crate::vacation::customer::Customer;
use crate::vacation::reservation::{Reservation, ReservationType, TotalUpdate};
use std::collections::HashMap;

// The benchmark implements tables as hashmaps.
// That is reasonable for point access but any range read
// will be a big performance burden.
pub(crate) struct Manager {
    carTable: HashMap<u64, Reservation>,
    roomTable: HashMap<u64, Reservation>,
    flightTable: HashMap<u64, Reservation>,
    customerTable: HashMap<u64, Customer>, // a customer should at least have a name etc.
}

impl Manager {
    pub(crate) fn new() -> Self {
        Manager {
            carTable : HashMap::new(),
            roomTable : HashMap::new(),
            flightTable : HashMap::new(),
            customerTable : HashMap::new()
        }
    }
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

pub(crate) trait Admin {
    /* =============================================================================
     * addCar
     * -- Add cars to a city
     * -- Adding to an existing car overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn addCar(&mut self, carId: u64, numCars: u64, price: u64) -> bool;

    /* =============================================================================
     * deleteCar
     * -- Delete cars from a city
     * -- Decreases available car count (those not allocated to a customer)
     * -- Fails if would make available car count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteCar(&mut self, carId: u64, numCar: u64) -> bool;

    /* =============================================================================
     * addRoom
     * -- Add rooms to a city
     * -- Adding to an existing room overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn addRoom(&mut self, roomId: u64, numRoom: u64, price: u64) -> bool;

    /* =============================================================================
     * deleteRoom
     * -- Delete rooms from a city
     * -- Decreases available room count (those not allocated to a customer)
     * -- Fails if would make available room count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteRoom(&mut self, roomId: u64, numRoom: u64) -> bool;

    /* =============================================================================
     * addFlight
     * -- Add seats to a flight
     * -- Adding to an existing flight overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, FALSE on failure
     * =============================================================================
     */
    fn addFlight(&mut self, flightId: u64, numSeat: u64, price: u64) -> bool;

    /* =============================================================================
     * deleteFlight
     * -- Delete an entire flight
     * -- Fails if customer has reservation on this flight
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteFlight(&mut self, flightId: u64) -> bool;

    /* =============================================================================
     * addCustomer
     * -- If customer already exists, returns failure
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn addCustomer(&mut self, customerId: u64) -> bool;

    /* =============================================================================
     * deleteCustomer
     * -- Delete this customer and associated reservations
     * -- If customer does not exist, returns success
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn deleteCustomer(&mut self, customerId: u64) -> bool;
}

impl Admin for Manager {
    fn addCar(&mut self, carId: u64, numCars: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.carTable,
            carId,
            TotalUpdate::Add(numCars),
            Some(price),
        )
    }

    fn deleteCar(&mut self, carId: u64, numCar: u64) -> bool {
        /* None keeps old price */
        upsert_reservation(
            &mut self.carTable,
            carId,
            TotalUpdate::Subtract(numCar),
            None,
        )
    }

    fn addRoom(&mut self, roomId: u64, numRoom: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.roomTable,
            roomId,
            TotalUpdate::Add(numRoom),
            Some(price),
        )
    }

    fn deleteRoom(&mut self, roomId: u64, numRoom: u64) -> bool {
        /* None keeps old price */
        upsert_reservation(
            &mut self.roomTable,
            roomId,
            TotalUpdate::Subtract(numRoom),
            None,
        )
    }

    fn addFlight(&mut self, flightId: u64, numSeat: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.flightTable,
            flightId,
            TotalUpdate::Add(numSeat),
            Some(price),
        )
    }

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

    fn addCustomer(&mut self, customerId: u64) -> bool {
        let customer_contained = self.customerTable.contains_key(&customerId);
        if customer_contained {
            false
        } else {
            self.customerTable
                .insert(customerId, Customer::new(customerId));
            true
        }
    }

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

pub(crate) trait QueryInterface {
    /* =============================================================================
     * queryCar
     * -- Return the number of empty seats on a car
     * =============================================================================
     */
    fn queryCar(&self, carId: u64) -> Option<u64>;

    /* =============================================================================
     * queryCarPrice
     * -- Return the price of the car
     * =============================================================================
     */
    fn queryCarPrice(&self, carId: u64) -> Option<u64>;

    /* =============================================================================
     * queryRoom
     * -- Return the number of empty seats on a room
     * =============================================================================
     */
    fn queryRoom(&self, roomId: u64) -> Option<u64>;

    /* =============================================================================
     * queryRoomPrice
     * -- Return the price of the room
     * =============================================================================
     */
    fn queryRoomPrice(&self, roomId: u64) -> Option<u64>;

    /* =============================================================================
     * queryFlight
     * -- Return the number of empty seats on a flight
     * =============================================================================
     */
    fn queryFlight(&self, flightId: u64) -> Option<u64>;

    /* =============================================================================
     * queryFlightPrice
     * -- Return the price of the flight
     * =============================================================================
     */
    fn queryFlightPrice(&self, flightId: u64) -> Option<u64>;

    /* =============================================================================
     * queryCustomerBill
     * -- Return the total price of all reservations held for a customer
     * =============================================================================
     */
    fn queryCustomerBill(&self, customerId: u64) -> Option<u64>;
}

/* =============================================================================
 * queryNumFree
 * -- Return numFree of a reservation, -1 if failure
 * =============================================================================
 */
fn queryNumFree(table: &HashMap<u64, Reservation>, id: u64) -> Option<u64> {
    table.get(&id).map(|r| r.num_free)
}

/* =============================================================================
 * queryPrice
 * -- Return price of a reservation, -1 if failure
 * =============================================================================
 */
fn queryPrice(table: &HashMap<u64, Reservation>, id: u64) -> Option<u64> {
    table.get(&id).map(|r| r.price)
}

impl QueryInterface for Manager {
    fn queryCar(&self, carId: u64) -> Option<u64> {
        queryNumFree(&self.carTable, carId)
    }

    fn queryCarPrice(&self, carId: u64) -> Option<u64> {
        queryPrice(&self.carTable, carId)
    }

    fn queryRoom(&self, roomId: u64) -> Option<u64> {
        queryNumFree(&self.roomTable, roomId)
    }

    fn queryRoomPrice(&self, roomId: u64) -> Option<u64> {
        queryPrice(&self.roomTable, roomId)
    }

    fn queryFlight(&self, flightId: u64) -> Option<u64> {
        queryNumFree(&self.flightTable, flightId)
    }

    fn queryFlightPrice(&self, flightId: u64) -> Option<u64> {
        queryPrice(&self.flightTable, flightId)
    }

    fn queryCustomerBill(&self, customerId: u64) -> Option<u64> {
        self.customerTable.get(&customerId).map(|c| c.get_bill())
    }
}

pub(crate) trait ReservationInterface {
    /* =============================================================================
     * reserveCar
     * -- Returns failure if the car or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserveCar(&mut self, customerId: u64, carId: u64) -> bool;

    /* =============================================================================
     * reserveRoom
     * -- Returns failure if the room or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserveRoom(&mut self, customerId: u64, roomId: u64) -> bool;

    /* =============================================================================
     * reserveFlight
     * -- Returns failure if the flight or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserveFlight(&mut self, customerId: u64, flightId: u64) -> bool;

    /* =============================================================================
     * cancelCar
     * -- Returns failure if the car, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancelCar(&mut self, customerId: u64, carId: u64) -> bool;

    /* =============================================================================
     * cancelRoom
     * -- Returns failure if the room, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancelRoom(&mut self, customerId: u64, roomId: u64) -> bool;

    /* =============================================================================
     * cancelFlight
     * -- Returns failure if the flight, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancelFlight(&mut self, customerId: u64, flightId: u64) -> bool;
}

/* =============================================================================
 * reserve
 * -- Customer is not allowed to reserve same (type, id) multiple times
 * -- Returns TRUE on success, else FALSE
 * =============================================================================
 */
fn reserve(
    table: &mut HashMap<u64, Reservation>,
    customerTable: &mut HashMap<u64, Customer>,
    customerId: u64,
    id: u64,
    typ: ReservationType,
) -> bool {
    match customerTable.get_mut(&customerId) {
        None => false,
        Some(customer) => match table.get_mut(&id) {
            None => false,
            Some(reservation) => match reservation.make() {
                false => false,
                true => {
                    customer.add_reservation_info(typ, id, reservation.price);
                    true
                }
            },
        },
    }
    // TODO the tranactional version needs to check whether the reservation was successful and
    // if not cancel it again.
}

/* =============================================================================
 * cancel
 * -- Customer is not allowed to cancel multiple times
 * -- Returns TRUE on success, else FALSE
 * =============================================================================
 */
fn cancel(
    table: &mut HashMap<u64, Reservation>,
    customerTable: &mut HashMap<u64, Customer>,
    customerId: u64,
    id: u64,
    typ: ReservationType,
) -> bool {
    match customerTable.get_mut(&customerId) {
        None => false,
        Some(customer) => match table.get_mut(&id) {
            None => false,
            Some(reservation) => match reservation.cancel() {
                false => false,
                true => {
                    customer.remove_reservation_info(typ, id);
                    true
                }
            },
        },
    }
    // TODO the tranactional version needs to check whether the cancellation was successful and
    // if not make it again.
}

impl ReservationInterface for Manager {
    fn reserveCar(&mut self, customerId: u64, carId: u64) -> bool {
        reserve(
            &mut self.carTable,
            &mut self.customerTable,
            customerId,
            carId,
            ReservationType::Car,
        )
    }

    fn reserveRoom(&mut self, customerId: u64, roomId: u64) -> bool {
        reserve(
            &mut self.roomTable,
            &mut self.customerTable,
            customerId,
            roomId,
            ReservationType::Room,
        )
    }

    fn reserveFlight(&mut self, customerId: u64, flightId: u64) -> bool {
        reserve(
            &mut self.flightTable,
            &mut self.customerTable,
            customerId,
            flightId,
            ReservationType::Flight,
        )
    }

    fn cancelCar(&mut self, customerId: u64, carId: u64) -> bool {
        cancel(
            &mut self.carTable,
            &mut self.customerTable,
            customerId,
            carId,
            ReservationType::Car,
        )
    }

    fn cancelRoom(&mut self, customerId: u64, roomId: u64) -> bool {
        cancel(
            &mut self.roomTable,
            &mut self.customerTable,
            customerId,
            roomId,
            ReservationType::Room,
        )
    }

    fn cancelFlight(&mut self, customerId: u64, flightId: u64) -> bool {
        cancel(
            &mut self.flightTable,
            &mut self.customerTable,
            customerId,
            flightId,
            ReservationType::Flight,
        )
    }
}
