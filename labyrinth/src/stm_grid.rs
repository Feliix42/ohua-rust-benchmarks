use crate::types::*;
//use stm::{StmError, StmResult, TVar, Transaction};
use std::ops::Deref;
use swym::tcell::TCell;
use swym::tx::{Ordering, Status};
use swym::RwTx;

/// The grid the maze is set up in. Contains for every field the information about its state.
pub type StmGrid = Vec<Vec<Vec<TCell<Field>>>>;
pub type Grid = Vec<Vec<Vec<Field>>>;

/// Initializes a new, empty grid, optionally with pre-defined walls which can be provided using the `walls` parameter.
///
/// The width defines the domain of the x-axis, the depth the domain of the y axis and depth the z-axis.
pub fn initialize_grid(
    width: usize,
    height: usize,
    depth: usize,
    _walls: &Option<Vec<Point>>,
) -> StmGrid {
    (0..width)
        .map(|_| {
            (0..height)
                .map(|_| (0..depth).map(|_| TCell::new(Field::Free)).collect())
                .collect()
        })
        .collect()
    //let mut grid = vec![vec![vec![TCell::new(Field::Free); depth]; height]; width];

    //// must initialize this way because of the `clone` semantics of `vec!`
    //for x in 0..width {
    //for y in 0..height {
    //for z in 0..depth {
    //grid[x][y][z] = TCell::new(Field::Free);
    //}
    //}
    //}

    //// place walls if any
    //if let Some(wall_vec) = walls {
    //for wall in wall_vec {
    //grid[wall.x][wall.y][wall.z] = TCell::new(Field::Wall);
    //}
    //}

    //grid
}

//pub fn at_stm_grid_coordinates<'a>(
//grid: &'a StmGrid,
//pt: &Point,
//transaction: &mut Transaction,
//) -> StmResult<Field> {
//Ok(grid[pt.x][pt.y][pt.z].read(transaction)?)
//}

pub fn at_grid_coordinates<'a>(grid: &'a Grid, pt: &Point) -> &'a Field {
    &grid[pt.x][pt.y][pt.z]
}

/// Updates the grid with mapped paths.
pub fn update_grid<'tcell>(
    grid: &'tcell StmGrid,
    path: &Path,
    transaction: &mut RwTx<'tcell>,
) -> Result<(), Status> {
    for pt in &path.path {
        // here we want a serializability of read and write accesses
        if grid[pt.x][pt.y][pt.z]
            .borrow(transaction, Ordering::ReadWrite)?
            .deref()
            == &Field::Free
        {
            grid[pt.x][pt.y][pt.z].set(transaction, Field::Used)?;
        } else {
            // NOTE(felix42): This may get stuck on collision, beware!
            return Err(Status::AWAIT_RETRY);
        }
    }

    Ok(())
}

//pub fn create_working_copy(grid: &StmGrid) -> Grid {
    //grid.iter()
        //.map(|y_grid| {
            //y_grid
                //.iter()
                //.map(|z_grid| z_grid.iter().map(|pt| pt.clone().into_inner()).collect())
                //.collect()
        //})
        //.collect()
//}
