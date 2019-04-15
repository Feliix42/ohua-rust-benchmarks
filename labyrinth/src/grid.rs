use crate::types::*;

/// The grid the maze is set up in. Contains for every field the information about its state.
pub type Grid = Vec<Vec<Vec<Field>>>;

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

/// Updates the maze with a mapped path by updating the underlying grid and the management data
/// structures in the `Maze` struct.
#[cfg(not(feature = "ohua"))]
pub fn update_maze(maze: &mut Maze, path: Path) {
    for pt in &path.path {
        maze.grid[pt.x][pt.y][pt.z] = Field::Used;
    }
    maze.paths.push(path);
}

/// Updates the grid with mapped paths.
#[cfg(feature = "ohua")]
pub fn update_maze(mut maze: Maze, paths: Vec<Option<Path>>) -> (Vec<(Point, Point)>, Maze) {
    let mut remap = Vec::new();

    for p in paths {
        // skip empty paths
        let path = match p {
            Some(pa) => pa,
            None => continue,
        };

        if path_available(&maze.grid, &path) {
            for pt in &path.path {
                maze.grid[pt.x][pt.y][pt.z] = Field::Used;
            }
        } else {
            remap.push((path.start, path.end));
        }
    }

    (remap, maze)
}

#[cfg(feature = "ohua")]
fn path_available(grid: &Grid, path: &Path) -> bool {
    for pt in &path.path {
        if at_grid_coordinates(grid, pt) != &Field::Free {
            return false;
        }
    }

    true
}
