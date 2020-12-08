use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::{collections::HashMap, rc::Rc};

pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct NetlistElement {
    pub item_name: Option<String>,
    pub fan_in: Vec<Rc<RefCell<NetlistElement>>>,
    pub fan_out: Vec<Rc<RefCell<NetlistElement>>>,
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

    /// Calculates the routing cost using the Manhattan distancee.
    pub fn routing_cost(&self) -> f64 {
        let mut fan_in_cost = 0_f64;
        let mut fan_out_cost = 0_f64;

        for element in &self.fan_in {
            let el = element.borrow();
            fan_in_cost += (self.location.x as isize - el.location.x as isize).abs() as f64;
            fan_in_cost += (self.location.y as isize - el.location.y as isize).abs() as f64;
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

            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
        }

        for element in &self.fan_out {
            let el = element.borrow();
            no_swap += (old.x as isize - el.location.x as isize).abs() as f64;
            no_swap += (old.y as isize - el.location.y as isize).abs() as f64;

            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
            yes_swap += (new.y as isize - el.location.y as isize).abs() as f64;
        }

        yes_swap - no_swap
    }
}

pub struct Netlist {
    elements: Vec<Rc<RefCell<NetlistElement>>>,
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

            println!("[debug] fan in: {:?}", fanins);

            // create the element
            let element = Rc::new(RefCell::new(NetlistElement::new(
                name.to_string(),
                cur_x,
                cur_y,
            )));
            elements.push(element.clone());
            // store the link for the second iteration
            tmp.insert(name.to_string(), element);
            // store the fanins
            to_link.push(fanins);

            // increase the location
            cur_y = cur_y + 1 % max_y;
            if cur_y == 0 {
                cur_x += 1;
            }
        }

        println!("[debug] Filled the chip until pos ({},{})", cur_x, cur_y);

        // fill up the elements vector with the remaining positions
        for _ in elements.len()..chip_size {
            elements.push(Rc::new(RefCell::new(NetlistElement::empty(cur_x, cur_y))));

            // increase the location
            cur_y = cur_y + 1 % max_y;
            if cur_y == 0 {
                cur_x += 1;
            }
        }

        println!("[debug] Filled the chip until pos ({},{})", cur_x, cur_y);

        // generate the links
        for (elem, links) in elements.iter().zip(to_link.drain(..)) {
            let mut mut_elem = elem.borrow_mut();

            for item in links {
                let link_target = tmp.get(&item).expect("All links must be valid");

                mut_elem.fan_in.push(link_target.clone());
                link_target.borrow_mut().fan_out.push(elem.clone());
            }
        }

        Ok(Self { elements })
    }
}
