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
