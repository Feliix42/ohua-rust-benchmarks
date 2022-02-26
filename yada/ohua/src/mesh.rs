//! In the original codebase, this was `mesh.c`

use crate::cavity::Cavity;
use crate::element::{Edge, Element, Triangle};
use crate::point::Point;
use decorum::R64;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use std::sync::Arc;

#[derive(Clone)]
pub struct Mesh {
    /// Triangles in the mesh with links to neighboring elements.
    /// The vector will **always** have 3 elements.
    ///
    /// The reason for this being a `Vec<Element>` instead of `[Element; 3]` is
    /// simply ergonimics: It's infinitely simpler to just append/remove elements
    /// from a vector than an array. This means that in the two places where the
    /// array is actually modified, the relaxed constraint allows the simple
    /// removal of elements without having to use indirections like casting the
    /// array to an array of `Option<Element>` etc.
    pub elements: Arc<HashMap<Triangle, Vec<Element>>>,
    // pub elements: HashMap<Triangle, [Element; 3]>,
    pub boundary_set: Arc<HashMap<Edge, Element>>,

    pub computation_steps: usize,
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
            if line.starts_with('#') {
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
                x: R64::from_inner(coords[0]),
                y: R64::from_inner(coords[1]),
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

        let mut edge_set = HashSet::with_capacity(num_segments);
        for line in poly_reader.lines() {
            // may be unnecessary
            //if num_segments == 0 {
            //break;
            //}
            let line = line?;

            // skip comments
            if line.starts_with('#') {
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
            assert!(coords[0] <= entry_count);
            assert!(coords[1] <= entry_count);

            // they count items from 1 for some reason
            let c_0 = coordinates[coords[0]];
            let c_1 = coordinates[coords[1]];
            edge_set.insert(Edge::new(c_0, c_1));
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

        let mut elem_vec = Vec::with_capacity(num_elements);
        for line in ele_reader.lines() {
            let line = line?;

            if line.starts_with('#') {
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
            let c_0 = coordinates[coords[0]];
            let c_1 = coordinates[coords[1]];
            let c_2 = coordinates[coords[2]];
            elem_vec.push(Triangle::new(c_0, c_1, c_2));
        }

        let mut edges: HashMap<Edge, Element> = HashMap::with_capacity(num_segments);
        let mut elems: HashMap<Triangle, Vec<Element>> = HashMap::with_capacity(num_elements);

        // establish neighboring relations
        let mut triangle_map: HashMap<Edge, Triangle> = HashMap::new();
        // NOTE: We assume here that the input files are properly structured, i.e., that no edge is neighbor to another edge

        for elem in elem_vec {
            for i in 0..3 {
                let e = elem.get_edge(i);

                if edge_set.contains(&e) {
                    // edge is shared with an outer edge -> establish link
                    // insert the item in the linked list
                    let item = elems.entry(elem).or_default();
                    item.push(e.into());

                    // insert backlink from the edge
                    // NOTE(feliix42): could be `assert!()`ed as `is_none()`
                    edges.insert(e, elem.into());
                } else if let Some(other) = triangle_map.remove(&e) {
                    let item = elems.entry(elem).or_default();
                    item.push(other.into());

                    let o_mut = elems.entry(other).or_default();
                    o_mut.push(elem.into());
                } else {
                    triangle_map.insert(e, elem);
                }
            }
        }
        // verify correctness
        #[cfg(verify)]
        {
            assert_eq!(edges.len(), edge_set.len());
            let _ = elems
                .iter()
                .map(|(_, v)| assert_eq!(v.len(), 3))
                .collect::<()>();
        }

        Ok(Mesh {
            elements: Arc::new(elems),
            boundary_set: Arc::new(edges),
            computation_steps: 0,
        })
    }

    pub fn find_bad(&self) -> Vec<Triangle> {
        let mut r = Vec::new();

        for elem in self.elements.keys() {
            if elem.is_bad() {
                r.push(elem.to_owned());
            }
        }

        r
    }

    /// Tests whether `node` is contained in the graph.
    pub fn contains(&self, node: &Element) -> bool {
        match node {
            Element::E(ref e) => self.boundary_set.contains_key(e),
            Element::T(ref t) => self.elements.contains_key(t),
        }
    }

    /// Tests whether `node` is contained in the graphs triangle set.
    pub fn contains_triangle(&self, node: &Triangle) -> bool {
        self.elements.contains_key(node)
    }

    /// Find the node that is opposite to the obtuse angle of the element.
    #[allow(dead_code)]
    pub fn get_opposite(&self, node: &Triangle) -> Element {
        let opposite_edge = node.get_opposite_edge();

        for neighbor in self.elements.get(node).unwrap() {
            match neighbor {
                Element::T(ref t) => {
                    // get related edge
                    if let Some(related_edge) = node.get_related_edge(t) {
                        // if points of the edge don't match obtuse point, return neighbor
                        if related_edge == opposite_edge {
                            return *neighbor;
                        }
                        // if !related_edge.contains(obtuse_pt) {
                        //     return *neighbor;
                        // }
                    }
                }
                Element::E(ref e) => {
                    if *e == opposite_edge {
                        return *neighbor;
                    }
                }
            }
        }

        unreachable!()
    }

    // NOTE(feliix42): So this one is fun: It can happen that a cavity is started
    // from an edge (only happens when the cavity is initialized and overwrites
    // itself). But apparently this is never translated to this stage of execution.
    //
    // It seems like that may not be a problem after all since the last few lines in the function are handling this.
    /// Update the mesh with the data of a corrected cavity. (Original code implements this in `Cavity.h`)
    ///
    /// Returns a list of new bad elements
    pub fn update(&mut self, cav: Cavity, original_bad: Triangle) -> VecDeque<Triangle> {
        let elems = Arc::get_mut(&mut self.elements).unwrap();
        let bounds = Arc::get_mut(&mut self.boundary_set).unwrap();
        // println!("Update: Replacing {} elements with {} elements.\nOld set:\n{:#?}\n--------------\nNew set:\n{:#?}", cav.previous_nodes.len(), cav.new_nodes.len(), cav.previous_nodes, cav.new_nodes);
        // std::thread::sleep_ms(1_000);

        // here we'd probably have to check if all elements of the `previous_nodes`
        // are still in the mesh before updating when doing this in parallel.
        // println!("Previous nodes: {}", cav.previous_nodes.len(),);

        // remove old elements
        // let mut failed = 0;
        // println!(
        //     "Is original bad in prev nodes? {}",
        //     cav.previous_nodes.contains(&Element::T(original_bad))
        // );
        for old in cav.previous_nodes {
            // print!("Searching {:?}", old.borrow().coordinates);
            match old {
                Element::T(ref t) => {
                    elems.remove(t);
                }
                Element::E(ref e) => {
                    bounds.remove(e);
                }
            }
        }
        // assert!(failed == 0, "Failed in removing {} elements", failed);

        //println!(
        //    "old connections: {}, new connections: {}",
        //    cav.connections.len(),
        //    cav.new_connections.len()
        //);

        // prune old connections!
        for (old, _, outer) in cav.connections {
            match outer {
                Element::T(ref t) => {
                    if let Some(neighborhood) = elems.get_mut(t) {
                        let _ = neighborhood
                            .drain_filter(|&mut x| x == old)
                            .collect::<Vec<_>>();
                    }
                }
                Element::E(ref e) => {
                    // we can just remove the edge because the new neighboring relation will add it again.
                    bounds.remove(e);
                }
            }
        }

        // add new data
        let mut new_bad = VecDeque::new();
        for new_node in cav.new_nodes {
            match new_node {
                Element::T(t) => {
                    elems.insert(t, Vec::with_capacity(3));
                    if t.is_bad() {
                        // println!("Appending triangle with area: {}", t.area());
                        new_bad.push_back(t);
                    }
                }
                Element::E(_) => (),
            }
        }

        // this `new_edge` is somewhat unnecessary I reckon, but that was part of the original code.
        // I'm now also realizing that this could probably be put into the
        // `compute` function of `cavity`, but thinking ahead it's probably
        // better kept here to make matters simpler when going for the parallel
        // versions.
        for (src, _, dst) in cav.new_connections {
            match src {
                Element::T(ref t) => {
                    let neighborhood = elems.get_mut(t).expect("Element does not exist");
                    neighborhood.push(dst);
                }
                Element::E(e) => {
                    bounds.insert(e, dst);
                }
            }

            match dst {
                Element::T(ref t) => {
                    let neighborhood = elems.get_mut(t).expect("Element does not exist");
                    neighborhood.push(src);
                }
                Element::E(e) => {
                    bounds.insert(e, src);
                }
            }
        }

        // if the original "bad element" is still in the mesh that's still a todo
        if elems.contains_key(&original_bad) {
            // println!("Ah shit here we go again");
            new_bad.push_back(original_bad);
        }

        // verify correctness
        #[cfg(verify)]
        {
            let _ = elems
                .iter()
                .map(|(_, v)| assert_eq!(v.len(), 3))
                .collect::<()>();
        }

        // println!("-- Done");

        new_bad
    }

    pub fn apply_updates(&mut self, cavities: Vec<Option<Cavity>>) -> Vec<Triangle> {
        let mut deleted_items: HashSet<Element> = HashSet::new();
        let mut old_work_items = Vec::new();
        let mut new_work_items = Vec::new();

        self.computation_steps += cavities.len();

        for cavity in cavities {
            if let Some(cav) = cavity {
                // if either:
                // a) a replaced element has already been touched or
                // b) a connection element is no longer in the mesh
                // -> Try again next iteration!
                if cav
                    .previous_nodes
                    .iter()
                    .any(|it| deleted_items.contains(it))
                    || cav
                        .connections
                        .iter()
                        .map(|(_, _, dst)| dst)
                        .any(|it| !self.contains(it))
                {
                    // we already changed this one
                    old_work_items.push(cav.original_center);
                } else {
                    // add items
                    deleted_items.extend(cav.previous_nodes.iter().copied());
                    // add elements
                    let orig = cav.original_center;
                    let new_work = self.update(cav, orig);
                    new_work_items.extend(new_work.into_iter());
                }
            }
        }

        old_work_items.append(&mut new_work_items);

        old_work_items
    }

    pub fn has_more_work(&self) -> bool {
        !self.find_bad().is_empty()
    }
}