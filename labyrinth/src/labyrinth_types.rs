#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A point in the 3D maze
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

/// The central maze data structure
#[derive(Debug)]
pub struct Maze {
    /// The Grid we are working on
    pub grid: Grid,
    /// If any obstacles have been provided, those are stored here
    pub obstacles: Option<Vec<Point>>,
    /// Paths already mapped into the grid
    pub paths: Vec<Path>,
    /// Paths that have not been mapped yet
    pub unmapped_paths: Vec<(Point, Point)>,
}

impl Maze {
    pub fn new(
        dimensions: Point,
        paths: Vec<(Point, Point)>,
        obstacles: Option<Vec<Point>>,
    ) -> Self {
        Maze {
            grid: initialize_grid(dimensions.x, dimensions.y, dimensions.z, &obstacles),
            obstacles: obstacles,
            paths: Vec::with_capacity(paths.len()),
            unmapped_paths: paths,
        }
    }
}

/// A single path in the maze.
#[derive(Debug, Eq, PartialEq)]
pub struct Path {
    /// Starting point
    pub start: Point,
    /// Ending point
    pub end: Point,
    /// All points to be visited from start to end
    pub path: Vec<Point>,
}

/// The grid the maze is set up in. Contains for every field the information about its state.
pub type Grid = Vec<Vec<Vec<Field>>>;

/// A single field. Can be either free or used or it may be a wall.
#[derive(Debug, Clone)]
pub enum Field {
    Free,
    Used,
    Wall,
}

/// Initializes a new, empty grid, optionally with pre-defined walls which can be provided using the `walls` parameter.
///
/// The width defines the domain of the x-axis, the depth the domain of the y axis and depth the z-axis.
pub fn initialize_grid(
    width: usize,
    height: usize,
    depth: usize,
    walls: &Option<Vec<Point>>,
) -> Grid {
    let mut grid = vec![vec![vec![Field::Free; depth]; height]; width];

    // place walls if any
    if let Some(wall_vec) = walls {
        for wall in wall_vec {
            grid[wall.x][wall.y][wall.z] = Field::Wall;
        }
    }

    grid
}

pub fn at_grid_coordinates<'a>(grid: &'a Grid, pt: &Point) -> &'a Field {
    &grid[pt.x][pt.y][pt.z]
}

/// Updates the grid with any paths that have been found. Overlapping or not matching paths are
/// not implemented but returned instead.
pub fn update_grid(mut grid: Grid, mut paths: Vec<Path>) -> (Grid, Vec<(Point, Point)>) {
    let mut non_matching = Vec::new();

    for path in paths.drain(..) {
        if path_is_available(&grid, &path) {
            for pt in path.path {
                grid[pt.x][pt.y][pt.z] = Field::Used;
            }
        } else {
            non_matching.push((path.start, path.end));
        }
    }

    (grid, non_matching)
}

/// Checks whether a path is still available (i.e., free) on the grid
fn path_is_available(grid: &Grid, path: &Path) -> bool {
    for point in &path.path {
        match at_grid_coordinates(grid, point) {
            &Field::Free => (),
            &Field::Used => return false,
            &Field::Wall => panic!("Routed a path through a wall"),
        }
    }

    true
}
