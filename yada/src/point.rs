//! In the original codebase, this was `coordinate.c`

use decorum::R64;
use num_traits::float::FloatConst;
use num_traits::real::Real;
use std::cmp::{Eq, Ord};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: R64,
    pub y: R64,
}

impl Point {
    pub fn distance_to(&self, other: &Self) -> R64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;

        let d = (delta_x * delta_x) + (delta_y * delta_y);
        R64::sqrt(d)
    }

    /// Angle formed by b, self, c
    pub fn angle(&self, b: &Self, c: &Self) -> R64 {
        let delta_b = *b - *self;
        let delta_c = *c - *self;

        let numerator = delta_b * delta_c;

        let distance_b = self.distance_to(b);
        let distance_c = self.distance_to(c);
        let denominator = distance_b * distance_c;

        let cosine = numerator / denominator;
        let radian = R64::acos(cosine);

        R64::from_inner(180_f64) * radian / R64::PI() //decorum::ConstrainedFloat::PI
    }

    /// Angle formed by b, self, c
    pub fn angle_is_obtuse(&self, b: &Self, c: &Self) -> bool {
        let vb = *b - *self;
        let vc = *c - *self;

        (vb * vc) < 0f64
    }

    // Seen in Tuple.h
    pub fn angle_check() {
        unimplemented!()
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// The Scalar Product
impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// The Scalar Product
impl Mul<R64> for Point {
    type Output = Self;

    fn mul(self, rhs: R64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// The Dot Product
impl Mul for Point {
    type Output = R64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

/*
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This seems like absolute gore and I agree, but we're only feeding the mesh with ints and
        // using this function on those numbers so it's ok.
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering_works() {
        let p0 = Point {
            x: R64::from_inner(0_f64),
            y: R64::from_inner(0_f64),
        };
        let p1 = Point {
            x: R64::from_inner(1_f64),
            y: R64::from_inner(0_f64),
        };
        let p2 = Point {
            x: R64::from_inner(1_f64),
            y: R64::from_inner(1_f64),
        };
        let p3 = Point {
            x: R64::from_inner(2_f64),
            y: R64::from_inner(2_f64),
        };
        let p4 = Point {
            x: R64::from_inner(1_f64),
            y: R64::from_inner(2_f64),
        };

        assert!(p1 > p0);
        assert!(p2 > p1);
        assert!(p3 > p4);
    }

    #[test]
    fn angle_computation_works() {
        let a = Point {
            x: R64::from_inner(0_f64),
            y: R64::from_inner(0_f64),
        };
        let b = Point {
            x: R64::from_inner(0_f64),
            y: R64::from_inner(1_f64),
        };
        let c = Point {
            x: R64::from_inner(1_f64),
            y: R64::from_inner(0_f64),
        };

        // don't assert for = 0 because of FPU imprecision
        dbg!(a.angle(&b, &c));
        assert!(a.angle(&b, &c) - 90.0_f64 < 1e-6);
        println!("{}", b.angle(&c, &a));
        assert!(b.angle(&c, &a) - 45.0_f64 < 1e-6);
        assert!(c.angle(&a, &b) - 45.0_f64 < 1e-6);
    }

    #[test]
    fn obtuse_angles_work() {
        let a = Point {
            x: R64::from_inner(0_f64),
            y: R64::from_inner(0_f64),
        };
        let b = Point {
            x: R64::from_inner(0_f64),
            y: R64::from_inner(1_f64),
        };
        let c = Point {
            x: R64::from_inner(1_f64),
            y: R64::from_inner(-1_f64),
        };
        let d = Point {
            x: R64::from_inner(1_f64),
            y: R64::from_inner(0_f64),
        };

        assert!(a.angle_is_obtuse(&c, &b));
        assert!(!a.angle_is_obtuse(&d, &b));
    }
}
