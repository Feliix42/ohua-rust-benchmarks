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
) -> StmResult<Field> {
    Ok(grid[pt.x][pt.y][pt.z].read(transaction)?)
}

/// Updates the grid with mapped paths.
pub fn update_grid(
    grid: &Grid,
    path: &Path,
    transaction: &mut Transaction,
) -> StmResult<()> {
    for pt in &path.path {
        grid[pt.x][pt.y][pt.z].write(transaction, Field::Used)?;
    }

    Ok(())
}
