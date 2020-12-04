use std::rc::Rc;
use std::cell::RefCell;

pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct NetlistElement {
    pub item_name: String,
    pub fan_in: Vec<Rc<RefCell<NetlistElement>>>,
    pub fan_out: Vec<Rc<RefCell<NetlistElement>>>,
    pub location: Location,
}

impl NetlistElement {
    pub fn new(name: String, location: Location) -> Self {
        Self {
            item_name: name,
            fan_in: Vec::new(),
            fan_out: Vec::new(),
            location
        }
    }

    /// Calculates the routing cost using the Manhattan distancee.
    pub fn routing_cost(&self) -> f64 {
        let mut fan_in_cost = 0_f64;
        let mut fan_out_cost = 0_f64;

        for element in &self.fan_in {
            let el = element.borrow();
            fan_in_cost += (self.location.x as isize - el.location.x as isize).abs() as f64;
            fan_in_cost += (self.location.y as isize- el.location.y as isize).abs() as f64;
        }

        for element in &self.fan_out {
            let el = element.borrow();
            fan_out_cost += (self.location.x as isize - el.location.x as isize).abs() as f64;
            fan_out_cost += (self.location.y as isize - el.location.y as isize).abs() as f64;
        }

        fan_in_cost + fan_out_cost
    }

    // TODO(feliix): Is the `old` parameter even necessary?
    /// Get the cost change of swapping from the present location to a new location
    pub fn swap_cost(&self, old: &Location, new: &Location) -> f64 {
        let mut no_swap = 0_f64;
        let mut yes_swap = 0_f64;

        for element in &self.fan_in {
            let el = element.borrow();
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;

            yes_swap += (new.y as isize- el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize- el.location.y as isize).abs() as f64;
        }

        for element in &self.fan_out {
            let el = element.borrow();
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.y as isize - el.location.y as isize).abs() as f64;

            yes_swap += (new.y as isize- el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize- el.location.y as isize).abs() as f64;
        }

        yes_swap - no_swap
    }
}
