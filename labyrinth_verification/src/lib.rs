pub mod parser;
pub mod types;
pub mod stmseq;
pub mod ohua;

pub use stmseq::*;

#[cfg(kani)]
mod verification {
    use super::*;
    use crate::types::Point;

    #[kani::proof]
    #[kani::unwind(2)]
    pub fn check_seq_unwrap() {
        const dim_x: usize = 3;
        const dim_y: usize = 3;
        const dim_z: usize = 2;
        //let dim_x: usize = kani::any();
        //let dim_y: usize = kani::any();
        //let dim_z: usize = kani::any();

        const dimensions: Point = Point { x: dim_x, y: dim_y, z: dim_z };

        //let mut paths = Vec::new();
        //for _ in 0..2 {
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
            //paths.push((start, end));
        //}
        let paths = vec![Some((start, end))];
        kani::assume(paths.len() == 1);

        //let p2 = paths.clone().into_iter().map(|x| Some(x)).collect();
        let (filled_maze_ohua, _) = ohua::less_arc::run(dimensions, paths, 200);

        //let maze = stmseq::types::Maze::new(dimensions.clone(), None);
        //let filled_maze_seq = stmseq::grid::route_paths(maze, paths);

        //assert!(filled_maze_ohua.grid == filled_maze_seq.grid);
    }
}
