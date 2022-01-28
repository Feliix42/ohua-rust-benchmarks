//! In the original codebase, this was `element.c`

use crate::point::Point;

const MIN_ANGLE: f64 = 30.0;

// pub enum Element {
//     T(Triangle),
//     E(Edge),
// }

// impl Element {
//     pub fn is_bad(&self) -> bool {
//         match self {
//             Self::T(t) => t.is_bad(),
//             Self::E(e) => e.is_bad(),
//         }
//     }

//     pub fn get_center(&self) -> Point {
//         match self {
//             Self::T(t) => t.get_center(),
//             Self::E(e) => e.get_center(),
//         }
//     }

//     pub fn get_edge(&self, i: usize) -> Edge {
//         match self {
//             Self::T(t) => t.get_edge(i),
//             Self::E(e) => e.get_edge(i),
//         }
//     }

//     pub fn get_obtuse(&self) -> Point {
//         match self {
//             Self::T(t) => t.get_obtuse(),
//             Self::E(_) => panic!("A line has no obtuse angles."),
//         }
//     }
// }

// impl From<Triangle> for Element {
//     fn from(t: Triangle) -> Self {
//         Self::T(t)
//     }
// }

// impl From<Edge> for Element {
//     fn from(e: Edge) -> Self {
//         Self::E(e)
//     }
// }

pub trait Element {
    /// Determines whether an element needs to be processed.
    fn is_bad(&self) -> bool;
    /// Get a list of (references to) all points of the element.
    fn get_points<'a>(&'a self) -> Vec<&'a Point>;
    /// Get the center point of the element.
    fn get_center(&self) -> Point;
    /// Get the edge with the assigned number.
    fn get_edge(&self, i: usize) -> Edge;
    /// Get the point with the obtuse angle.
    fn get_obtuse(&self) -> Point;
    /// Returns `true` if both elements share an edge.
    fn is_related_to(&self, other: &dyn Element) -> bool {
        let this_pt = self.get_points();
        let other_pt = other.get_points();

        let mut num_matching_points = 0;

        for pt in this_pt {
            for pt2 in &other_pt {
                if &pt == pt2 {
                    num_matching_points += 1;
                }
            }
        }

        num_matching_points == 2
    }

    /// If both elements share an edge, it is returned.
    fn get_related_edge(&self, other: &dyn Element) -> Option<Edge> {
        let this_pt = self.get_points();
        let other_pt = other.get_points();
        let mut points: Vec<Point> = Vec::with_capacity(2);

        for coord in this_pt {
            for ocoord in &other_pt {
                if &coord == ocoord {
                    points.push(*coord);
                }
            }
        }

        if points.len() == 2 {
            Some(Edge::new(points[0], points[1]))
        } else {
            panic!("Found no related edge, got: {:?}", points);
            //None
        }
    }

    fn get_radius(&self, pt: Point) -> f64;

    fn in_circle(&self, p: Point) -> bool {
        let center = self.get_center();
        let ds = center.distance_to(&p);
        ds <= self.get_radius(center)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Triangle {
    /// The coordinates of the triangle, sorted by size.
    coordinates: [Point; 3],

    /// The index of the point that houses the obtuse angle.
    obtuse_angle: Option<usize>,
}

impl Triangle {
    /// Create a new triangle
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
        assert_ne!(p3, p1);

        let mut coordinates: [Point; 3] = [p1, p2, p3];
        // coordinates.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        coordinates.sort_unstable();

        // coordinates.sort_unstable();
        let mut obtuse = None;

        for i in 0..3 {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            if coordinates[j].angle_is_obtuse(&coordinates[i], &coordinates[k]) {
                obtuse = Some(i);
            }
        }

        Triangle {
            coordinates,
            obtuse_angle: obtuse,
        }
    }
}

impl Element for Triangle {
    fn is_bad(&self) -> bool {
        for i in 0..3 {
            let ang = self.coordinates[i].angle(
                &self.coordinates[(i + 1) % 3],
                &self.coordinates[(i + 2) % 3],
            );

            if ang < MIN_ANGLE {
                return true;
            }
        }

        false
    }

    fn get_center(&self) -> Point {
        let a = self.coordinates[0];
        let b = self.coordinates[1];
        let c = self.coordinates[2];

        let x = b - a;
        let y = c - a;
        let x_len = a.distance_to(&b);
        let y_len = a.distance_to(&c);
        let cosine = (x * y) / (x_len * y_len);
        let sine_sq = 1.0 - cosine * cosine;
        let p_len = y_len / x_len;

        let s = p_len * cosine;
        let t = p_len * sine_sq;

        let wp = (p_len - cosine) / (2f64 * t);
        let wb = 0.5 - (wp * s);

        let mut tmp_val = a * (1f64 - wb - wp);
        tmp_val = tmp_val + (b * wb);
        tmp_val + (c * wp)
    }

    fn get_points<'a>(&'a self) -> Vec<&'a Point> {
        vec![
            &self.coordinates[0],
            &self.coordinates[1],
            &self.coordinates[2],
        ]
    }

    fn get_edge(&self, i: usize) -> Edge {
        match i {
            0 => Edge::new(self.coordinates[0], self.coordinates[1]),
            1 => Edge::new(self.coordinates[1], self.coordinates[2]),
            2 => Edge::new(self.coordinates[2], self.coordinates[0]),
            // error case
            _ => Edge::new(self.coordinates[0], self.coordinates[0]),
        }
    }

    fn get_obtuse(&self) -> Point {
        if let Some(i) = self.obtuse_angle {
            self.coordinates[i]
        } else {
            panic!("No obtuse angle exists for this triangle!");
        }
    }

    fn get_radius(&self, pt: Point) -> f64 {
        pt.distance_to(&self.coordinates[0])
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Edge(Point, Point);

impl Edge {
    pub fn new(p1: Point, p2: Point) -> Self {
        if p1 < p2 {
            Edge(p1, p2)
        } else {
            Edge(p2, p1)
        }
    }
}

impl Element for Edge {
    fn is_bad(&self) -> bool {
        // this can't actually ever be true
        false
    }

    fn get_center(&self) -> Point {
        (self.0 + self.1) * 0.5
    }

    fn get_points<'a>(&'a self) -> Vec<&'a Point> {
        vec![&self.0, &self.1]
    }
    fn get_edge(&self, i: usize) -> Edge {
        match i {
            1 => Self(self.1, self.0),
            _ => *self,
        }
    }

    fn get_obtuse(&self) -> Point {
        panic!("A line has no obtuse angles.")
    }

    fn get_radius(&self, pt: Point) -> f64 {
        pt.distance_to(&self.0)
    }
}
/*
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        // everything is derived from the coordinates
        self.coordinates == other.coordinates
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.coordinates.cmp(&other.coordinates)
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

*/
