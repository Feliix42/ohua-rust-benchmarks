use crate::element::Triangle;
use crate::mesh::Mesh;
use crate::{compute_cavity, id};
use std::sync::Arc;

pub fn refine(mut mesh: Mesh, bad: Vec<Triangle>) -> Mesh {
    let mut computed = Vec::new();

    let m = Arc::new(mesh.clone());

    for item in bad {
        let cav = compute_cavity(m.clone(), item);
        computed.push(cav);
    }

    let remaining_work = mesh.apply_updates(computed);
    let is_not_empty = mesh.has_more_work();

    if is_not_empty {
        refine(mesh, remaining_work)
    } else {
        mesh
    }
}

pub fn run_refining(mesh: Mesh) -> Mesh {
    let m = id(mesh);
    let bad_queue = m.find_bad();

    refine(m, bad_queue)
}
