use std::fmt;
use std::sync::Arc;
use crate::grid::*;


// dummy data structures for now
#[derive(Clone, Debug)]
pub struct Maze {
    /// The Grid we are working on
    pub grid: Grid,
    /// If any obstacles have been provided, those are stored here
    pub obstacles: Option<Vec<Point>>,
    /// Paths already mapped into the grid
    pub paths: Vec<Path>,
    /// Paths that could not be mapped
    pub unmappable_paths: Vec<(Point, Point)>,
}

impl Maze {
    /// Initialize the maze, for now w/o obstacles
    pub fn init(dimensions: Point) -> Self {
        Maze {
            grid: initialize_grid(dimensions.x, dimensions.y, dimensions.z, &None),
            obstacles: None,
            paths: Vec::new(),
            unmappable_paths: Vec::new(),
        }
    }

    pub fn update(&mut self, path: Option<Path>) -> Option<(Point, Point)> {
        unimplemented!()
    }

    pub fn is_valid(&self) -> bool {
        let mut ctrl_grid = self.grid.clone();

        for path in &self.paths {
            for pt in &path.path {
                if at_grid_coordinates(&ctrl_grid, &pt) == &Field::Used {
                    ctrl_grid[pt.x][pt.y][pt.z] = Field::Free;
                } else {
                    return false;
                }
            }
        }

        true
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

/// A single path in the maze.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path {
    /// Starting point
    pub start: Point,
    /// Ending point
    pub end: Point,
    /// All points to be visited from start to end
    pub path: Vec<Point>,
}

/// A single field. Can be either free or used or it may be a wall.
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Free,
    Used,
    Wall,
}

pub fn find_path(m: Arc<Maze>, pair: Option<(Point, Point)>) -> Option<Path> {
    unimplemented!()
}

//pub fn get_unmapped(
    //results: Vec<Option<(Point, Point)>>,
    //its_left: u32,
//) -> (Vec<(Point, Point)>, bool, u32) {
    //unimplemented!()
//}

pub fn filter_mapped(results: Vec<Option<(Point, Point)>>) -> Vec<Option<(Point, Point)>> {
    results.into_iter().filter(Option::is_some).collect()
}

pub fn calculate_done(results: Vec<Option<(Point, Point)>>, its_left: u32) -> (u32, bool) {
    let done = results.iter().all(Option::is_none);
    (its_left-1, done)
}

//pub fn decrement(u: u32) -> u32 {
    //unimplemented!()
//}

//pub fn fill1(m: Maze, p: Vec<(Point, Point)>, ma: u32) -> Maze {
    //unimplemented!()
//}
