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
    /// Paths that could not be mapped
    pub unmappable_paths: Vec<(Point, Point)>,
}

impl Maze {
    pub fn new(dimensions: Point, obstacles: Option<Vec<Point>>) -> Self {
        Maze {
            grid: initialize_grid(dimensions.x, dimensions.y, dimensions.z, &obstacles),
            obstacles: obstacles,
            paths: Vec::new(),
            unmappable_paths: Vec::new(),
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
#[derive(Debug, Clone, PartialEq)]
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

/// Updates the maze with mapped paths by updating the underlying grid and the management data
/// structures in the `Maze` struct.
///
/// Returns the updated struct and the paths that require remapping (i.e., due to overlapping paths).
pub fn update_maze(mut maze: Maze, mut paths: Vec<Path>) -> (Maze, Vec<(Point, Point)>) {
    let mut non_matching = Vec::new();

    for path in paths.drain(..) {
        if path_is_available(&maze.grid, &path) {
            for pt in &path.path {
                maze.grid[pt.x][pt.y][pt.z] = Field::Used;
            }
            maze.paths.push(path);
        } else {
            non_matching.push((path.start, path.end));
        }
    }

    (maze, non_matching)
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
