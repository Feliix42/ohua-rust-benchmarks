use crate::benchs::*;
use std::sync::Arc;

fn fill(mut maze: Maze, pairs: Vec<Option<(Point, Point)>>, its: u32) -> Maze {
    let (sx, _rx) = std::sync::mpsc::channel();

    let mut rs = Vec::default();
    // let rs = UnmappedPaths::default();
    let m2 = maze.clone(); // the type check for state threads in Ohua forces me to put this here. this is good!
    let mro = Arc::new(m2);
    for pair in pairs {
        // FIXME This type check seems not be implemented yet.
        //       The test `var multi fail` also does not show the desired result: an error message!
        let path = find_path(mro.clone(), pair);
        let r = maze.update(path, &sx);
        rs.push(r);
    }
    // rs.evict_mapped();
    let rs1 = filter_mapped(rs);
    let rs2 = rs1.clone();
    let (new_its, not_done) = calculate_done(rs1, its);
    // let new_its_left = decrement(its_left);
    // let new_its_left1 = new_its_left.clone();
    // // let not_done = rs.calculate_done1(new_its_left);
    // let not_done = calculate_done(rs1, new_its_left);
    if not_done {
        fill(maze, rs2, new_its)
    } else {
        maze
    }
}

pub fn run(dimensions: Point, pairs: Vec<Option<(Point, Point)>>, max_it: u32) -> (Maze, usize) {
    let maze = Maze::init(dimensions);
    (fill(maze, pairs, max_it), 42)
}
