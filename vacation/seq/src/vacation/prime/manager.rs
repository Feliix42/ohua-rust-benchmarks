use crate::vacation::customer::Customer;
use crate::vacation::reservation::{Reservation, ReservationType, TotalUpdate};
use rand::Rng;
use std::collections::HashMap;

// The benchmark implements tables as hashmaps.
// That is reasonable for point access but any range read
// will be a big performance burden.
pub struct Manager {
    car_table: HashMap<u64, Reservation>,
    room_table: HashMap<u64, Reservation>,
    flight_table: HashMap<u64, Reservation>,
    customer_table: HashMap<u64, Customer>, // a customer should at least have a name etc.
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            car_table: HashMap::new(),
            room_table: HashMap::new(),
            flight_table: HashMap::new(),
            customer_table: HashMap::new(),
        }
    }

    pub fn initialize<T: Rng>(&mut self, rng: &mut T, num_relations: usize) {
        let mut ids: Vec<u64> = (1..(num_relations as u64) + 1).collect();

        for tab in 0..3 {
            for _ in 0..num_relations {
                let x: usize = rng.gen_range(0..num_relations);
                let y: usize = rng.gen_range(0..num_relations);
                ids.swap(x, y);
            }

            for id in &ids {
                let num = (rng.gen_range(0..5) + 1) * 100;
                let price = (rng.gen_range(0..5) * 10) + 50;
                match tab {
                    0 => self.add_car(*id, num, price),
                    1 => self.add_flight(*id, num, price),
                    2 => self.add_room(*id, num, price),
                    _ => unreachable!("table num cannot exceed 0..2"),
                };
            }
        }

        for _ in 0..num_relations {
            let x: usize = rng.gen_range(0..num_relations);
            let y: usize = rng.gen_range(0..num_relations);
            ids.swap(x, y);
        }

        ids.into_iter().for_each(|id| {
            self.add_customer(id);
        });
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
                Some(res) => {
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
     * add_car
     * -- Add cars to a city
     * -- Adding to an existing car overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn add_car(&mut self, car_id: u64, num_cars: u64, price: u64) -> bool;

    /* =============================================================================
     * delete_car
     * -- Delete cars from a city
     * -- Decreases available car count (those not allocated to a customer)
     * -- Fails if would make available car count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_car(&mut self, car_id: u64, num_car: u64) -> bool;

    /* =============================================================================
     * add_room
     * -- Add rooms to a city
     * -- Adding to an existing room overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn add_room(&mut self, room_id: u64, num_room: u64, price: u64) -> bool;

    /* =============================================================================
     * delete_room
     * -- Delete rooms from a city
     * -- Decreases available room count (those not allocated to a customer)
     * -- Fails if would make available room count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_room(&mut self, room_id: u64, num_room: u64) -> bool;

    /* =============================================================================
     * add_flight
     * -- Add seats to a flight
     * -- Adding to an existing flight overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, FALSE on failure
     * =============================================================================
     */
    fn add_flight(&mut self, flight_id: u64, num_seat: u64, price: u64) -> bool;

    /* =============================================================================
     * delete_flight
     * -- Delete an entire flight
     * -- Fails if customer has reservation on this flight
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_flight(&mut self, flight_id: u64) -> bool;

    /* =============================================================================
     * add_customer
     * -- If customer already exists, returns failure
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn add_customer(&mut self, customer_id: u64) -> bool;

    /* =============================================================================
     * delete_customer
     * -- Delete this customer and associated reservations
     * -- If customer does not exist, returns success
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_customer(&mut self, customer_id: u64) -> bool;
}

impl Admin for Manager {
    fn add_car(&mut self, car_id: u64, num_cars: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.car_table,
            car_id,
            TotalUpdate::Add(num_cars),
            Some(price),
        )
    }

    fn delete_car(&mut self, car_id: u64, num_car: u64) -> bool {
        /* None keeps old price */
        upsert_reservation(
            &mut self.car_table,
            car_id,
            TotalUpdate::Subtract(num_car),
            None,
        )
    }

    fn add_room(&mut self, room_id: u64, num_room: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.room_table,
            room_id,
            TotalUpdate::Add(num_room),
            Some(price),
        )
    }

    fn delete_room(&mut self, room_id: u64, num_room: u64) -> bool {
        /* None keeps old price */
        upsert_reservation(
            &mut self.room_table,
            room_id,
            TotalUpdate::Subtract(num_room),
            None,
        )
    }

    fn add_flight(&mut self, flight_id: u64, num_seat: u64, price: u64) -> bool {
        upsert_reservation(
            &mut self.flight_table,
            flight_id,
            TotalUpdate::Add(num_seat),
            Some(price),
        )
    }

    fn delete_flight(&mut self, flight_id: u64) -> bool {
        let reservation = self.flight_table.get(&flight_id);
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
            Some(num_total) => upsert_reservation(
                &mut self.flight_table,
                flight_id,
                TotalUpdate::Subtract(num_total),
                None,
            ),
            None => false,
        }
    }

    fn add_customer(&mut self, customer_id: u64) -> bool {
        let customer_contained = self.customer_table.contains_key(&customer_id);
        if customer_contained {
            false
        } else {
            self.customer_table
                .insert(customer_id, Customer::new(customer_id));
            true
        }
    }

    fn delete_customer(&mut self, customer_id: u64) -> bool {
        let customer = self.customer_table.remove(&customer_id);
        match customer {
            None => false,
            Some(cus) => {
                /* Cancel this customer's reservations */
                for reservation in cus.reservation_info_list {
                    let tbl = match reservation.typ {
                        ReservationType::Car => &mut self.car_table,
                        ReservationType::Flight => &mut self.flight_table,
                        ReservationType::Room => &mut self.room_table,
                    };
                    let e = tbl.get_mut(&reservation.id);
                    e.map(|e0| e0.cancel());
                }
                true
            }
        }
    }
}

pub(crate) trait QueryInterface {
    /* =============================================================================
     * query_car
     * -- Return the number of empty seats on a car
     * =============================================================================
     */
    fn query_car(&self, car_id: u64) -> Option<u64>;

    /* =============================================================================
     * query_car_price
     * -- Return the price of the car
     * =============================================================================
     */
    fn query_car_price(&self, car_id: u64) -> Option<u64>;

    /* =============================================================================
     * query_room
     * -- Return the number of empty seats on a room
     * =============================================================================
     */
    fn query_room(&self, room_id: u64) -> Option<u64>;

    /* =============================================================================
     * query_room_price
     * -- Return the price of the room
     * =============================================================================
     */
    fn query_room_price(&self, room_id: u64) -> Option<u64>;

    /* =============================================================================
     * query_flight
     * -- Return the number of empty seats on a flight
     * =============================================================================
     */
    fn query_flight(&self, flight_id: u64) -> Option<u64>;

    /* =============================================================================
     * query_flight_price
     * -- Return the price of the flight
     * =============================================================================
     */
    fn query_flight_price(&self, flight_id: u64) -> Option<u64>;

    /* =============================================================================
     * query_customer_bill
     * -- Return the total price of all reservations held for a customer
     * =============================================================================
     */
    fn query_customer_bill(&self, customer_id: u64) -> Option<u64>;
}

/* =============================================================================
 * query_num_free
 * -- Return numFree of a reservation, -1 if failure
 * =============================================================================
 */
fn query_num_free(table: &HashMap<u64, Reservation>, id: u64) -> Option<u64> {
    table.get(&id).map(|r| r.num_free)
}

/* =============================================================================
 * query_price
 * -- Return price of a reservation, -1 if failure
 * =============================================================================
 */
fn query_price(table: &HashMap<u64, Reservation>, id: u64) -> Option<u64> {
    table.get(&id).map(|r| r.price)
}

impl QueryInterface for Manager {
    fn query_car(&self, car_id: u64) -> Option<u64> {
        query_num_free(&self.car_table, car_id)
    }

    fn query_car_price(&self, car_id: u64) -> Option<u64> {
        query_price(&self.car_table, car_id)
    }

    fn query_room(&self, room_id: u64) -> Option<u64> {
        query_num_free(&self.room_table, room_id)
    }

    fn query_room_price(&self, room_id: u64) -> Option<u64> {
        query_price(&self.room_table, room_id)
    }

    fn query_flight(&self, flight_id: u64) -> Option<u64> {
        query_num_free(&self.flight_table, flight_id)
    }

    fn query_flight_price(&self, flight_id: u64) -> Option<u64> {
        query_price(&self.flight_table, flight_id)
    }

    fn query_customer_bill(&self, customer_id: u64) -> Option<u64> {
        self.customer_table.get(&customer_id).map(|c| c.get_bill())
    }
}

pub(crate) trait ReservationInterface {
    /* =============================================================================
     * reserve_car
     * -- Returns failure if the car or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserve_car(&mut self, customer_id: u64, car_id: u64) -> bool;

    /* =============================================================================
     * reserve_room
     * -- Returns failure if the room or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserve_room(&mut self, customer_id: u64, room_id: u64) -> bool;

    /* =============================================================================
     * reserve_flight
     * -- Returns failure if the flight or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserve_flight(&mut self, customer_id: u64, flight_id: u64) -> bool;

    /* =============================================================================
     * cancel_car
     * -- Returns failure if the car, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancel_car(&mut self, customer_id: u64, car_id: u64) -> bool;

    /* =============================================================================
     * cancel_room
     * -- Returns failure if the room, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancel_room(&mut self, customer_id: u64, room_id: u64) -> bool;

    /* =============================================================================
     * cancel_flight
     * -- Returns failure if the flight, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancel_flight(&mut self, customer_id: u64, flight_id: u64) -> bool;
}

/* =============================================================================
 * reserve
 * -- Customer is not allowed to reserve same (type, id) multiple times
 * -- Returns TRUE on success, else FALSE
 * =============================================================================
 */
fn reserve(
    table: &mut HashMap<u64, Reservation>,
    customer_table: &mut HashMap<u64, Customer>,
    customer_id: u64,
    id: u64,
    typ: ReservationType,
) -> bool {
    match customer_table.get_mut(&customer_id) {
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
    customer_table: &mut HashMap<u64, Customer>,
    customer_id: u64,
    id: u64,
    typ: ReservationType,
) -> bool {
    match customer_table.get_mut(&customer_id) {
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
    fn reserve_car(&mut self, customer_id: u64, car_id: u64) -> bool {
        reserve(
            &mut self.car_table,
            &mut self.customer_table,
            customer_id,
            car_id,
            ReservationType::Car,
        )
    }

    fn reserve_room(&mut self, customer_id: u64, room_id: u64) -> bool {
        reserve(
            &mut self.room_table,
            &mut self.customer_table,
            customer_id,
            room_id,
            ReservationType::Room,
        )
    }

    fn reserve_flight(&mut self, customer_id: u64, flight_id: u64) -> bool {
        reserve(
            &mut self.flight_table,
            &mut self.customer_table,
            customer_id,
            flight_id,
            ReservationType::Flight,
        )
    }

    fn cancel_car(&mut self, customer_id: u64, car_id: u64) -> bool {
        cancel(
            &mut self.car_table,
            &mut self.customer_table,
            customer_id,
            car_id,
            ReservationType::Car,
        )
    }

    fn cancel_room(&mut self, customer_id: u64, room_id: u64) -> bool {
        cancel(
            &mut self.room_table,
            &mut self.customer_table,
            customer_id,
            room_id,
            ReservationType::Room,
        )
    }

    fn cancel_flight(&mut self, customer_id: u64, flight_id: u64) -> bool {
        cancel(
            &mut self.flight_table,
            &mut self.customer_table,
            customer_id,
            flight_id,
            ReservationType::Flight,
        )
    }
}
