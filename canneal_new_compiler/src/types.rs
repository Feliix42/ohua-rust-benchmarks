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
    pub internal_state: InternalState,
    pub failed_updates: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl Netlist {
    /// Create a netlist from a file specification
    pub fn new(path: &str, max_steps: Option<i32>, swaps_per_temp: usize) -> io::Result<Self> {
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
            internal_state: InternalState::initialize(size, max_steps, swaps_per_temp),
            failed_updates: 0,
            max_x,
            max_y,
        })
    }

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
        updt_sets: Vec<Vec<(MoveDecision, (usize, usize))>>,
    ) -> Vec<Vec<(usize, usize)>> {
        //println!("Strong counts: {} (elems), {} (changed)", Arc::strong_count(&self.elements), Arc::strong_count(&self.changed_fields));
        let mut res = Vec::with_capacity(updt_sets.len());
        let elems = Arc::get_mut(&mut self.elements).unwrap();

        let mut changed = vec![false; elems.len()];

        let mut computations_this_step = 0;

        for updts in updt_sets {
            let mut tmp = Vec::with_capacity(updts.len());
            computations_this_step += updts.len();

            for updt in updts {
                let (a, b) = updt.1;

                match updt.0 {
                    MoveDecision::Good => {
                        if !changed[a] && !changed[b] {
                            swap_locations(elems, a, b);
                            changed[a] = true;
                            changed[b] = true;

                            self.internal_state.accepted_good_moves += 1;
                        } else {
                            self.failed_updates += 1;
                            tmp.push(updt.1);
                        }
                    }
                    MoveDecision::Bad => {
                        if !changed[a] && !changed[b] {
                            swap_locations(elems, a, b);
                            changed[a] = true;
                            changed[b] = true;

                            self.internal_state.accepted_bad_moves += 1;
                        } else {
                            self.failed_updates += 1;
                            tmp.push(updt.1);
                        }
                    }
                    MoveDecision::Rejected => (),
                }
            }
            res.push(tmp);
        }

        self.internal_state.total_moves += computations_this_step;

        if res.iter().flatten().count() == 0 {
            //println!("Current done!");
            // current worklist done!
            let keep_going = self.get_keep_going();

            self.internal_state.accepted_good_moves = 0;
            self.internal_state.accepted_bad_moves = 0;
            self.internal_state.completed_steps += 1;

            if keep_going {
                res = self.internal_state.generate_worklist();
            }
        } /* else {
              println!("Remaining elements: {}", res.len());
              println!("Failed updates: {}", self.failed_updates);
          } */

        res
    }

    pub fn get_keep_going(&self) -> bool {
        if let Some(bound) = self.internal_state.max_steps {
            self.internal_state.completed_steps < bound
        } else {
            self.internal_state.accepted_good_moves > self.internal_state.accepted_bad_moves
        }
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
}

pub fn reduce_temp(temperature: f64) -> f64 {
    temperature / 1.5
}

pub fn process_move(
    items: Vec<(usize, usize)>,
    netlist: Arc<Netlist>,
    temperature: f64,
) -> Vec<(MoveDecision, (usize, usize))> {
    let mut res = Vec::with_capacity(items.len());

    for item in items {
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

        res.push((decision, item));
    }

    res
}

#[derive(Clone, Debug)]
pub struct InternalState {
    rng: ChaCha12Rng,
    total_elements: usize,
    pub total_moves: usize,
    accepted_good_moves: u32,
    accepted_bad_moves: u32,
    max_steps: Option<i32>,
    completed_steps: i32,
    swaps_per_temp: usize,
}

impl InternalState {
    pub fn initialize(
        total_elements: usize,
        max_steps: Option<i32>,
        swaps_per_temp: usize,
    ) -> Self {
        Self {
            rng: ChaCha12Rng::seed_from_u64(0),
            total_elements,
            total_moves: 0,
            accepted_good_moves: 0,
            accepted_bad_moves: 0,
            max_steps,
            completed_steps: 0,
            swaps_per_temp,
        }
    }

    //pub fn generate_worklist(
    //&mut self,
    //) -> Vec<(usize, usize)> {
    //let mut res = Vec::with_capacity(self.swaps_per_temp);
    //let mut idx_a;
    //let mut idx_b;

    //for _ in 0..self.swaps_per_temp {
    //// todo
    //idx_a = self.rng.gen_range(0..self.total_elements);
    //idx_b = self.rng.gen_range(0..self.total_elements);

    //while idx_a == idx_b {
    //idx_b = self.rng.gen_range(0..self.total_elements);
    //}

    //res.push((idx_a, idx_b));
    //}

    //res
    //}

    pub fn generate_worklist(&mut self) -> Vec<Vec<(usize, usize)>> {
        // Optimization: Bigger work packages
        let mut res = Vec::with_capacity(self.swaps_per_temp / 100);
        let mut idx_a;
        let mut idx_b;

        let mut tmp = Vec::with_capacity(100);
        for _ in 0..self.swaps_per_temp {
            // todo
            idx_a = self.rng.gen_range(0..self.total_elements);
            idx_b = self.rng.gen_range(0..self.total_elements);

            while idx_a == idx_b {
                idx_b = self.rng.gen_range(0..self.total_elements);
            }

            tmp.push((idx_a, idx_b));
            if tmp.len() == 100 {
                res.push(tmp);
                tmp = Vec::with_capacity(100);
            }
        }

        if !tmp.is_empty() {
            res.push(tmp);
        }

        println!("Generated {} elements", res.iter().flatten().count());

        res
    }
}
