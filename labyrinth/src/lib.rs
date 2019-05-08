pub mod parser;
pub mod pathfinder;
pub mod types;

#[cfg(feature = "transactional")]
pub mod stm_grid;

#[cfg(not(feature = "transactional"))]
pub mod grid;

#[cfg(feature = "ohua")]
pub fn increase(
    collision_count: u32,
    remap_paths: Vec<(crate::types::Point, crate::types::Point)>,
) -> u32 {
    collision_count + remap_paths.len() as u32
}

#[cfg(feature = "ohua")]
pub fn pack(maze: crate::types::Maze, collision_count: u32) -> (crate::types::Maze, u32) {
    (maze, collision_count)
}
