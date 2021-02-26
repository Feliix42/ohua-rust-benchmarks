use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::{collections::HashMap, rc::Rc};
use stm::{StmResult, TVar, Transaction};

use rand::Rng;

#[derive(PartialEq, Clone, Eq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct NetlistElement {
    pub item_name: Option<String>,
    pub fan_in: Vec<TVar<NetlistElement>>,
    pub fan_out: Vec<TVar<NetlistElement>>,
    pub location: Location,
}

impl PartialEq for NetlistElement {
    fn eq(&self, other: &Self) -> bool {
        self.item_name == other.item_name
            && self.location == other.location
            && self
                .fan_in
                .iter()
                .zip(other.fan_in.iter())
                .fold(true, |acc, (this, other)| acc && TVar::ref_eq(this, other))
            && self
                .fan_out
                .iter()
                .zip(other.fan_out.iter())
                .fold(true, |acc, (this, other)| acc && TVar::ref_eq(this, other))
    }
}

impl Eq for NetlistElement {}

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

    /// Calculates the routing cost using the Manhattan distancee.
    pub fn routing_cost(&self, trans: &mut Transaction) -> StmResult<f64> {
        let mut fan_in_cost = 0_f64;
        let mut fan_out_cost = 0_f64;

        for element in &self.fan_in {
            let el = element.read(trans)?;
            fan_in_cost += (self.location.x as isize - el.location.x as isize).abs() as f64;
            fan_in_cost += (self.location.y as isize - el.location.y as isize).abs() as f64;
        }

        for element in &self.fan_out {
            let el = element.read(trans)?;
            fan_out_cost += (self.location.x as isize - el.location.x as isize).abs() as f64;
            fan_out_cost += (self.location.y as isize - el.location.y as isize).abs() as f64;
        }

        Ok(fan_in_cost + fan_out_cost)
    }

    // TODO(feliix): Is the `old` parameter even necessary?
    /// Get the cost change of swapping from the present location to a new location
    pub fn swap_cost(
        &self,
        old: &Location,
        new: &Location,
        trans: &mut Transaction,
    ) -> StmResult<f64> {
        let mut no_swap = 0_f64;
        let mut yes_swap = 0_f64;

        for element in &self.fan_in {
            let el = element.read(trans)?;
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;

            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
        }

        for element in &self.fan_out {
            let el = element.read(trans)?;
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.y as isize - el.location.y as isize).abs() as f64;

            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
        }

        Ok(yes_swap - no_swap)
    }
}

#[derive(Clone)]
pub struct Netlist {
    pub elements: Vec<TVar<NetlistElement>>,
    max_x: usize,
    max_y: usize,
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

        println!("Dimensions: [{}, {}]", max_x, max_y);
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
            let element = TVar::new(NetlistElement::new(name.to_string(), cur_x, cur_y));
            elements.push(element.clone());
            // store the link for the second iteration
            tmp.insert(name.to_string(), element);
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
            elements.push(TVar::new(NetlistElement::empty(cur_x, cur_y)));

            // increase the location
            cur_y = (cur_y + 1) % max_y;
            if cur_y == 0 {
                cur_x += 1;
            }
        }

        // println!("[debug] Added empty positions until pos ({},{})", cur_x, cur_y);

        // generate the links
        stm::atomically(|trans| {
            for (elem, links) in elements.iter().zip(to_link.iter()) {
                // let mut mut_elem = elem.borrow_mut();

                for item in links {
                    let link_target = tmp.get(item).expect("All links must be valid");

                    // mut_elem.fan_in.push(link_target.clone());
                    elem.modify(trans, |mut x| {
                        x.fan_in.push(link_target.clone());
                        x
                    })?;
                    link_target.modify(trans, |mut x| {
                        x.fan_out.push(elem.clone());
                        x
                    })?;
                }
            }
            Ok(())
        });

        Ok(Self {
            elements,
            max_x,
            max_y,
        })
    }

    pub fn get_element_by_name(&self, _name: &str) -> Rc<RefCell<NetlistElement>> {
        unimplemented!()
    }

    /// Selects a random pair of different elements from the element list and returns their indices
    pub fn get_random_pair<R: Rng>(&self, rng: &mut R) -> (usize, usize) {
        assert!(self.elements.len() > 1);

        let idx_a = rng.gen_range(0..self.elements.len());
        let mut idx_b = rng.gen_range(0..self.elements.len());

        while idx_a == idx_b {
            idx_b = rng.gen_range(0..self.elements.len());
        }

        (idx_a, idx_b)
    }

    pub fn get_random_element<R: Rng>(&self, different_from: Option<usize>, rng: &mut R) -> usize {
        assert!(self.elements.len() > 1);

        let mut idx = rng.gen_range(0..self.elements.len());
        if let Some(diff) = different_from {
            while idx == diff {
                idx = rng.gen_range(0..self.elements.len());
            }
        }

        idx
    }

    /// Swap the location information for two elements, effectively swapping their positions
    pub fn swap_locations(
        &self,
        idx_a: usize,
        idx_b: usize,
        trans: &mut Transaction,
    ) -> StmResult<()> {
        let mut element_a = self.elements[idx_a].read(trans)?;
        let mut element_b = self.elements[idx_b].read(trans)?;

        std::mem::swap(&mut element_a.location, &mut element_b.location);

        self.elements[idx_a].write(trans, element_a)?;
        self.elements[idx_b].write(trans, element_b)
    }

    /// Shuffle the elements vector by randomly switching out x * y * 1000 pairs
    pub fn shuffle<R: Rng>(&self, rng: &mut R, trans: &mut Transaction) -> StmResult<()> {
        let bounds = self.max_x * self.max_y * 1000;

        for _ in 0..bounds {
            let (a, b) = self.get_random_pair(rng);
            self.swap_locations(a, b, trans)?;
        }

        Ok(())
    }

    /// Count the total routing cost for the netlist.
    pub fn total_routing_cost(&self, trans: &mut Transaction) -> StmResult<f64> {
        let mut cost = 0f64;

        for element in &self.elements {
            cost += element.read(trans)?.routing_cost(trans)?;
        }

        // divide by two since the `routing_cost` function considers both fan-in and fan-out.
        Ok(cost / 2f64)
    }
}
