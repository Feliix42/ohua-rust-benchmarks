use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ReservationType {
    Car,
    Flight,
    Room
}

pub(crate) struct ReservationInfo {
    pub(crate) typ: ReservationType,
    pub(crate) id: u64,
    pub(crate) price: u64, /* holds price at time reservation was made */
}

pub(crate) struct Reservation {
    id: u64,
    pub(crate) num_used: u64,
    pub(crate) num_free: u64,
    pub(crate) num_total: u64,
    pub(crate) price: u64,
}

impl Distribution<ReservationType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ReservationType {
        match rng.gen_range(0..=2) { // rand 0.8
            0 => ReservationType::Car,
            1 => ReservationType::Flight,
            _ => ReservationType::Room,
        }
    }
}

impl ReservationInfo {
    pub(crate) fn new(typ: ReservationType, id: u64, price: u64) -> Self {
        ReservationInfo { typ, id, price }
    }
}

impl Eq for ReservationInfo {}

impl Ord for ReservationInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.typ.cmp(&other.typ) {
            Ordering::Equal => self.id.cmp(&other.id),
            o => o,
        }
    }
}

impl PartialOrd for ReservationInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ReservationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ && self.id == other.id
    }
}

pub(crate) enum TotalUpdate {
    Add(u64),
    Subtract(u64),
}

impl Reservation {
    pub(crate) fn new(id: u64, num_total: u64, price: u64) -> Self {
        Reservation {
            id,
            num_used: 0,
            num_free: num_total,
            num_total,
            price,
        }
    }

    /* =============================================================================
     * checkReservation
     * -- Check if consistent
     * =============================================================================
     */
    #[allow(dead_code)]
    fn check(&self) {
        // assert!(self.num_used >= 0);
        // assert!(self.num_free >= 0);
        // assert!(self.num_total >= 0);
        assert!((self.num_used + self.num_free) == self.num_total);
        // assert!(self.price >= 0);
    }

    pub(crate) fn update_total(&mut self, num: TotalUpdate) -> u64 {
        match num {
            TotalUpdate::Add(i) => {
                self.num_free += i;
                self.num_total += i;
            }
            TotalUpdate::Subtract(i) => {
                self.num_free = if i < self.num_free {
                    self.num_free - i
                } else {
                    0
                };
                self.num_total = if i < self.num_total {
                    self.num_total - i
                } else {
                    0
                };
            }
        };
        self.num_total
    }

    pub(crate) fn make(&mut self) -> bool {
        if self.num_free < 1 {
            false
        } else {
            self.num_used += 1;
            self.num_free -= 1;
            true
        }
    }

    pub(crate) fn cancel(&mut self) -> bool {
        if self.num_used < 1 {
            false
        } else {
            self.num_used -= 1;
            self.num_free += 1;
            true
        }
    }

    pub(crate) fn update_price(&mut self, new_price: u64) {
        self.price = new_price;
    }

    #[allow(dead_code)]
    fn hash0(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Hash for Reservation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Eq for Reservation {}

impl Ord for Reservation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Reservation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Reservation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
