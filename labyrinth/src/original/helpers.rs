use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};
use crate::types::{Maze, Path, Point};
use crate::pathfinder::find_path;


/// Splits the input vector into evenly sized vectors for `taskcount` workers.
pub fn split_evenly(mut points: Vec<(Point, Point)>, taskcount: usize) -> Vec<Vec<(Point, Point)>> {
    let l = points.len() / taskcount;
    let mut rest = points.len() % taskcount;

    let mut paths_to_map = vec![Vec::with_capacity(l); taskcount];

    for t_num in 0..taskcount {
        if rest > 0 {
            paths_to_map[t_num] = points.split_off(points.len() - l - 1);
            rest -= 1;
        } else {
            if points.len() <= l {
                paths_to_map[t_num] = points.split_off(0);
            } else {
                paths_to_map[t_num] = points.split_off(points.len() - l);
            }
        }
    }

    paths_to_map
}

pub fn vec_pathfind(maze: Arc<Maze>, mut points: Vec<(Point, Point)>) -> Vec<Option<Path>> {
    points.drain(..).map(|p| find_path(&maze, p)).collect()
}

pub fn spawn_onto_pool(
    mut worklist: Vec<Vec<(Point, Point)>>,
    maze: Maze,
    rt: Arc<Runtime>,
) -> (Arc<Runtime>, Vec<Receiver<Vec<Option<Path>>>>) {
    let maze = Arc::new(maze);

    let mut handles = Vec::with_capacity(worklist.len());

    for lst in worklist.drain(..) {
        let m = maze.clone();
        let (sx, rx) = mpsc::channel();

        rt.spawn(async move { sx.send(vec_pathfind(m, lst)).unwrap() });

        handles.push(rx);
    }

    (rt, handles)
}

pub fn create_runtime(threadcount: usize) -> Arc<Runtime> {
    Arc::new(
        Builder::new()
            .threaded_scheduler()
            .core_threads(threadcount)
            .thread_name("ohua-tokio-worker")
            .build()
            .unwrap(),
    )
}

pub fn collect_work<T>(tokio_data: (Arc<Runtime>, Vec<Receiver<Vec<T>>>)) -> Vec<T> {
    let (_rt, mut receivers) = tokio_data;
    receivers
        .drain(..)
        .map(|h| h.recv().unwrap())
        .flatten()
        .collect()
}
