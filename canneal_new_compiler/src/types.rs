use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha12Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

use rand::Rng;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetlistElement {
    pub item_name: Option<String>,
    pub fan_in: Vec<usize>,
    pub fan_out: Vec<usize>,
    pub location: Location,
}

impl NetlistElement {
    pub fn new(name: String, x: usize, y: usize) -> Self {
        Self {
            item_name: Some(name),
            fan_in: Vec::new(),
            fan_out: Vec::new(),
            location: Location { x, y },
        }
    }

    pub fn empty(x: usize, y: usize) -> Self {
        Self {
            item_name: None,
            fan_in: Vec::new(),
            fan_out: Vec::new(),
            location: Location { x, y },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Netlist {
    pub elements: Arc<Vec<NetlistElement>>,
    changed_fields: Arc<Vec<bool>>,
    pub failed_updates: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl Netlist {
    /// Create a netlist from a file specification
    pub fn new(path: &str) -> io::Result<Self> {
        let f = File::open(path)?;

        let mut reader = BufReader::new(f);

        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        // parse parameters
        let w: Vec<&str> = buf.trim().split_whitespace().collect();
        let element_count = usize::from_str(w[0]).unwrap();
        let max_x = usize::from_str(w[1]).unwrap();
        let max_y = usize::from_str(w[2]).unwrap();
        let chip_size = max_x * max_y;

        //println!("Dimensions: [{}, {}]", max_x, max_y);
        // create # elements and assign them to a location
        // let mut elem_vec = Vec::with_capacity(chip_size);
        // for x in 0..max_x {
        //     for y in 0..max_y {
        //         elem_vec.push(NetlistElement::new(String::new(), Location { x, y }));
        //     }
        // }

        // read the file line by line and create the elements
        let mut cur_x = 0;
        let mut cur_y = 0;
        // let mut elements = HashMap::with_capacity(element_count);
        let mut elements = Vec::with_capacity(chip_size);
        let mut tmp = HashMap::with_capacity(element_count);
        let mut to_link = Vec::with_capacity(element_count);

        // just read the file, creating the unlinked elements
        for line in reader.lines() {
            let line = line?;
            // Plan: go through the lines and create elements on the fly, storing away the fanins for later
            // second iteration: build the links
            // BUT: Just create a Vec<T> and no HashMap -> when we are using links anyway a HashMap (that either points to the vec index or contains a link to the element (like in the signature above)) is only necessary in the linking phase
            // so: fill Vec with blankos in the first run (note the type!!) and then enrich it with the information from the hashmap.

            // TODO(feliix42): Handle empty lines
            // read the data from the file
            let mut contents = line.trim().split_whitespace();
            let name = contents.next().unwrap();
            let _type = contents.next().unwrap();
            // drop the last element which is a `END`
            contents.next_back();
            let fanins: Vec<String> = contents.map(String::from).collect();

            // create the element
            let element = NetlistElement::new(name.to_string(), cur_x, cur_y);
            elements.push(element);
            // store the link for the second iteration
            tmp.insert(name.to_string(), elements.len() - 1);
            // store the fanins
            to_link.push(fanins);

            // increase the location
            cur_y = (cur_y + 1) % max_y;
            if cur_y == 0 {
                cur_x += 1;
            }
        }

        // println!("[debug] Filled the chip until pos ({},{})", cur_x, cur_y);

        // fill up the elements vector with the remaining positions
        for _ in elements.len()..chip_size {
            elements.push(NetlistElement::empty(cur_x, cur_y));

            // increase the location
            cur_y = (cur_y + 1) % max_y;
            if cur_y == 0 {
                cur_x += 1;
            }
        }

        // println!("[debug] Added empty positions until pos ({},{})", cur_x, cur_y);

        // generate the links
        for (idx, links) in (0..elements.len()).zip(to_link.into_iter()) {
            for item in links {
                let link_target = *tmp.get(&item).expect("All links must be valid");

                elements[idx].fan_in.push(link_target);
                elements[link_target].fan_out.push(idx);
            }
        }

        let size = elements.len();
        Ok(Self {
            elements: Arc::new(elements),
            changed_fields: Arc::new(vec![false; size]),
            failed_updates: 0,
            max_x,
            max_y,
        })
    }

    //pub fn get_element_by_name(&self, name: &str) -> usize {
        //unimplemented!()
    //}

    ///// Selects a random pair of different elements from the element list and returns their indices
    //pub fn get_random_pair<R: Rng>(&self, rng: &mut R) -> (usize, usize) {
        //assert!(self.elements.len() > 1);

        //let idx_a = rng.gen_range(0..self.elements.len());
        //let mut idx_b = rng.gen_range(0..self.elements.len());

        //while idx_a == idx_b {
            //idx_b = rng.gen_range(0..self.elements.len());
        //}

        //(idx_a, idx_b)
    //}

    //pub fn get_random_element<R: Rng>(&self, different_from: Option<usize>, rng: &mut R) -> usize {
        //assert!(self.elements.len() > 1);

        //let mut idx = rng.gen_range(0..self.elements.len());
        //if let Some(diff) = different_from {
            //while idx == diff {
                //idx = rng.gen_range(0..self.elements.len());
            //}
        //}

        //idx
    //}

    ///// Swap the location information for two elements, effectively swapping their positions
    //pub fn swap_locations(&mut self, idx_a: usize, idx_b: usize) {
        //unsafe {
            //let elems = Arc::get_mut_unchecked(&mut self.elements);
            //let mut tmp = std::mem::take(&mut elems[idx_a].location);
            //std::mem::swap(&mut tmp, &mut elems[idx_b].location);
            //elems[idx_a].location = tmp;
        //}
    //}

    ///// Shuffle the elements vector by randomly switching out x * y * 1000 pairs
    //pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        //let bounds = self.max_x * self.max_y * 1000;

        //for _ in 0..bounds {
            //let (a, b) = self.get_random_pair(rng);
            //self.swap_locations(a, b);
        //}
    //}

    ///// Count the total routing cost for the netlist.
    //pub fn total_routing_cost(&self) -> f64 {
        //let mut cost = 0f64;

        //for element in 0..self.elements.len() {
            //cost += self.element_routing_cost(element);
        //}

        //// divide by two since the `routing_cost` function considers both fan-in and fan-out.
        //cost / 2f64
    //}

    ///// Calculates the routing cost using the Manhattan distancee.
    //pub fn element_routing_cost(&self, elem: usize) -> f64 {
        //let mut fan_in_cost = 0_f64;
        //let mut fan_out_cost = 0_f64;

        //let element = &self.elements[elem];

        //for other in &element.fan_in {
            //let el = &self.elements[*other];
            //fan_in_cost += (element.location.x as isize - el.location.x as isize).abs() as f64;
            //fan_in_cost += (element.location.y as isize - el.location.y as isize).abs() as f64;
        //}

        //for other in &element.fan_out {
            //let el = &self.elements[*other];
            //fan_out_cost += (element.location.x as isize - el.location.x as isize).abs() as f64;
            //fan_out_cost += (element.location.y as isize - el.location.y as isize).abs() as f64;
        //}

        //fan_in_cost + fan_out_cost
    //}

    /// Get the cost change of swapping from the present location to a new location
    pub fn element_swap_cost(&self, elem: usize, old: &Location, new: &Location) -> f64 {
        let mut no_swap = 0_f64;
        let mut yes_swap = 0_f64;

        for other in &self.elements[elem].fan_in {
            let el = &self.elements[*other];
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;

            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
        }

        for other in &self.elements[elem].fan_out {
            let el = &self.elements[*other];
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.y as isize - el.location.y as isize).abs() as f64;

            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
        }

        yes_swap - no_swap
    }

    pub fn calculate_delta_routing_cost(&self, a: usize, b: usize) -> f64 {
        // TODO: WIP-ish implementation of the original
        let mut delta_cost =
            self.element_swap_cost(a, &self.elements[a].location, &self.elements[b].location);
        delta_cost +=
            self.element_swap_cost(b, &self.elements[b].location, &self.elements[a].location);

        delta_cost
    }

    pub fn update(
        &mut self,
        updts: Vec<(MoveDecision, (usize, usize))>,
    ) -> Vec<(MoveDecision, (usize, usize))> {
        //println!("Strong counts: {} (elems), {} (changed)", Arc::strong_count(&self.elements), Arc::strong_count(&self.changed_fields));
        let mut res = Vec::with_capacity(updts.len());
        let elems = Arc::get_mut(&mut self.elements).unwrap();
        let changed = Arc::get_mut(&mut self.changed_fields).unwrap();

        for updt in updts {
            let (a, b) = updt.1;

            match updt.0 {
                MoveDecision::Good |
                MoveDecision::Bad => {
                    if !changed[a] && !changed[b] {
                        swap_locations(elems, a, b);
                        changed[a] = true;
                        changed[b] = true;

                        res.push(updt);
                    } else {
                        self.failed_updates += 1;
                        res.push((MoveDecision::Undecided, updt.1));
                    }
                }
                MoveDecision::Rejected => (), // Ok(updt.0)
                MoveDecision::Undecided => res.push(updt) // Ok(updt.0)
            }
        }

        res
    }

    pub fn clear_changes(&mut self) {
        self.changed_fields = Arc::new(vec![false; self.elements.len()]);
    }
}
    
/// Swap the location information for two elements, effectively swapping their positions
pub fn swap_locations(elems: &mut Vec<NetlistElement>, idx_a: usize, idx_b: usize) {
    let mut tmp = std::mem::take(&mut elems[idx_a].location);
    std::mem::swap(&mut tmp, &mut elems[idx_b].location);
    elems[idx_a].location = tmp;
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveDecision {
    Good,
    Bad,
    Rejected,
    Undecided,
}

//pub fn increment(completed_steps: i32) -> i32 {
    //completed_steps + 1
//}

pub fn reduce_temp(temperature: f64) -> f64 {
    temperature / 1.5
}

pub fn process_move(
    work_item: (MoveDecision, (usize, usize)),
    netlist: Arc<Netlist>,
    temperature: f64,
) -> (MoveDecision, (usize, usize)) {
    // TODO: Uncomment
    assert_eq!(work_item.0, MoveDecision::Undecided);
    let item = work_item.1; 
    let total_cost = netlist.calculate_delta_routing_cost(item.0, item.1);

    let decision = if total_cost < 0f64 {
        MoveDecision::Good
    } else {
        let random_value: f64 = rand::random();
        let boltzman = (-total_cost / temperature).exp();
        if boltzman > random_value {
            MoveDecision::Bad
        } else {
            MoveDecision::Rejected
        }
    };

    (decision, item)
}

pub fn dup<T: Clone>(item: T) -> (T, T) {
    (item.clone(), item)
}

pub fn filter_work(
    work: Vec<(MoveDecision, (usize, usize))>,
) -> Vec<(MoveDecision, (usize, usize))> {
    //let mut retry = Vec::new();

    //for item in work {
        //match item {
            //Ok(res) => (),
            //Err(retr) => retry.push(retr),
        //}
    //}

    //retry

    work.into_iter().filter(|x| x.0 == MoveDecision::Undecided).collect()
}

pub struct InternalState {
    rng: ChaCha12Rng,
    total_elements: usize,
    accepted_good_moves: u32,
    accepted_bad_moves: u32,
    max_steps: Option<i32>,
    completed_steps: i32,
    swaps_per_temp: usize,
}

impl InternalState {
    pub fn initialize(total_elements: usize, max_steps: Option<i32>, swaps_per_temp: usize) -> Self {
        Self {
            rng: ChaCha12Rng::seed_from_u64(0),
            total_elements,
            accepted_good_moves: 0,
            accepted_bad_moves: 0,
            max_steps,
            completed_steps: 0,
            swaps_per_temp,
        }
    }

    pub fn assess_updates(
        &mut self,
        results: Vec<(MoveDecision, (usize, usize))>,
        length: usize,
    ) -> (Vec<(MoveDecision, (usize, usize))>, bool) {
         // update internal state
        for res in results {
            match res.0 {
                MoveDecision::Good => self.accepted_good_moves += 1,
                MoveDecision::Bad => self.accepted_bad_moves += 1,
                MoveDecision::Rejected |
                MoveDecision::Undecided => (),
            }
        }

        // generate a new worklist if necessary
        if length == 0 {
            println!("Current done!");
            // current worklist done!
            let keep_going = if let Some(bound) = self.max_steps {
                self.completed_steps < bound
            } else {
                self.accepted_good_moves > self.accepted_bad_moves
            };

            self.accepted_good_moves = 0;
            self.accepted_bad_moves = 0;
            self.completed_steps += 1;

            if keep_going {
                (self.generate_worklist(), true)
            } else {
                (Vec::with_capacity(0), false)
            }
        } else {
            println!("Remaining elements: {}", length);
            (Vec::with_capacity(0), true)
        }
    }

    pub fn generate_worklist(
        &mut self,
    ) -> Vec<(MoveDecision, (usize, usize))> {
        let mut res = Vec::with_capacity(self.swaps_per_temp);
        let mut idx_a;
        let mut idx_b;

        for _ in 0..self.swaps_per_temp {
            // todo
            idx_a = self.rng.gen_range(0..self.total_elements);
            idx_b = self.rng.gen_range(0..self.total_elements);
            
            while idx_a == idx_b {
                idx_b = self.rng.gen_range(0..self.total_elements);
            }

            res.push((MoveDecision::Undecided, (idx_a, idx_b)));
        }

        res
    }
}

pub trait Expand {
    fn exp(&mut self, elem: Self);
}

impl<T> Expand for Vec<T>
where
    T: Clone,
{
    fn exp(&mut self, elem: Vec<T>) {
        self.extend_from_slice(&elem);
    }
}
