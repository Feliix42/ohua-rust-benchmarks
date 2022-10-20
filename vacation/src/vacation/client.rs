use crate::vacation::action::Action;
use crate::vacation::manager::{Admin, Manager, QueryInterface, ReservationInterface};
use crate::vacation::reservation::ReservationType;
use rand::{Rng, RngCore, SeedableRng};

pub struct Client<T: RngCore + SeedableRng> {
    id: u64,
    manager: Manager,
    random: T,
    num_operation: usize,
    num_query_per_transaction: usize,
    query_range: u64,
    percent_user: i64,
}

impl<T: RngCore + SeedableRng> Client<T> {
    pub fn new(
        id: u64,
        manager: Manager,
        num_operation: usize,
        num_query_per_transaction: usize,
        query_range: u64,
        percent_user: i64,
    ) -> Self {
        Client {
            id,
            manager,
            random: <T as SeedableRng>::seed_from_u64(1),
            num_operation,
            num_query_per_transaction,
            query_range,
            percent_user,
        }
    }

    /* =============================================================================
     * client_run
     * -- Execute list operations on the database
     * =============================================================================
     */
    pub fn run(&mut self) {
        for _ in 0..self.num_operation {
            let r = self.random.gen::<i64>() % 100;
            let action = select_action(r, self.percent_user);

            match action {
                Action::MakeReservation => {
                    let mut max_prices = vec![None,None,None];
                    let mut max_ids = vec![None,None,None];
                    let num_query = self.random.gen::<usize>() % self.num_query_per_transaction + 1;
                    let customer_id = self.random.gen::<u64>() % self.query_range + 1;
                    let mut is_found = false;
                    for _ in 0..num_query {
                        let t = self.random.gen::<ReservationType>();
                        let id = (self.random.gen::<u64>() % self.query_range) + 1;
                        let price = match t {
                            ReservationType::Car => {
                                if self.manager.query_car(id).is_some() {
                                    self.manager.query_car_price(id)
                                } else {
                                    None
                                }
                            }
                            ReservationType::Flight => {
                                if self.manager.query_flight(id).is_some() {
                                    self.manager.query_flight_price(id)
                                } else {
                                    None
                                }
                            }
                            ReservationType::Room => {
                                if self.manager.query_room(id).is_some() {
                                    self.manager.query_room_price(id)
                                } else {
                                    None
                                }
                            }
                        };
                        let idx = t as usize;
                        if price > max_prices[idx] {
                            max_prices[idx] = price;
                            max_ids[idx] = Some(id);
                            is_found = true;
                        } else {
                            // nothing
                        }
                    } /* for n */

                    if is_found {
                        self.manager.add_customer(customer_id);
                    } else {
                        // nothing
                    }

                    match max_ids[ReservationType::Car as usize] {
                        Some(id) => {
                            self.manager.reserve_car(customer_id, id);
                        }
                        _ => (),
                    }

                    match max_ids[ReservationType::Flight as usize] {
                        Some(id) => {
                            self.manager.reserve_flight(customer_id, id);
                        }
                        _ => (),
                    }

                    match max_ids[ReservationType::Room as usize] {
                        Some(id) => {
                            self.manager.reserve_room(customer_id, id);
                        }
                        _ => (),
                    }
                }
                Action::DeleteCustomer => {
                    let customer_id = self.random.gen::<u64>() % self.query_range + 1;
                    let bill = self.manager.query_customer_bill(customer_id);
                    if bill.is_some() {
                        self.manager.delete_customer(customer_id);
                    } else {
                        //nothing
                    }
                }
                Action::UpdateTables => {
                    let num_update = self.random.gen::<usize>() % self.num_query_per_transaction + 1;
                    for _ in 0..num_update {
                        let t = self.random.gen::<ReservationType>();
                        let id = (self.random.gen::<u64>() % self.query_range) + 1;
                        let tmp = self.random.gen::<bool>();
                        let new_price0 = if tmp {
                            Some(((self.random.gen::<u64>() % 5) * 10) + 50)
                        } else {
                            None
                        };
                        match new_price0 {
                            Some(new_price) => match t {
                                ReservationType::Car => self.manager.add_car(id, 100, new_price),
                                ReservationType::Flight => {
                                    self.manager.add_flight(id, 100, new_price)
                                }
                                ReservationType::Room => self.manager.add_room(id, 100, new_price),
                            },
                            None => {
                                /* do delete */
                                match t {
                                    ReservationType::Car => self.manager.delete_car(id, 100),
                                    ReservationType::Flight => self.manager.delete_flight(id),
                                    ReservationType::Room => self.manager.delete_room(id, 100),
                                }
                            }
                        };
                    }
                } /* switch (action) */
            }
        } /* for i */
    }
}

/* =============================================================================
 * select_action
 * =============================================================================
 */
fn select_action(r: i64, percent_user: i64) -> Action {
    if r < percent_user {
        Action::MakeReservation
    } else if r & 1 == 1 {
        // FIXME check this again. the original code was just `if r&1 {`.
        Action::DeleteCustomer
    } else {
        Action::UpdateTables
    }
}
