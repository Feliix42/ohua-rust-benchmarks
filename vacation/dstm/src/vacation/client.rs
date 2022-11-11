use crate::vacation::action::Action;
use crate::vacation::manager::{Admin, Manager, QueryInterface, ReservationInterface};
use crate::vacation::reservation::ReservationType;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};
use stm::{det_atomically, DTMHandle};

pub struct Client<T: RngCore + SeedableRng> {
    //id: u64,
    manager: Arc<Manager>,
    random: T,
    num_operation: usize,
    num_query_per_transaction: usize,
    query_range: u64,
    percent_user: i64,
}

impl<T: RngCore + SeedableRng> Client<T> {
    pub fn new(
        //id: u64,
        manager: Arc<Manager>,
        num_operation: usize,
        num_query_per_transaction: usize,
        query_range: u64,
        percent_user: i64,
    ) -> Self {
        Client {
            //id,
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
    pub fn run(&mut self, done_sx: Sender<()>, handle_rx: Receiver<DTMHandle>) {
        for _ in 0..self.num_operation {
            // request new handle
            done_sx.send(()).unwrap();

            let r = self.random.gen::<i64>() % 100;
            let action = select_action(r, self.percent_user);

            // receive new handle
            let dtm_handle = handle_rx.recv().unwrap();

            match action {
                Action::MakeReservation => {
                    let num_query = self.random.gen::<usize>() % self.num_query_per_transaction + 1;
                    let customer_id = self.random.gen::<u64>() % self.query_range + 1;
                    let seed = self.random.gen::<u64>();

                    det_atomically(dtm_handle, |trans| {
                        let mut max_prices = vec![None, None, None];
                        let mut max_ids = vec![None, None, None];
                        let mut is_found = false;
                        let mut rng = ChaCha12Rng::seed_from_u64(seed);

                        for _ in 0..num_query {
                            let t = rng.gen::<ReservationType>();
                            let id = (rng.gen::<u64>() % self.query_range) + 1;
                            let price = match t {
                                ReservationType::Car => {
                                    if self.manager.query_car(id, trans)?.is_some() {
                                        self.manager.query_car_price(id, trans)?
                                    } else {
                                        None
                                    }
                                }
                                ReservationType::Flight => {
                                    if self.manager.query_flight(id, trans)?.is_some() {
                                        self.manager.query_flight_price(id, trans)?
                                    } else {
                                        None
                                    }
                                }
                                ReservationType::Room => {
                                    if self.manager.query_room(id, trans)?.is_some() {
                                        self.manager.query_room_price(id, trans)?
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
                            self.manager.add_customer(customer_id, trans)?;
                        } else {
                            // nothing
                        }

                        match max_ids[ReservationType::Car as usize] {
                            Some(id) => {
                                self.manager.reserve_car(customer_id, id, trans)?;
                            }
                            _ => (),
                        }

                        match max_ids[ReservationType::Flight as usize] {
                            Some(id) => {
                                self.manager.reserve_flight(customer_id, id, trans)?;
                            }
                            _ => (),
                        }

                        match max_ids[ReservationType::Room as usize] {
                            Some(id) => {
                                self.manager.reserve_room(customer_id, id, trans)?;
                                Ok(())
                            }
                            _ => Ok(()),
                        }
                    });
                }
                Action::DeleteCustomer => {
                    let customer_id = self.random.gen::<u64>() % self.query_range + 1;
                    det_atomically(dtm_handle, |trans| {
                        let bill = self.manager.query_customer_bill(customer_id, trans)?;
                        if bill.is_some() {
                            self.manager.delete_customer(customer_id, trans)?;
                            Ok(())
                        } else {
                            Ok(())
                        }
                    });
                }
                Action::UpdateTables => {
                    let num_update =
                        self.random.gen::<usize>() % self.num_query_per_transaction + 1;
                    let seed = self.random.gen::<u64>();

                    det_atomically(dtm_handle, |trans| {
                        let mut rng = ChaCha12Rng::seed_from_u64(seed);
                        for _ in 0..num_update {
                            let t = rng.gen::<ReservationType>();
                            let id = (rng.gen::<u64>() % self.query_range) + 1;
                            let tmp = rng.gen::<bool>();
                            let new_price0 = if tmp {
                                Some(((rng.gen::<u64>() % 5) * 10) + 50)
                            } else {
                                None
                            };
                            match new_price0 {
                                Some(new_price) => match t {
                                    ReservationType::Car => {
                                        self.manager.add_car(id, 100, new_price, trans)?
                                    }
                                    ReservationType::Flight => {
                                        self.manager.add_flight(id, 100, new_price, trans)?
                                    }
                                    ReservationType::Room => {
                                        self.manager.add_room(id, 100, new_price, trans)?
                                    }
                                },
                                None => {
                                    /* do delete */
                                    match t {
                                        ReservationType::Car => {
                                            self.manager.delete_car(id, 100, trans)?
                                        }
                                        ReservationType::Flight => {
                                            self.manager.delete_flight(id, trans)?
                                        }
                                        ReservationType::Room => {
                                            self.manager.delete_room(id, 100, trans)?
                                        }
                                    }
                                }
                            };
                        }
                        Ok(())
                    });
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
    } else if r & 1 != 0 {
        // FIXME check this again. the original code was just `if r&1 {`.
        Action::DeleteCustomer
    } else {
        Action::UpdateTables
    }
}
