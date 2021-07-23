use std::fmt;


// dummy data structures for now
#[derive(Clone, Debug)]
pub struct Maze {}
impl Maze {
    pub fn init(salt: i32) -> Self {
        unimplemented!()
    }

    pub fn update(&mut self, path: Vec<Point>) -> Option<(Point, Point)> {
        unimplemented!()
    }

    pub fn is_valid(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
//#[derive(Clone)]
//pub struct Point {}

pub fn find_path(m: Maze, pair: (Point, Point)) -> Vec<Point> {
    unimplemented!()
}

pub fn get_unmapped(
    results: Vec<Option<(Point, Point)>>,
    its_left: u32,
) -> (Vec<(Point, Point)>, bool, u32) {
    unimplemented!()
}

pub fn filter_mapped(results: Vec<Option<(Point, Point)>>) -> Vec<(Point, Point)> {
    unimplemented!()
}

pub fn calculate_done(results: Vec<(Point, Point)>, its_left: u32) -> bool {
    unimplemented!()
}

pub fn decrement(u: u32) -> u32 {
    unimplemented!()
}

pub fn fill1(m: Maze, p: Vec<(Point, Point)>, ma: u32) -> Maze {
    unimplemented!()
}
