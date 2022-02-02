//! In the original codebase, this was `mesh.c`

// use crate::cavity::Cavity;
use crate::element::{Edge, Element, Triangle};
use crate::point::Point;
use decorum::R64;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Mesh {
    pub elements: HashMap<Triangle, [Element; 3]>,
    //root: Rc<RefCell<Element>>,
    //initial_bad_queue: VecDeque<()>,
    //size: usize,
    pub boundary_set: HashMap<Edge, Element>,
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
            assert!(coords[0] <= entry_count);
            assert!(coords[1] <= entry_count);

            // they count items from 1 for some reason
            let c_0 = coordinates[coords[0] - 1];
            let c_1 = coordinates[coords[1] - 1];
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
            elem_vec.push(Triangle::new(c_0, c_1, c_2));
        }

        let mut edges: HashMap<Edge, Element> = HashMap::with_capacity(num_segments);
        let mut elems: HashMap<Triangle, [Option<Element>; 3]> =
            HashMap::with_capacity(num_elements);

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
                    item[0] = Some(e.into());
                    item.rotate_left(1);

                    // insert backlink from the edge
                    // NOTE(feliix42): could be `assert!()`ed as `is_none()`
                    edges.insert(e, elem.into());
                } else if let Some(other) = triangle_map.remove(&e) {
                    let item = elems.entry(elem).or_default();
                    item[0] = Some(other.into());
                    item.rotate_left(1);

                    let o_mut = elems.entry(other).or_default();
                    o_mut[0] = Some(elem.into());
                    o_mut.rotate_left(1);
                } else {
                    triangle_map.insert(e, elem.clone());
                }
            }
        }
        // verify correctness
        assert_eq!(edges.len(), edge_set.len());
        let _ = elems
            .iter()
            .map(|(_, v)| assert!(v.iter().any(Option::is_none)))
            .collect::<()>();

        Ok(Mesh {
            elements: elems
                .into_iter()
                .map(|(k, v)| {
                    let [a, b, c] = v;
                    (k, [a.unwrap(), b.unwrap(), c.unwrap()])
                })
                .collect(),
            boundary_set: edges,
        })
    }

    pub fn find_bad(&self) -> VecDeque<Triangle> {
        let mut r = VecDeque::new();

        for elem in self.elements.keys() {
            if elem.is_bad() {
                r.push_back(elem.to_owned());
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

    // TODO: Continue with this function: Should it take a `Triangle`? I'm confused since the original code does not distinguish between edge & triangle but a edge can't have an obtuse angle...
    /// Find the node that is opposite to the obtuse angle of the element.
    fn get_opposite(&self, node: &Element) -> Element {
        let obtuse_pt = node.get_obtuse();

        for neighbor in self.elements.get(node).unwrap() {
            // get related edge
            if let Some(related_edge) = node.get_related_edge(&neighbor.borrow()) {
                // if points of the edge don't match obtuse point, return neighbor
                if obtuse_pt != related_edge.0 && obtuse_pt != related_edge.1 {
                    return neighbor.clone();
                }
            }
        }

        unreachable!()
        //std::mem::drop(inner);
        //node
    }

    // /// Update the mesh with the data of a corrected cavity. (Original code implements this in `Cavity.h`)
    // ///
    // /// Returns a list of new bad elements
    // pub fn update(
    //     &mut self,
    //     cav: Cavity,
    //     original_bad: Rc<RefCell<Element>>,
    // ) -> VecDeque<Rc<RefCell<Element>>> {
    //     // here we'd probably have to check if all elements of the `previous_nodes`
    //     // are still in the mesh before updating when doing this in parallel.
    //     // remove old elements
    //     // println!("Previous nodes: {}", cav.previous_nodes.len(),);
    //     let mut failed = 0;
    //     for old in cav.previous_nodes.into_iter() {
    //         // print!("Searching {:?}", old.borrow().coordinates);
    //         if let Some(pos) = find_pos(&self.elements, &old) {
    //             self.elements.remove(pos);
    //             // println!(" - found");
    //         } else {
    //             if self.elements.contains(&old) {
    //                 panic!("Element was already removed??");
    //             }
    //             // println!(" - nope");
    //             failed += 1;
    //         }
    //     }
    //     assert!(failed == 0, "Failed in removing {} elements", failed);

    //     //println!(
    //     //    "old connections: {}, new connections: {}",
    //     //    cav.connections.len(),
    //     //    cav.new_connections.len()
    //     //);

    //     // prune old connections!
    //     for (old, _, outer) in cav.connections.into_iter() {
    //         let mut o_inner = outer.borrow_mut();

    //         if let Some(pos) = find_pos(&o_inner.neighbors, &old) {
    //             o_inner.neighbors.remove(pos);
    //         } else {
    //             panic!("delete w/o success");
    //         }
    //     }

    //     // add new data
    //     let mut new_bad = VecDeque::new();
    //     for new_node in cav.new_nodes.into_iter() {
    //         self.elements.push(new_node.clone());

    //         if new_node.borrow().is_bad() {
    //             new_bad.push_back(new_node);
    //         }
    //     }

    //     // this `new_edge` is somewhat unnecessary I reckon, but that was part of the original code.
    //     // I'm now also realizing that this could probably be put into the
    //     // `compute` function of `cavity`, but thinking ahead it's probably
    //     // better kept here to make matters simpler when going for the parallel
    //     // versions.
    //     for (src, _, dst) in cav.new_connections.into_iter() {
    //         src.borrow_mut().neighbors.push(dst.clone());
    //         dst.borrow_mut().neighbors.push(src);
    //     }

    //     // if the original "bad element" is still in the mesh that's still a todo
    //     if self.contains(&original_bad) {
    //         new_bad.push_back(original_bad);
    //     }

    //     new_bad
    // }

    // pub fn refine(&mut self, mut bad: VecDeque<Triangle>) {
    //     // println!("Current number of bad elements: {}", bad.len());
    //     let mut i = 0;
    //     while !bad.is_empty() {
    //         i += 1;
    //         let item = bad.pop_front().unwrap();
    //         if !self.contains(&item) {
    //             continue;
    //         }

    //         let mut cav = Cavity::new(&self, item.clone());
    //         cav.build(&self);
    //         cav.compute();
    //         //println!("Created {} new elements", cav.new_nodes.len());
    //         let mut result = self.update(cav, item);
    //         //println!("Got {} new bad items", result.len());
    //         bad.append(&mut result);

    //         if (i % 10000) == 0 {
    //             println!("Iteration: {}, bad elements: {}", i, bad.len());
    //         }
    //         //if i >= 100 {
    //         //    println!("Current number of bad elements: {}", bad.len());
    //         //    i = 0;
    //         //}
    //     }

    //     println!("Did {} iterations", i);
    // }
}

// fn find_pos(list: &Vec<Rc<RefCell<Element>>>, other: &Rc<RefCell<Element>>) -> Option<usize> {
//     let mut idx = 0;
//     for elem in list {
//         if elem == other {
//             return Some(idx);
//         } else {
//             idx += 1;
//         }
//     }

//     None
// }
