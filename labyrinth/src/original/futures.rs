use crate::grid::update_maze;
use crate::{inc_stats, is_not_empty, pack_stat, take_n, join};
use super::helpers::{collect_work, create_runtime, spawn_onto_pool, split_evenly};
use crate::types::{Maze, Point};
use std::sync::Arc;

pub fn map_paths(maze: Maze, to_map: Vec<(Point, Point)>, frequency: usize, threadcount: usize, taskcount: usize) -> (Maze, usize) {
    let rt = create_runtime(threadcount);

    transact(maze, to_map, frequency, threadcount, taskcount, rt, 0)
}

fn transact(maze: Maze, to_map: Vec<(Point, Point)>, frequency: usize, threadcount: usize, taskcount: usize, rt: Arc<tokio::runtime::Runtime>, stats: usize) -> (Maze, usize) {
    // worklist absplitten
    let (points, still_to_map) = take_n(to_map, frequency);

    // worklist aufsplitten
    let worklist = split_evenly(points, taskcount);

    // threads spawnen, pool und handles zurück
    let tokio_stuff = spawn_onto_pool(worklist, maze.clone(), rt.clone());

    // handles collecten
    let paths = collect_work(tokio_stuff);


    let (remap_paths, new_maze) = update_maze(maze, paths);
    let to_remap = join(remap_paths.clone(), still_to_map);

    let ct = inc_stats(stats, remap_paths);
    let ret = pack_stat(new_maze.clone(), stats);

    if is_not_empty(to_remap.clone()) {
        transact(new_maze, to_remap, frequency, threadcount, taskcount, rt, ct)
    } else {
        ret
    }
}
