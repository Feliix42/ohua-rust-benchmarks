use crate::types::*;
use stm::{StmResult, TVar, Transaction};

/// The grid the maze is set up in. Contains for every field the information about its state.
pub type Grid = Vec<Vec<Vec<TVar<Field>>>>;

/// Initializes a new, empty grid, optionally with pre-defined walls which can be provided using the `walls` parameter.
///
/// The width defines the domain of the x-axis, the depth the domain of the y axis and depth the z-axis.
pub fn initialize_grid(
    width: usize,
    height: usize,
    depth: usize,
    walls: &Option<Vec<Point>>,
) -> Grid {
    let mut grid = vec![vec![vec![TVar::new(Field::Free); depth]; height]; width];

    // place walls if any
    if let Some(wall_vec) = walls {
        for wall in wall_vec {
            grid[wall.x][wall.y][wall.z] = TVar::new(Field::Wall);
        }
    }

    grid
}

pub fn at_grid_coordinates<'a>(
    grid: &'a Grid,
    pt: &Point,
    transaction: &mut Transaction,
) -> StmResult<&'a Field> {
    &grid[pt.x][pt.y][pt.z].read(transaction)
}

/// Updates the maze with mapped paths by updating the underlying grid and the management data
/// structures in the `Maze` struct.
///
/// Returns the updated struct and the paths that require remapping (i.e., due to overlapping paths).
pub fn update_maze(
    mut maze: Maze,
    mut paths: Vec<Path>,
    transaction: &mut Transaction,
) -> StmResult<(Maze, Vec<(Point, Point)>)> {
    let mut non_matching = Vec::new();

    for path in paths.drain(..) {
        if path_is_available(&maze.grid, &path) {
            for pt in &path.path {
                maze.grid[pt.x][pt.y][pt.z].write(transaction, Field::Used)?;
            }
            maze.paths.push(path);
        } else {
            non_matching.push((path.start, path.end));
        }
    }

    (maze, non_matching)
}

/// Checks whether a path is still available (i.e., free) on the grid
fn path_is_available(grid: &Grid, path: &Path, transaction: &mut Transaction) -> StmResult<bool> {
    for point in &path.path {
        match at_grid_coordinates(grid, point)? {
            &Field::Free => (),
            &Field::Used => return false,
            &Field::Wall => panic!("Routed a path through a wall"),
        }
    }

    true
}
