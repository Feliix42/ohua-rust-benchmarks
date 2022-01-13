//! In the original codebase, this was `element.c`

use crate::point::Point;
use std::cell::RefCell;
use std::rc::Rc;

const MIN_ANGLE: f64 = 30.0;

pub type Edge = (Point, Point);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Element {
    /// The coordinates of the element.
    pub coordinates: Vec<Point>,
    /// The number of coordinates stored. Two make it a line, three a triangle.
    pub num_coordinates: usize,

    /// Neighboring elements
    pub neighbors: Vec<Rc<RefCell<Element>>>,

    /// The triangle may have exactly one obtuse angle. If so, this variable defines which is the
    /// one.
    pub obtuse_angle: Option<usize>,
    //circum_center: Point,
    //circum_radius: f64,
    //edges: (Edge, Edge, Edge),
    //num_edges: usize,
    ///// midpoint of each edge
    //midpoints: (Point, Point, Point),
    ///// half of edge length
    //radii: (f64, f64, f64),
    ///// opposite obtuse angle
    //encroached_edge: Edge,
    //is_skinny: bool,
    //neighbor_list: Vec<Rc<RefCell<Element>>>,
    //is_garbage: bool,
    //is_referenced: bool
}

impl Element {
    pub fn new_line(p1: Point, p2: Point) -> Self {
        Element {
            coordinates: vec![p1, p2],
            num_coordinates: 2,
            neighbors: Vec::with_capacity(1),
            obtuse_angle: None,
        }
    }

    pub fn new_poly(p1: Point, p2: Point, p3: Point) -> Self {
        let coordinates = vec![p1, p2, p3];
        let mut obtuse = None;

        for i in 0..3 {
            let j = i + 1 % 3;
            let k = i + 2 % 3;
            if coordinates[j].angle_is_obtuse(&coordinates[i], &coordinates[k]) {
                obtuse = Some(i);
            }
        }

        Element {
            coordinates,
            num_coordinates: 3,
            neighbors: Vec::with_capacity(3),
            obtuse_angle: obtuse,
        }
    }

    /// Determines whether the node needs to be processed
    pub fn is_bad(&self) -> bool {
        if self.num_coordinates != 3 {
            false
        } else {
            for i in 0..self.coordinates.len() {
                let ang = self.coordinates[i].angle(
                    &self.coordinates[(i + 1) % self.num_coordinates],
                    &self.coordinates[(i + 2) % self.num_coordinates],
                );

                if ang < MIN_ANGLE {
                    return true;
                }
            }

            false
        }
    }

    pub fn get_center(&self) -> Point {
        if self.num_coordinates == 2 {
            (self.coordinates[0] + self.coordinates[1]) * 0.5
        } else {
            let a = &self.coordinates[0];
            let b = &self.coordinates[1];
            let c = &self.coordinates[2];

            let x = b - a;
            let y = c - a;
            let x_len = a.distance_to(b);
            let y_len = a.distance_to(c);
            let cosine = (x * y) / (x_len * y_len);
            let sine_sq = 1.0 - cosine * cosine;
            let p_len = y_len / x_len;

            let s = p_len * cosine;
            let t = p_len * sine_sq;

            let wp = (p_len - cosine) / (2f64 * t);
            let wb = 0.5 - (wp * s);

            let mut tmp_val = *a * (1f64 - wb - wp);
            tmp_val = tmp_val + (*b * wb);
            tmp_val + (*c * wp)
        }
    }

    /// Get the edge with the assigned number
    pub fn get_edge(&self, i: usize) -> Edge {
        if i == 0 {
            (self.coordinates[0], self.coordinates[1])
        } else {
            if self.num_coordinates == 2 {
                if i == 1 {
                    (self.coordinates[1], self.coordinates[0])
                } else {
                    // error case
                    (self.coordinates[0], self.coordinates[0])
                }
            } else {
                match i {
                    1 => (self.coordinates[1], self.coordinates[2]),
                    2 => (self.coordinates[2], self.coordinates[0]),
                    // error case
                    _ => (self.coordinates[0], self.coordinates[0]),
                }
            }
        }
    }

    pub fn get_obtuse(&self) -> Point {
        if let Some(i) = self.obtuse_angle {
            self.coordinates[i]
        } else {
            panic!("Cannot retrieve obtuse point from line because it has none.");
        }
    }

    /// Returns `true` if both elements share an edge
    pub fn is_related_to(&self, other: &Element) -> bool {
        let mut num_matching_points = 0;

        for pt in &self.coordinates {
            for pt2 in &other.coordinates {
                if pt == pt2 {
                    num_matching_points += 1;
                }
            }
        }

        num_matching_points == 2
    }

    /// If both elements share an edge, it is returned.
    pub fn get_related_edge(&self, other: &Element) -> Option<Edge> {
        let mut points: Vec<Point> = Vec::with_capacity(2);

        for coord in &self.coordinates {
            for ocoord in &other.coordinates {
                if coord == ocoord {
                    points.push(*coord);
                }
            }
        }

        if points.len() == 2 {
            Some((points[0], points[1]))
        } else {
            None
        }
    }

    fn get_radius(&self, pt: Point) -> f64 {
        pt.distance_to(&self.coordinates[0])
    }

    pub fn in_circle(&self, p: Point) -> bool {
        let center = self.get_center();
        let ds = center.distance_to(&p);
        ds <= self.get_radius(center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_triangles_work() {
        let p0 = Point { x: 0_f64, y: 0_f64 };
        let p1 = Point {
            x: 10_f64,
            y: 0_f64,
        };
        let p2 = Point {
            x: 0_f64,
            y: 10_f64,
        };

        let e = Element {
            coordinates: vec![p0, p1, p2],
            num_coordinates: 3,
            // the following values are possibly bogus
            neighbors: vec![],
            obtuse_angle: None,
        };

        assert!(e.is_bad() == false);
    }

    #[test]
    fn bad_triangles_work() {
        let p0 = Point { x: 0_f64, y: 0_f64 };
        let p1 = Point {
            x: 10_f64,
            y: 0_f64,
        };
        let p2 = Point {
            x: 0_f64,
            y: 10000_f64,
        };

        let e = Element {
            coordinates: vec![p0, p1, p2],
            num_coordinates: 3,
            // the following values are possibly bogus
            neighbors: vec![],
            obtuse_angle: None,
        };

        assert!(e.is_bad() == true);
    }
}
