use crate::element::{Edge, Element, Triangle};
use crate::mesh::Mesh;
use crate::point::Point;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Connection between two elements. Format: (src, edge, dest)
type Connection = (Element, Edge, Element);

pub struct Cavity {
    pub center_element: Triangle,
    pub center: Point,

    pub frontier: VecDeque<Element>,
    pub previous_nodes: Vec<Element>,
    pub new_nodes: Vec<Element>,
    pub new_connections: Vec<Connection>,

    /// Connections to elements neighboring the cavity.
    pub connections: Vec<Connection>,

    pub dimension: usize,
}

impl Cavity {
    /// Initializes a new cavity.
    pub fn new(mesh: &Mesh, node: Triangle) -> Self {
        let mut frontier = VecDeque::new();
        let mut previous_nodes = Vec::new();
        let new_nodes = Vec::new();

        let mut center_element = node;

        // TODO(feliix42): What if mesh.contains(node) fails?
        let mut circ = Vec::new();
        while mesh.contains_triangle(&center_element) && center_element.obtuse_angle.is_some() {
            circ.push(center_element.clone());
            let new_center = get_opposite(&center_element);
            // the original code did not handle loops at all (i.e. 2 triangles that share an opposite side)
            if circ.contains(&new_center) {
                break;
            } else {
                center_element = new_center;
            }
        }

        let center = center_element.borrow().get_center();
        // println!("Center: {:?}", center);
        // println!("Coordinates: {:?}", center_element.borrow().coordinates);
        let dimension = center_element.borrow().coordinates.len();
        frontier.push_back(center_element.clone());
        previous_nodes.push(center_element.clone());

        Cavity {
            center_element,
            center,
            frontier,
            previous_nodes,
            new_nodes,
            new_connections: Vec::new(),
            connections: Vec::new(),
            dimension,
        }
    }

    // fn expand(
    //     &mut self,
    //     curr: Rc<RefCell<Element>>,
    //     next: &Rc<RefCell<Element>>,
    // ) -> Result<(), Rc<RefCell<Element>>> {
    //     let next_inner = next.borrow();

    //     if !(self.dimension == 2
    //         && next_inner.coordinates.len() == 2
    //         && next != &self.center_element)
    //         && next_inner.in_circle(self.center)
    //     {
    //         // part of the cavity and we're not the second segment encroaching on this cavity
    //         if next_inner.coordinates.len() == 2 && self.dimension != 2 {
    //             // is segment and we're encroaching
    //             return Err(next.clone());
    //         } else {
    //             if !self.previous_nodes.contains(&next) {
    //                 // println!("Adding {:?} as node", next_inner.coordinates);
    //                 self.previous_nodes.push(next.clone());
    //                 self.frontier.push_back(next.clone());
    //             }
    //         }
    //     } else {
    //         // not a member
    //         let curr_inner = curr.borrow();
    //         let edge = next_inner.get_related_edge(&curr_inner).unwrap();

    //         let connection = (curr.clone(), edge, next.clone());
    //         if !self.connections.contains(&connection) {
    //             self.connections.push(connection);
    //         }
    //     }

    //     Ok(())
    // }

    // /// Expand the cavity to cover all related elements.
    // pub fn build(&mut self, mesh: &Mesh) {
    //     while !self.frontier.is_empty() {
    //         let curr = self.frontier.pop_back().unwrap();
    //         let curr_inner = curr.borrow();

    //         for neighbor in &curr_inner.neighbors {
    //             if let Err(other) = self.expand(curr.clone(), neighbor) {
    //                 *self = Self::new(mesh, other);
    //             }
    //         }
    //     }
    // }

    // /// Compute a corrected cavity
    // pub fn compute(&mut self) {
    //     if self.dimension == 2 {
    //         // we've actually built around a segment (or an edge)
    //         let c = self.center_element.borrow();
    //         let n1 = Rc::new(RefCell::new(Element::new_line(
    //             self.center,
    //             c.coordinates[0],
    //         )));
    //         let n2 = Rc::new(RefCell::new(Element::new_line(
    //             self.center,
    //             c.coordinates[1],
    //         )));

    //         self.new_nodes.push(n1);
    //         self.new_nodes.push(n2);
    //     }

    //     let mut circtest = Vec::new();
    //     for conn in &self.connections {
    //         if circtest.contains(conn) {
    //             panic!("ALARM");
    //         }
    //         circtest.push(conn.clone());
    //         assert_ne!(conn.1 .0, conn.1 .1);
    //         let ele = Element::new_poly(self.center, (conn.1).0, (conn.1).1);
    //         let other = if self.previous_nodes.contains(&conn.2) {
    //             // if the destination is contained in previous nodes, go for the source
    //             conn.0.clone()
    //         } else {
    //             conn.2.clone()
    //         };

    //         let other_edge = match ele.get_related_edge(&other.borrow()) {
    //             Some(e) => e,
    //             None => panic!(
    //                 "There is no related edge between the following coordinates:\n{:#?}\n{:#?}\nMatch check: {}\nEquality? {}\nCenter: {:?}",
    //                 ele.coordinates,
    //                 other.borrow().coordinates,
    //                 ele.is_related_to(&other.borrow()),
    //                 ele == *other.borrow(),
    //                 self.center
    //             ),
    //         };
    //         let ele_wrapped = Rc::new(RefCell::new(ele));
    //         // Connection structure: (source, shared edge, destination)
    //         self.new_connections
    //             .push((ele_wrapped.clone(), other_edge, other));

    //         for element in &self.new_nodes {
    //             let el = element.borrow();
    //             let ele = ele_wrapped.borrow();
    //             if el.is_related_to(&ele) {
    //                 let additional_edge = ele.get_related_edge(&el).unwrap();
    //                 self.new_connections.push((
    //                     ele_wrapped.clone(),
    //                     additional_edge,
    //                     element.clone(),
    //                 ));
    //             }
    //         }

    //         self.new_nodes.push(ele_wrapped);
    //     }
    // }
}
