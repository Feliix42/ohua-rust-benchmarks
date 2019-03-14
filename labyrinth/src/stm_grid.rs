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
    let mut grid = vec![vec![Vec::with_capacity(depth); height]; width];

    // must initialize this way because of the `clone` semantics of `vec!`
    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                grid[x][y][z] = TVar::new(Field::Free);
            }
        }
    }

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

/// Updates the grid with mapped paths.
pub fn update_grid(
    mut maze: &Grid,
    mut paths: Vec<Path>,
    transaction: &mut Transaction,
) -> StmResult<(Vec<Path>, Vec<(Point, Point)>)> {
    // TODO: should this return a list of nodes that have been overwritten by previously mapped nodes from _this current_ run? (i.e., should we remap locally?)
    let mut mapped = Vec::new();
    let mut not_mapped = Vec::new();

    for path in paths.drain(..) {
        if path_is_available(&maze.grid, &path) {
            for pt in &path.path {
                grid[pt.x][pt.y][pt.z].write(transaction, Field::Used)?;
            }
            mapped.push(path);
        } else {
            not_mapped.push((path.start, path.end));
        }
    }

    Ok((mapped, not_mapped))
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
