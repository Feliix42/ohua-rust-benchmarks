//! In the original codebase, this was `mesh.c`

use crate::cavity::Cavity;
use crate::element::{Edge, Element};
use crate::point::Point;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

pub struct Mesh {
    pub elements: Vec<Rc<RefCell<Element>>>,
    //root: Rc<RefCell<Element>>,
    //initial_bad_queue: VecDeque<()>,
    //size: usize,
    //boundary_set: TODO
}

impl Mesh {
    pub fn load_from_file(filename_prefix: &str) -> std::io::Result<Self> {
        // load *.node file
        let node_file = File::open(format!("{}.node", filename_prefix))?;
        let mut node_reader = BufReader::new(node_file);

        let mut buf = String::new();
        node_reader.read_line(&mut buf)?;
        let node_header: Vec<usize> = buf
            .split_whitespace()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        let entry_count = node_header[0]; // this is both numEntry and numCoordinate (because why??)
        let num_dimensions = node_header[1];
        assert_eq!(num_dimensions, 2);

        let mut coordinates = Vec::with_capacity(entry_count);
        for line in node_reader.lines() {
            let line = line?;
            // skip comments
            if line.starts_with("#") {
                continue;
            }

            let coords: Vec<f64> = line
                .split_whitespace()
                .skip(1)
                .take(2)
                .map(f64::from_str)
                .map(Result::unwrap)
                .collect();

            coordinates.push(Point {
                x: coords[0],
                y: coords[1],
            });
        }

        assert_eq!(coordinates.len(), entry_count);

        // load the *.poly file containing lines (?)
        let poly_file = File::open(format!("{}.poly", filename_prefix))?;
        let mut poly_reader = BufReader::new(poly_file);

        buf.clear();
        poly_reader.read_line(&mut buf)?;
        let poly_header: Vec<usize> = buf
            .split_whitespace()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        assert_eq!(poly_header[0], 0);
        assert_eq!(poly_header[1], 2);
        // line 2 gives us the total element #
        poly_reader.read_line(&mut buf)?;
        let poly_header: Vec<usize> = buf
            .split_whitespace()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        let num_segments = poly_header[0];

        let mut elems = Vec::with_capacity(num_segments);
        for line in poly_reader.lines() {
            // may be unnecessary
            //if num_segments == 0 {
            //break;
            //}
            let line = line?;

            // skip comments
            if line.starts_with("#") {
                continue;
            }

            let coords: Vec<usize> = line
                .split_whitespace()
                .skip(1)
                .take(2)
                .map(usize::from_str)
                .map(Result::unwrap)
                .collect();

            if coords.is_empty() {
                continue;
            }
            // off by one possible here?
            assert!(coords[0] < entry_count);
            assert!(coords[1] < entry_count);

            // they count items from 1 for some reason
            let c_0 = coordinates[coords[0] - 1];
            let c_1 = coordinates[coords[1] - 1];
            elems.push(Rc::new(RefCell::new(Element::new_line(c_0, c_1))));
        }

        // load the *.ele file
        let ele_file = File::open(format!("{}.ele", filename_prefix))?;
        let mut ele_reader = BufReader::new(ele_file);

        buf.clear();
        ele_reader.read_line(&mut buf)?;
        let ele_header: Vec<usize> = buf
            .split_whitespace()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        let num_elements = ele_header[0];

        elems.reserve(num_elements);
        for line in ele_reader.lines() {
            let line = line?;

            if line.starts_with("#") {
                continue;
            }

            let coords: Vec<usize> = line
                .split_whitespace()
                .skip(1)
                .take(3)
                .map(usize::from_str)
                .map(Result::unwrap)
                .collect();
            assert!(coords[0] <= entry_count);
            assert!(coords[1] <= entry_count);
            assert!(coords[2] <= entry_count);

            // they count items from 1 for some reason
            let c_0 = coordinates[coords[0] - 1];
            let c_1 = coordinates[coords[1] - 1];
            let c_2 = coordinates[coords[2] - 1];
            elems.push(Rc::new(RefCell::new(Element::new_poly(c_0, c_1, c_2))));
        }

        // establish neighboring relations
        let mut edge_map: HashMap<Edge, Rc<RefCell<Element>>> = HashMap::new();
        for elem in &elems {
            let mut inner = elem.borrow_mut();
            if inner.num_coordinates == 2 {
                let e = inner.get_edge(0);
                if let Some(other) = edge_map.remove(&e) {
                    // found edge, link together
                    other.borrow_mut().neighbors.push(elem.clone());
                    inner.neighbors.push(other);
                } else {
                    // insert
                    edge_map.insert(e, elem.clone());
                }
            } else {
                for i in 0..3 {
                    let e = inner.get_edge(i);
                    if let Some(other) = edge_map.remove(&e) {
                        // found edge, link together
                        other.borrow_mut().neighbors.push(elem.clone());
                        inner.neighbors.push(other);
                    } else {
                        // insert
                        edge_map.insert(e, elem.clone());
                    }
                }
            }
        }

        Ok(Mesh { elements: elems })
    }

    pub fn find_bad(&self) -> VecDeque<Rc<RefCell<Element>>> {
        let mut r = VecDeque::new();

        for elem in &self.elements {
            if elem.borrow().is_bad() {
                r.push_back(elem.clone());
            }
        }

        r
    }

    /// Tests whether `node` is contained in the graph.
    pub fn contains(&self, node: &Rc<RefCell<Element>>) -> bool {
        self.elements.contains(node)
    }

    /// Update the mesh with the data of a corrected cavity. (Original code implements this in `Cavity.h`)
    ///
    /// Returns a list of new bad elements
    pub fn update(
        &mut self,
        cav: Cavity,
        original_bad: Rc<RefCell<Element>>,
    ) -> VecDeque<Rc<RefCell<Element>>> {
        // here we'd probably have to check if all elements of the `previous_nodes`
        // are still in the mesh before updating when doing this in parallel.
        // remove old elements
        for old in cav.previous_nodes.into_iter() {
            let pos = self.elements.binary_search(&old).unwrap();
            self.elements.remove(pos);
        }

        // add new data
        let mut new_bad = VecDeque::new();
        for new_node in cav.new_nodes.into_iter() {
            self.elements.push(new_node.clone());

            if new_node.borrow().is_bad() {
                new_bad.push_back(new_node);
            }
        }

        // this `new_edge` is somewhat unnecessary I reckon, but that was part of the original code.
        // I'm now also realizing that this could probably be put into the
        // `compute` function of `cavity`, but thinking ahead it's probably
        // better kept here to make matters simpler when going for the parallel
        // versions.
        for (src, _, dst) in cav.new_connections.into_iter() {
            src.borrow_mut().neighbors.push(dst.clone());
            dst.borrow_mut().neighbors.push(src);
        }

        // if the original "bad element" is still in the mesh that's still a todo
        if self.contains(&original_bad) {
            new_bad.push_back(original_bad);
        }

        new_bad
    }

    pub fn refine(&mut self, mut bad: VecDeque<Rc<RefCell<Element>>>) {
        println!("Current number of bad elements: {}", bad.len());
        let mut i = 0;
        while !bad.is_empty() {
            i += 1;
            let item = bad.pop_front().unwrap();
            if !self.contains(&item) {
                continue;
            }

            let mut cav = Cavity::new(&self, item.clone());
            cav.build();
            cav.compute();
            bad.append(&mut self.update(cav, item));

            if i >= 100 {
                println!("Current number of bad elements: {}", bad.len());
                i = 0;
            }
        }
    }
}
