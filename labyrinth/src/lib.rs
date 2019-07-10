pub mod parser;
pub mod pathfinder;
pub mod types;

#[cfg(feature = "transactional")]
pub mod stm_grid;

#[cfg(not(feature = "transactional"))]
pub mod grid;

#[cfg(feature = "ohua")]
pub fn increase(
    inp: (u32, u32),
    remap_paths: Vec<(crate::types::Point, crate::types::Point)>,
) -> (u32, u32) {
    let (rollbacks, iterations) = inp;

    (rollbacks + remap_paths.len() as u32, iterations + 1)
}

#[cfg(feature = "ohua")]
pub fn inc_stats(
    rollbacks: usize,
    remap_paths: Vec<(crate::types::Point, crate::types::Point)>,
) -> usize {
    rollbacks + remap_paths.len()
}

#[cfg(feature = "ohua")]
pub fn pack(maze: crate::types::Maze, stats: (u32, u32)) -> (crate::types::Maze, (u32, u32)) {
    (maze, stats)
}

#[cfg(feature = "ohua")]
pub fn pack_stat(maze: crate::types::Maze, stats: usize) -> (crate::types::Maze, usize) {
    (maze, stats)
}

#[cfg(feature = "ohua")]
pub fn init_tup() -> (u32, u32) {
    (0, 1)
}

#[cfg(feature = "ohua")]
pub fn is_not_empty(v: Vec<(crate::types::Point, crate::types::Point)>) -> bool {
    !v.is_empty()
}

#[cfg(feature = "ohua")]
pub fn join<T>(mut v1: Vec<T>, mut v2: Vec<T>) -> Vec<T> {
    v1.append(&mut v2);
    v1
}

#[cfg(feature = "ohua")]
pub fn take_n<T>(mut v: Vec<T>, count: usize) -> (Vec<T>, Vec<T>) {
    let split = if count <= v.len() { count } else { v.len() };
    let v2 = v.split_off(split);
    (v, v2)
}
