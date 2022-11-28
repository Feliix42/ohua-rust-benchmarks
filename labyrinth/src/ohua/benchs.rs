use crate::ohua::grid::*;
use crate::types::*;
use std::collections::LinkedList;
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
    pub fn update(&mut self, path: Option<Path>
    //              , retry_sender: &std::sync::mpsc::Sender<usize>
    ) -> Option<(Point, Point)> {
        let path = path?;

        if path_available(&self.grid, &path) {
            for pt in &path.path {
                self.grid[pt.x][pt.y][pt.z] = Field::Used;
            }
            self.paths.push(path);
            None
        } else {
//            retry_sender.send(1).unwrap();
            Some((path.start, path.end))
        }
    }

    pub fn update_paths(&mut self, paths: Vec<Option<Path>>) -> Vec<OPoint> {
        paths.into_iter().map(|p| self.update(p)).collect()
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

pub type OPoint = Option<(Point, Point)>;

#[derive(Default)]
pub struct Unmapped {
    rs: Vec<OPoint>
}

impl Unmapped {

    pub fn filter_mapped(&mut self) {
        self.rs.retain(Option::is_some);
    }

    pub fn calculate_done(mut self) -> Vec<OPoint> {
        self.filter_mapped();
        //let should_cont = !self.rs.is_empty();
        self.rs
    }

    pub fn calculate_done_with_cont(mut self, iterations_finished: u32) -> (u32, bool, Vec<OPoint>) {
        self.filter_mapped();
        let should_cont = !self.rs.is_empty();
        (iterations_finished + 1, should_cont, self.rs)
    }

    pub fn push(&mut self, n:OPoint) {
        self.rs.push(n)
    }
}

pub trait UnmappedPaths {
    fn calculate_done(&mut self) -> bool;
}

impl UnmappedPaths for Vec<OPoint> {
    fn calculate_done(&mut self) -> bool {
        // filter old elements
        self.retain(Option::is_some);

        !self.is_empty()
    }
}

pub trait Unarc {
    fn unarc(&self, item: Arc<Maze>) -> Maze {
        match Arc::try_unwrap(item) {
            Ok(ap) => ap,
            _ => panic!("Failed to unwrap the Arc. Please make sure that the construction of `x` has destructed all previous Arcs.")
        }
    }
}

impl Unarc for Vec<Option<Path>> {}

pub fn filter_paths(mut unm: Unmapped) -> Unmapped {
    unm.filter_mapped();
    unm
}

pub fn seq_arc_unwrap<S, T>(a: Arc<S>, x: T) -> (S, T) {
    match Arc::<S>::try_unwrap(a) {
        Ok(ap) => (ap,x),
        _ => panic!("Failed to unwrap the Arc. Please make sure that the construction of `x` has destructed all previous Arcs.")
    }
}

pub fn not_done(lst: Vec<OPoint>, iteration_count: u32) -> (u32, bool, Vec<OPoint>) {
    (iteration_count + 1, !lst.is_empty(), lst)
}

pub fn inc(num: u32) -> u32 {
    num + 1
}
