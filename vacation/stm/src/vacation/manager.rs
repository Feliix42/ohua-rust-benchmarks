use crate::vacation::customer::Customer;
use crate::vacation::reservation::{Reservation, ReservationType, TotalUpdate};
use rand::Rng;
use std::collections::HashMap;
use stm::{StmResult, TVar, Transaction};
use stm_datastructures::THashMap;

// The benchmark implements tables as hashmaps.
// That is reasonable for point access but any range read
// will be a big performance burden.
pub struct Manager {
    car_table: THashMap<u64, TVar<Reservation>>,
    room_table: THashMap<u64, TVar<Reservation>>,
    flight_table: THashMap<u64, TVar<Reservation>>,
    customer_table: THashMap<u64, TVar<Customer>>, // a customer should at least have a name etc.
}

impl Manager {
    #[allow(dead_code)]
    pub fn new(bucket_no: usize) -> Self {
        Manager {
            car_table: THashMap::new(bucket_no),
            room_table: THashMap::new(bucket_no),
            flight_table: THashMap::new(bucket_no),
            customer_table: THashMap::new(bucket_no),
        }
    }

    pub fn initialize<T: Rng>(rng: &mut T, num_relations: usize, bucket_no: usize) -> Self {
        let mut cars = HashMap::new();
        let mut rooms = HashMap::new();
        let mut flights = HashMap::new();
        let mut customers = HashMap::new();

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
                    0 => cars.insert(*id, TVar::new(Reservation::new(*id, num, price))),
                    1 => flights.insert(*id, TVar::new(Reservation::new(*id, num, price))),
                    2 => rooms.insert(*id, TVar::new(Reservation::new(*id, num, price))),
                    _ => unreachable!("table num cannot exceed 0..2"),
                };
            }
        }

        for _ in 0..num_relations {
            let x: usize = rng.gen_range(0..num_relations);
            let y: usize = rng.gen_range(0..num_relations);
            ids.swap(x, y);
        }

        for id in ids {
            customers.insert(id, TVar::new(Customer::new(id)));
        }

        Manager {
            car_table: THashMap::from_hashmap(cars, bucket_no),
            room_table: THashMap::from_hashmap(rooms, bucket_no),
            flight_table: THashMap::from_hashmap(flights, bucket_no),
            customer_table: THashMap::from_hashmap(customers, bucket_no),
        }
    }
}

fn upsert_reservation(
    table: &THashMap<u64, TVar<Reservation>>,
    id: u64,
    num: TotalUpdate,
    price: Option<u64>,
    trans: &mut Transaction,
) -> StmResult<bool> {
    let bucket = table.get_bucket(&id);
    let tab = bucket
        .read_ref_atomic()
        .downcast::<HashMap<u64, TVar<Reservation>>>()
        .unwrap();

    // NOTE(feliix42): Possibly we'll run into a data race here: removed an item from the map which
    // is not caught by the atomic read?

    if let Some(p) = price {
        let (changed, remove) = if let Some(entry) = tab.get(&id) {
            /* Update existing reservation */
            let mut res = entry.read(trans)?;
            let num_total = res.update_total(num);

            let retval = if num_total == 0 {
                (true, true)
            } else {
                res.update_price(p);
                (true, false)
            };

            entry.write(trans, res)?;
            retval
        } else {
            // insert the entry
            match num {
                TotalUpdate::Subtract(_) => (false, false),
                TotalUpdate::Add(n) => {
                    bucket.modify(trans, |mut m| {
                        m.insert(id, TVar::new(Reservation::new(id, n, p)));
                        m
                    })?;
                    (true, false)
                }
            }
        };

        if remove {
            bucket.modify(trans, |mut m| {
                m.remove(&id);
                m
            })?;
        }

        Ok(changed)
    } else {
        Ok(false)
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
    fn add_car(
        &self,
        car_id: u64,
        num_cars: u64,
        price: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * delete_car
     * -- Delete cars from a city
     * -- Decreases available car count (those not allocated to a customer)
     * -- Fails if would make available car count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_car(&self, car_id: u64, num_car: u64, trans: &mut Transaction) -> StmResult<bool>;

    /* =============================================================================
     * add_room
     * -- Add rooms to a city
     * -- Adding to an existing room overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn add_room(
        &self,
        room_id: u64,
        num_room: u64,
        price: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * delete_room
     * -- Delete rooms from a city
     * -- Decreases available room count (those not allocated to a customer)
     * -- Fails if would make available room count negative
     * -- If decresed to 0, deletes entire entry
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_room(&self, room_id: u64, num_room: u64, trans: &mut Transaction) -> StmResult<bool>;

    /* =============================================================================
     * add_flight
     * -- Add seats to a flight
     * -- Adding to an existing flight overwrite the price if 'price' >= 0
     * -- Returns TRUE on success, FALSE on failure
     * =============================================================================
     */
    fn add_flight(
        &self,
        flight_id: u64,
        num_seat: u64,
        price: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * delete_flight
     * -- Delete an entire flight
     * -- Fails if customer has reservation on this flight
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_flight(&self, flight_id: u64, trans: &mut Transaction) -> StmResult<bool>;

    /* =============================================================================
     * add_customer
     * -- If customer already exists, returns failure
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn add_customer(&self, customer_id: u64, trans: &mut Transaction) -> StmResult<bool>;

    /* =============================================================================
     * delete_customer
     * -- Delete this customer and associated reservations
     * -- If customer does not exist, returns success
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn delete_customer(&self, customer_id: u64, trans: &mut Transaction) -> StmResult<bool>;
}

impl Admin for Manager {
    fn add_car(
        &self,
        car_id: u64,
        num_cars: u64,
        price: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        upsert_reservation(
            &self.car_table,
            car_id,
            TotalUpdate::Add(num_cars),
            Some(price),
            trans,
        )
    }

    fn delete_car(&self, car_id: u64, num_car: u64, trans: &mut Transaction) -> StmResult<bool> {
        /* None keeps old price */
        upsert_reservation(
            &self.car_table,
            car_id,
            TotalUpdate::Subtract(num_car),
            None,
            trans,
        )
    }

    fn add_room(
        &self,
        room_id: u64,
        num_room: u64,
        price: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        upsert_reservation(
            &self.room_table,
            room_id,
            TotalUpdate::Add(num_room),
            Some(price),
            trans,
        )
    }

    fn delete_room(&self, room_id: u64, num_room: u64, trans: &mut Transaction) -> StmResult<bool> {
        /* None keeps old price */
        upsert_reservation(
            &self.room_table,
            room_id,
            TotalUpdate::Subtract(num_room),
            None,
            trans,
        )
    }

    fn add_flight(
        &self,
        flight_id: u64,
        num_seat: u64,
        price: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        upsert_reservation(
            &self.flight_table,
            flight_id,
            TotalUpdate::Add(num_seat),
            Some(price),
            trans,
        )
    }

    fn delete_flight(&self, flight_id: u64, trans: &mut Transaction) -> StmResult<bool> {
        let bucket = self.flight_table.get_bucket(&flight_id);
        let tab = bucket
            .read_ref_atomic()
            .downcast::<HashMap<u64, TVar<Reservation>>>()
            .unwrap();

        let delete = match tab.get(&flight_id) {
            None => None,
            Some(r) => {
                let res = r.read(trans)?;
                if res.num_used > 0 {
                    None
                } else {
                    Some(res.num_total)
                }
            }
        };

        match delete {
            Some(num_total) => upsert_reservation(
                &self.flight_table,
                flight_id,
                TotalUpdate::Subtract(num_total),
                None,
                trans,
            ),
            None => Ok(false),
        }
    }

    fn add_customer(&self, customer_id: u64, trans: &mut Transaction) -> StmResult<bool> {
        let bucket = self.customer_table.get_bucket(&customer_id);
        let tab = bucket
            .read_ref_atomic()
            .downcast::<HashMap<u64, TVar<Customer>>>()
            .unwrap();

        if tab.contains_key(&customer_id) {
            Ok(false)
        } else {
            bucket.modify(trans, |mut m| {
                m.insert(customer_id, TVar::new(Customer::new(customer_id)));
                m
            })?;
            Ok(true)
        }
    }

    fn delete_customer(&self, customer_id: u64, trans: &mut Transaction) -> StmResult<bool> {
        let bucket = self.customer_table.get_bucket(&customer_id);
        let mut tab = bucket.read(trans)?;

        let customer = tab.remove(&customer_id);
        bucket.write(trans, tab)?;

        match customer {
            None => Ok(false),
            Some(c) => {
                let cus = c.read(trans)?;
                /* Cancel this customer's reservations */
                for reservation in cus.reservation_info_list {
                    let tbl = match reservation.typ {
                        ReservationType::Car => &self.car_table,
                        ReservationType::Flight => &self.flight_table,
                        ReservationType::Room => &self.room_table,
                    };
                    // TODO: update this
                    let e = tbl
                        .get_bucket(&reservation.id)
                        .read_ref_atomic()
                        .downcast::<HashMap<u64, TVar<Reservation>>>()
                        .unwrap();
                    if let Some(entry) = e.get(&reservation.id) {
                        entry.modify(trans, |mut e| {
                            e.cancel();
                            e
                        })?;
                    }
                }
                Ok(true)
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
    fn query_car(&self, car_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>>;

    /* =============================================================================
     * query_car_price
     * -- Return the price of the car
     * =============================================================================
     */
    fn query_car_price(&self, car_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>>;

    /* =============================================================================
     * query_room
     * -- Return the number of empty seats on a room
     * =============================================================================
     */
    fn query_room(&self, room_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>>;

    /* =============================================================================
     * query_room_price
     * -- Return the price of the room
     * =============================================================================
     */
    fn query_room_price(&self, room_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>>;

    /* =============================================================================
     * query_flight
     * -- Return the number of empty seats on a flight
     * =============================================================================
     */
    fn query_flight(&self, flight_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>>;

    /* =============================================================================
     * query_flight_price
     * -- Return the price of the flight
     * =============================================================================
     */
    fn query_flight_price(&self, flight_id: u64, trans: &mut Transaction)
        -> StmResult<Option<u64>>;

    /* =============================================================================
     * query_customer_bill
     * -- Return the total price of all reservations held for a customer
     * =============================================================================
     */
    fn query_customer_bill(
        &self,
        customer_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<Option<u64>>;
}

/* =============================================================================
 * query_num_free
 * -- Return numFree of a reservation, -1 if failure
 * =============================================================================
 */
fn query_num_free(
    table: &THashMap<u64, TVar<Reservation>>,
    id: u64,
    trans: &mut Transaction,
) -> StmResult<Option<u64>> {
    let tab = table
        .get_bucket(&id)
        .read_ref_atomic()
        .downcast::<HashMap<u64, TVar<Reservation>>>()
        .unwrap();
    match tab.get(&id) {
        Some(entry) => Ok(Some(entry.read(trans)?.num_free)),
        None => Ok(None),
    }
}

/* =============================================================================
 * query_price
 * -- Return price of a reservation, -1 if failure
 * =============================================================================
 */
fn query_price(
    table: &THashMap<u64, TVar<Reservation>>,
    id: u64,
    trans: &mut Transaction,
) -> StmResult<Option<u64>> {
    let tab = table
        .get_bucket(&id)
        .read_ref_atomic()
        .downcast::<HashMap<u64, TVar<Reservation>>>()
        .unwrap();
    match tab.get(&id) {
        Some(entry) => Ok(Some(entry.read(trans)?.price)),
        None => Ok(None),
    }
}

impl QueryInterface for Manager {
    fn query_car(&self, car_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>> {
        query_num_free(&self.car_table, car_id, trans)
    }

    fn query_car_price(&self, car_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>> {
        query_price(&self.car_table, car_id, trans)
    }

    fn query_room(&self, room_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>> {
        query_num_free(&self.room_table, room_id, trans)
    }

    fn query_room_price(&self, room_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>> {
        query_price(&self.room_table, room_id, trans)
    }

    fn query_flight(&self, flight_id: u64, trans: &mut Transaction) -> StmResult<Option<u64>> {
        query_num_free(&self.flight_table, flight_id, trans)
    }

    fn query_flight_price(
        &self,
        flight_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<Option<u64>> {
        query_price(&self.flight_table, flight_id, trans)
    }

    fn query_customer_bill(
        &self,
        customer_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<Option<u64>> {
        let tab = self
            .customer_table
            .get_bucket(&customer_id)
            .read_ref_atomic()
            .downcast::<HashMap<u64, TVar<Customer>>>()
            .unwrap();
        match tab.get(&customer_id) {
            Some(entry) => Ok(Some(entry.read(trans)?.get_bill())),
            None => Ok(None),
        }
    }
}

pub(crate) trait ReservationInterface {
    /* =============================================================================
     * reserve_car
     * -- Returns failure if the car or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserve_car(
        &self,
        customer_id: u64,
        car_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * reserve_room
     * -- Returns failure if the room or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserve_room(
        &self,
        customer_id: u64,
        room_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * reserve_flight
     * -- Returns failure if the flight or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn reserve_flight(
        &self,
        customer_id: u64,
        flight_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * cancel_car
     * -- Returns failure if the car, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancel_car(&self, customer_id: u64, car_id: u64, trans: &mut Transaction)
        -> StmResult<bool>;

    /* =============================================================================
     * cancel_room
     * -- Returns failure if the room, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancel_room(
        &self,
        customer_id: u64,
        room_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;

    /* =============================================================================
     * cancel_flight
     * -- Returns failure if the flight, reservation, or customer does not exist
     * -- Returns TRUE on success, else FALSE
     * =============================================================================
     */
    fn cancel_flight(
        &self,
        customer_id: u64,
        flight_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool>;
}

/* =============================================================================
 * reserve
 * -- Customer is not allowed to reserve same (type, id) multiple times
 * -- Returns TRUE on success, else FALSE
 * =============================================================================
 */
fn reserve(
    table: &THashMap<u64, TVar<Reservation>>,
    customer_table: &THashMap<u64, TVar<Customer>>,
    customer_id: u64,
    id: u64,
    typ: ReservationType,
    trans: &mut Transaction,
) -> StmResult<bool> {
    let c_bucket = customer_table.get_bucket(&customer_id);
    let c_tab = c_bucket
        .read_ref_atomic()
        .downcast::<HashMap<u64, TVar<Customer>>>()
        .unwrap();

    match c_tab.get(&customer_id) {
        None => Ok(false),
        Some(customer) => {
            let t_buck = table.get_bucket(&id);
            let tab = t_buck
                .read_ref_atomic()
                .downcast::<HashMap<u64, TVar<Reservation>>>()
                .unwrap();

            match tab.get(&id) {
                None => Ok(false),
                Some(r) => {
                    let mut res = r.read(trans)?;
                    match res.make() {
                        // this is ok without write due to the semantics of make()
                        false => Ok(false),
                        true => {
                            customer.modify(trans, |mut c| {
                                c.add_reservation_info(typ, id, res.price);
                                c
                            })?;
                            r.write(trans, res)?;
                            Ok(true)
                        }
                    }
                }
            }
        }
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
    table: &THashMap<u64, TVar<Reservation>>,
    customer_table: &THashMap<u64, TVar<Customer>>,
    customer_id: u64,
    id: u64,
    typ: ReservationType,
    trans: &mut Transaction,
) -> StmResult<bool> {
    let c_bucket = customer_table.get_bucket(&customer_id);
    let c_tab = c_bucket
        .read_ref_atomic()
        .downcast::<HashMap<u64, TVar<Customer>>>()
        .unwrap();

    match c_tab.get(&customer_id) {
        None => Ok(false),
        Some(customer) => {
            let t_buck = table.get_bucket(&id);
            let tab = t_buck
                .read_ref_atomic()
                .downcast::<HashMap<u64, TVar<Reservation>>>()
                .unwrap();

            match tab.get(&id) {
                None => Ok(false),
                Some(r) => {
                    let mut res = r.read(trans)?;
                    match res.cancel() {
                        // this is ok without write due to the semantics of make()
                        false => Ok(false),
                        true => {
                            customer.modify(trans, |mut c| {
                                c.remove_reservation_info(typ, id);
                                c
                            })?;
                            r.write(trans, res)?;
                            Ok(true)
                        }
                    }
                }
            }
        }
    }
    //match customer_table.get_mut(&customer_id) {
    //None => false,
    //Some(customer) => match table.get_mut(&id) {
    //None => false,
    //Some(reservation) => match reservation.cancel() {
    //false => false,
    //true => {
    //customer.remove_reservation_info(typ, id);
    //true
    //}
    //},
    //},
    //}
    // TODO the tranactional version needs to check whether the cancellation was successful and
    // if not make it again.
}

impl ReservationInterface for Manager {
    fn reserve_car(
        &self,
        customer_id: u64,
        car_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        reserve(
            &self.car_table,
            &self.customer_table,
            customer_id,
            car_id,
            ReservationType::Car,
            trans,
        )
    }

    fn reserve_room(
        &self,
        customer_id: u64,
        room_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        reserve(
            &self.room_table,
            &self.customer_table,
            customer_id,
            room_id,
            ReservationType::Room,
            trans,
        )
    }

    fn reserve_flight(
        &self,
        customer_id: u64,
        flight_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        reserve(
            &self.flight_table,
            &self.customer_table,
            customer_id,
            flight_id,
            ReservationType::Flight,
            trans,
        )
    }

    fn cancel_car(
        &self,
        customer_id: u64,
        car_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        cancel(
            &self.car_table,
            &self.customer_table,
            customer_id,
            car_id,
            ReservationType::Car,
            trans,
        )
    }

    fn cancel_room(
        &self,
        customer_id: u64,
        room_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        cancel(
            &self.room_table,
            &self.customer_table,
            customer_id,
            room_id,
            ReservationType::Room,
            trans,
        )
    }

    fn cancel_flight(
        &self,
        customer_id: u64,
        flight_id: u64,
        trans: &mut Transaction,
    ) -> StmResult<bool> {
        cancel(
            &self.flight_table,
            &self.customer_table,
            customer_id,
            flight_id,
            ReservationType::Flight,
            trans,
        )
    }
}
