#![allow(dead_code, unused_mut, non_snake_case)]
use crate::ohua::benchs::*;
use crate::types::Point;
use std::sync::Arc;

// this is the most efficient form that I can think of
fn fill(mut maze: Maze, pairs: Vec<OPoint>, its: u32) -> Maze {
    let mut rs: Vec<OPoint> = Vec::default();
    let m2: Maze = maze.clone();
    let mro: Arc<Maze> = Arc::new(m2);
    for pair0 in pairs {
        let pair: Option<(Point,Point)> = pair0;
        // FIXME This type check seems not be implemented yet.
        //       The test `var multi fail` also does not show the desired result: an error message!
        let mro_c: Arc<Maze> = mro.clone();
        let path: Option<Path> = find_path(mro_c, pair);
        let r: Option<(Point, Point)> = maze.update(path);
        rs.push(r);
    }

    let not_done: bool = rs.calculate_done();
    let new_its: u32 = inc(its);
    //let (new_its, not_done, pending): (u32, bool, Vec<OPoint>) = rs.calculate_done_with_cont(its);

    // FIXME: didn't work!!
    // let (new_its, pending): (u32, Vec<OPoint>) = rs.calculate_done(its);
    // let (not_done, p2): (bool, Vec<OPoint>) = not_done(pending);

    //let pending: Vec<OPoint> = rs.calculate_done();
    //let (new_its, not_done, p2): (u32, bool, Vec<OPoint>) = not_done(pending, its);

    // let new_its_left = decrement(its_left);
    // let new_its_left1 = new_its_left.clone();
    // // let not_done = rs.calculate_done1(new_its_left);
    // let not_done = calculate_done(rs1, new_its_left);
    if not_done {
        fill(maze, rs, new_its)
    } else {
        maze
    }
}

pub fn run(dimensions: Point, pairs: Vec<Option<(Point, Point)>>, max_it: u32) -> (Maze, usize) {
    let maze: Maze = Maze::init(dimensions);
    let new_maze: Maze = fill(maze, pairs, max_it);
    (new_maze, 0)
}
