use main::compute_cavity;
use element::Triangle;
use mesh::*;
use std::*;

pub fn refine(mut mesh: Mesh, bad: Vec<Triangle>) -> Mesh {
    let mut computed = Vec::new2();

    let m = Arc::new1(mesh.clone());

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

pub fn run_refine(mesh: Mesh) -> Mesh {
    let m = id(mesh);
    let bad = m.find_bad();

    refine(m, bad)
}

