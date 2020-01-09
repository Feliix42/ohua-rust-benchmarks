//! In the original codebase, this was `coordinate.c`

use std::ops::Sub;

#[derive(PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;

        f64::sqrt((delta_x * delta_x) + (delta_y * delta_y))
    }

    pub fn angle(a: &Self, b: &Self, c: &Self) -> f64 {
        let delta_b = b - a;
        let delta_c = c - a;

        let numerator = (delta_b.x * delta_c.x) + (delta_b.y * delta_c.y);

        let distance_b = a.distance_to(b);
        let distance_c = a.distance_to(c);
        let denominator = distance_b * distance_c;

        let cosine = numerator / denominator;
        let radian = f64::acos(cosine);

        180_f64 * radian / std::f64::consts::PI
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering_works() {
        let p0 = Point { x: 0_f64, y: 0_f64 };
        let p1 = Point { x: 1_f64, y: 0_f64 };
        let p2 = Point { x: 1_f64, y: 1_f64 };
        let p3 = Point { x: 2_f64, y: 2_f64 };
        let p4 = Point { x: 1_f64, y: 2_f64 };
        
        assert!(p1 > p0);
        assert!(p2 > p1);
        assert!(p3 > p4);
    }

    #[test]
    fn angle_computation_works() {
        let a = Point { x: 0_f64, y: 0_f64 };
        let b = Point { x: 0_f64, y: 1_f64 };
        let c = Point { x: 1_f64, y: 0_f64 };

        // don't assert for = 0 because of FPU imprecision
        assert!(Point::angle(&a, &b, &c) - 90.0_f64 < 1e-6);
        assert!(Point::angle(&b, &c, &a) - 45.0_f64 < 1e-6);
        assert!(Point::angle(&c, &a, &b) - 45.0_f64 < 1e-6);

    }
}
