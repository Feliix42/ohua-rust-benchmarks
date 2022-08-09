use crate::grid::*;
use std::collections::LinkedList;
use std::fmt;
use std::sync::Arc;

/// This data structure contains information on whether a point has been visited before and
/// information on how to get back from the end point to the start.
/// Each point is assigned the previous point in the path to allow easy backtracking.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum PointStatus {
    #[default]
    Unvisited,
    BacktrackInfo(Option<Point>)
}

impl PointStatus {
    #[inline]
    pub fn unvisited(&self) -> bool {
        *self == Self::Unvisited
    }
}


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

    /// Updates the labyrinth structure, returns the start and end point if the update was
    /// unsuccessful
    pub fn update(&mut self, path: Option<Path>, retry_sender: &std::sync::mpsc::Sender<usize>) -> Option<(Point, Point)> {
        let path = path?;

        if path_available(&self.grid, &path) {
            for pt in &path.path {
                self.grid[pt.x][pt.y][pt.z] = Field::Used;
            }
            self.paths.push(path);
            None
        } else {
            retry_sender.send(1).unwrap();
            Some((path.start, path.end))
        }
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

pub fn path_available(grid: &Grid, path: &Path) -> bool {
    for pt in &path.path {
        if at_grid_coordinates(grid, pt) != &Field::Free {
            return false;
        }
    }

    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Field {
    Free,
    Used,
    Wall,
}

pub fn find_path(maze: Arc<Maze>, pair: Option<(Point, Point)>) -> Option<Path> {
    // TODO: Add costs?
    let (start, end) = pair.unwrap();

    // check if the route is still available
    if at_grid_coordinates(&maze.grid, &start) != &Field::Free {
        return None;
    }

    let mut point_status = vec![vec![vec![PointStatus::Unvisited; maze.grid[0][0].len()]; maze.grid[0].len()]; maze.grid.len()];
    let mut unseen_points = LinkedList::new();

    // set the start point
    point_status[start.x][start.y][start.z] = PointStatus::BacktrackInfo(None);
    unseen_points.push_back(start);

    while !unseen_points.is_empty() {
        let current = unseen_points.pop_front().unwrap();

        // stop when reacing the end node
        if current == end {
            return Some(generate_path(current, &point_status));
        }

        // get a list of all possible successors
        for child in get_successors(&current, &maze.grid) {
            // sort out anything that has been seen or is blocked
            match at_grid_coordinates(&maze.grid, &child) {
                &Field::Used => continue,
                &Field::Wall => continue,
                &Field::Free => (),
            }

            if point_status[child.x][child.y][child.z].unvisited() {
                point_status[child.x][child.y][child.z] = PointStatus::BacktrackInfo(Some(current));
                unseen_points.push_back(child);
            }
        }
    }

    // All points have been processed and no path was found
    None
}

fn get_successors(cur: &Point, grid: &Grid) -> Vec<Point> {
    let mut res = Vec::with_capacity(6);

    if cur.x > 0 {
        res.push(Point {
            x: cur.x - 1,
            y: cur.y,
            z: cur.z,
        });
    }
    if cur.x < grid.len() - 1 {
        res.push(Point {
            x: cur.x + 1,
            y: cur.y,
            z: cur.z,
        });
    }
    if cur.y > 0 {
        res.push(Point {
            x: cur.x,
            y: cur.y - 1,
            z: cur.z,
        });
    }
    if cur.y < grid[0].len() - 1 {
        res.push(Point {
            x: cur.x,
            y: cur.y + 1,
            z: cur.z,
        });
    }
    if cur.z > 0 {
        res.push(Point {
            x: cur.x,
            y: cur.y,
            z: cur.z - 1,
        });
    }
    if cur.z < grid[0][0].len() - 1 {
        res.push(Point {
            x: cur.x,
            y: cur.y,
            z: cur.z + 1,
        });
    }

    res
}

fn generate_path(end_node: Point, meta_info: &Vec<Vec<Vec<PointStatus>>>) -> Path {
    let mut path = vec![end_node];
    let mut current = end_node;

    while let PointStatus::BacktrackInfo(Some(next)) = meta_info[current.x][current.y][current.z] {
        path.push(next);
        current = next;
    }

    // important, we built the vec from end -> start
    path.reverse();

    Path {
        start: current,
        end: end_node,
        path,
    }
}

//pub fn get_unmapped(
//results: Vec<Option<(Point, Point)>>,
//iterations_finished: u32,
//) -> (Vec<(Point, Point)>, bool, u32) {
//unimplemented!()
//}

pub fn filter_mapped(results: Vec<Option<(Point, Point)>>) -> Vec<Option<(Point, Point)>> {
    results.into_iter().filter(Option::is_some).collect()
}

pub fn calculate_done(results: Vec<Option<(Point, Point)>>, iterations_finished: u32) -> (u32, bool) {
    let should_cont = results.iter().any(Option::is_some);
    (iterations_finished + 1, should_cont)
}

//pub fn decrement(u: u32) -> u32 {
//unimplemented!()
//}

//pub fn fill1(m: Maze, p: Vec<(Point, Point)>, ma: u32) -> Maze {
//unimplemented!()
//}
