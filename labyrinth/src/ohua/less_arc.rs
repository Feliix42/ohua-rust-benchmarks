#![allow(dead_code, unused_mut, non_snake_case)]
use crate::ohua::benchs::*;
use crate::types::Point;
use std::sync::Arc;

// this is the most efficient form that I can think of
fn fill(mut maze: Maze, pairs: Vec<OPoint>, its: u32) -> Maze {

    let mut rs: Vec<Option<Path>> = Vec::default();
    let mro: Arc<Maze> = Arc::new(maze);
    for pair0 in pairs {
        let pair: Option<(Point,Point)> = pair0;
        // FIXME This type check seems not be implemented yet.
        //       The test `var multi fail` also does not show the desired result: an error message!
        let mro_c: Arc<Maze> = mro.clone();
        let path: Option<Path> = find_path(mro_c, pair);
        rs.push(path);
    }

    let mut maz: Maze = rs.unarc(mro);
    let mut res: Vec<OPoint> = maz.update_paths(rs);
    //let (mut maz, paths): (Maze, Vec<Option<Path>>) = seq_arc_unwrap(mro, rs);
    //let res: Vec<OPoint> = maz.update_paths(paths);

    //rs.filter_mapped();
    // let (new_its, not_done, pending): (u32, bool, Vec<OPoint>) = res.calculate_done_with_cont(its);

    let not_done: bool = res.calculate_done();
    let new_its: u32 = inc(its);
    // let new_its_left = decrement(its_left);
    // let new_its_left1 = new_its_left.clone();
    // // let not_done = rs.calculate_done1(new_its_left);
    // let not_done = calculate_done(rs1, new_its_left);
    if not_done {
        fill(maz, res, new_its)
    } else {
        maz
    }
}

pub fn run(dimensions: Point, pairs: Vec<Option<(Point, Point)>>, max_it: u32) -> (Maze, usize) {
    let maze: Maze = Maze::init(dimensions);
    let new_maze: Maze = fill(maze, pairs, max_it);
    (new_maze, 0)
}
