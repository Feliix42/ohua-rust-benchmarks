use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// A point in the 3D maze
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.x, self.y, self.z)
    }
}
