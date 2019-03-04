#[derive(Debug)]
/// A point in the 3D maze
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

/// The central maze data structure
pub struct Maze {
    /// The Grid we are working on
    pub grid: Grid,
    /// If any obstacles have been provided, those are stored here
    pub obstacles: Option<Vec<Point>>,
    /// Paths already mapped into the grid
    pub paths: Vec<Path>,
    /// Paths that have not been mapped yet
    pub unmapped_paths: Vec<(Point, Point)>
}

/// A single path in the maze.
pub struct Path {
    /// Starting point
    start: Point,
    /// Ending point
    end: Point,
    /// All points to be visitad between start and end
    path: Vec<Point>
}

/// The grid the maze is set up in. Contains for every field the information about its state.
pub type Grid = Vec<Vec<Vec<Field>>>;

#[derive(Debug, Clone)]
/// A single field. Can be either free or used or it may be a wall.
pub enum Field {
    Free,
    Used,
    Wall,
}

/// Initializes a new, empty grid, optionally with pre-defined walls which can be provided using the `walls` parameter.
///
/// The width defines the domain of the x-axis, the depth the domain of the y axis and depth the z-axis.
pub fn initialize_grid(width: usize, height: usize, depth: usize, walls: Option<&Vec<Point>>) -> Grid {
    let mut grid = vec![vec![vec![Field::Free; depth]; height]; width];

    // place walls if any
    if let Some(wall_vec) = walls {
        for wall in wall_vec {
            grid[wall.x][wall.y][wall.z] = Field::Wall;
        }
    }

    grid
}
