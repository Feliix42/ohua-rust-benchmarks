pub mod parser;
pub mod types;
pub mod stmseq;
pub mod ohua;

pub use stmseq::*;

#[cfg(kani)]
mod verification {
    // TODO: allow any status of the labyrinth (filled, ...)

    #[kani::proof]
    pub fn compare_seq_ohua() {
        let dim_x: usize = kani::any();
        let dim_y: usize = kani::any();
        let dim_z: usize = kani::any();

        let dimensions = Point { x: dim_x, y: dim_y, z: dim_z };

        let mut paths = Vec::new();
        for _ in 0..2 {
            let x = kani::any();
            kani::assume(x < dim_x);
            let y = kani::any();
            kani::assume(y < dim_y);
            let z = kani::any();
            kani::assume(z < dim_z);

            let start = Point { x, y, z };

            let x = kani::any();
            kani::assume(x < dim_x);
            let y = kani::any();
            kani::assume(y < dim_y);
            let z = kani::any();
            kani::assume(z < dim_z);

            let end = Point { x, y, z };
            paths.push((start, end));
        }

    let (filled_maze_ohua, _) = ohua::original::run(dimensions, paths.clone(), 200);

    let maze = stmseq::types::Maze::new(dimensions.clone(), None);
    let filled_maze_seq = stmseq::grid::route_paths(maze, paths);

    assert!(filled_maze_ohua == filled_maze_seq);
    }
}
