use crate::element::{Edge, Element};
use crate::mesh::Mesh;
use crate::point::Point;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Connection between two elements. Format: (src, edge, dest)
type Connection = (Rc<RefCell<Element>>, Edge, Rc<RefCell<Element>>);

pub struct Cavity {
    pub center_element: Rc<RefCell<Element>>,
    pub center: Point,

    pub frontier: VecDeque<Rc<RefCell<Element>>>,
    pub previous_nodes: Vec<Rc<RefCell<Element>>>,
    pub new_nodes: Vec<Rc<RefCell<Element>>>,
    pub new_connections: Vec<Connection>,

    /// Connections to elements neighboring the cavity.
    pub connections: Vec<Connection>,

    pub dimension: usize,
}

impl Cavity {
    /// Initializes a new cavity.
    pub fn new(mesh: &Mesh, node: Rc<RefCell<Element>>) -> Self {
        let mut frontier = VecDeque::new();
        let mut previous_nodes = Vec::new();
        let new_nodes = Vec::new();

        let mut center_element = node;

        // TODO(feliix42): What if mesh.contains(node) fails?
        while mesh.contains(&center_element) && center_element.borrow().obtuse_angle.is_some() {
            center_element = get_opposite(center_element);
        }

        let center = center_element.borrow().get_center();
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

    // TODO(feliix42): Maybe return `Result<(), Rc<RefCell<Element>>` instead to fix the self-overwriting thingy?
    fn expand(&mut self, curr: Rc<RefCell<Element>>, next: &Rc<RefCell<Element>>) {
        let next_inner = next.borrow();

        if !(self.dimension == 2
            && next_inner.coordinates.len() == 2
            && next != &self.center_element)
            && next_inner.in_circle(self.center)
        {
            // part of the cavity and we're not the second segment encroaching on this cavity
            if next_inner.coordinates.len() == 2 && self.dimension != 2 {
                // is segment and we're encroaching
                // self = Self::new();
                // self.build();
                todo!()
            } else {
                if !self.previous_nodes.contains(&next) {
                    self.previous_nodes.push(next.clone());
                    self.frontier.push_back(next.clone());
                }
            }
        } else {
            // not a member
            let curr_inner = curr.borrow();
            let edge = next_inner.get_related_edge(&curr_inner).unwrap();

            let connection = (curr.clone(), edge, next.clone());
            if !self.connections.contains(&connection) {
                self.connections.push(connection);
            }
        }
    }

    /// Expand the cavity to cover all related elements.
    pub fn build(&mut self) {
        while !self.frontier.is_empty() {
            let curr = self.frontier.pop_back().unwrap();
            let curr_inner = curr.borrow();

            for neighbor in &curr_inner.neighbors {
                self.expand(curr.clone(), neighbor);
            }
        }
    }

    /// Compute a corrected cavity
    pub fn compute(&mut self) {
        if self.dimension == 2 {
            // we've actually built around a segment (or an edge)
            let c = self.center_element.borrow();
            let n1 = Rc::new(RefCell::new(Element::new_line(
                self.center,
                c.coordinates[0],
            )));
            let n2 = Rc::new(RefCell::new(Element::new_line(
                self.center,
                c.coordinates[1],
            )));

            self.new_nodes.push(n1);
            self.new_nodes.push(n2);
        }

        for conn in &self.connections {
            // Connection structure: (source, shared edge, destination)
            let ele = Element::new_poly(self.center, conn.1 .0, conn.1 .1);
            let other = if self.previous_nodes.contains(&conn.2) {
                // if the destination is contained in previous nodes, go for the source
                conn.0.clone()
            } else {
                conn.2.clone()
            };

            let other_edge = ele.get_related_edge(&other.borrow()).unwrap();
            let ele_wrapped = Rc::new(RefCell::new(ele));
            self.new_connections
                .push((ele_wrapped.clone(), other_edge, other));

            for element in &self.new_nodes {
                let el = element.borrow();
                let ele = ele_wrapped.borrow();
                if el.is_related_to(&ele) {
                    let additional_edge = ele.get_related_edge(&el).unwrap();
                    self.new_connections.push((
                        ele_wrapped.clone(),
                        additional_edge,
                        element.clone(),
                    ));
                }
            }

            self.new_nodes.push(ele_wrapped);
        }
    }
}

/// Find the node that is opposite to the obtuse angle of the element.
fn get_opposite(node: Rc<RefCell<Element>>) -> Rc<RefCell<Element>> {
    let inner = node.borrow();
    let obtuse_pt = inner.get_obtuse();

    for neighbor in &inner.neighbors {
        // get related edge
        if let Some(related_edge) = inner.get_related_edge(&neighbor.borrow()) {
            // if points of the edge don't match obtuse point, return neighbor
            if obtuse_pt != related_edge.0 && obtuse_pt != related_edge.1 {
                return neighbor.clone();
            }
        }
    }

    std::mem::drop(inner);
    node
}
