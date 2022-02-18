use crate::functions::*;
use std::collections::HashMap;

/**
The global relabeling heuristic up dates the distance function by computing shortest path
distances in the residual graph from all nodes to the sink.
 */
pub fn assign_distance_to_sink_in_residual(
    graph: &Graph,
    mut residual_nodes: HashMap<NodeID, Node>,
    bdsts: Vec<NodeID>,
    distance: u32,
) -> HashMap<NodeID, Node> {
    let mut next_round = Vec::new();
    for bdst in bdsts {
        // local update
        let mut src_node = residual_nodes.get_mut(&bdst).unwrap(); // cannot fail because we added it below
                                                                   // where we already checked.
        src_node.distance_to_sink = distance;

        // extract next
        match graph.bedges.get(&bdst) {
            None => (),
            Some(new_bdsts) => {
                for new_bdst in new_bdsts {
                    let new_bdst_node = residual_nodes.get(&new_bdst);
                    match new_bdst_node {
                        // only residual nodes = residual graph
                        Some(x) if x.height == (graph.nodes.len() as u64) =>
                        // idempotence: select the shortest path.
                        // that is, we have *not* visited this node.
                        {
                            next_round.push(*new_bdst)
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    if next_round.is_empty() {
        residual_nodes
    } else {
        assign_distance_to_sink_in_residual(graph, residual_nodes, next_round, distance + 1)
    }
}

pub fn global_relabel_do(graph: &Graph) -> HashMap<NodeID, Node> {
    // step 1: reset
    let graph_size = graph.nodes.len() as u64;
    let mut residual_nodes = HashMap::new();
    for node in graph.nodes.values() {
        if node.excess > 0 {
            let mut node0 = node.clone();
            node0.reset_height_current(&graph.sink, graph_size);
            residual_nodes.insert(node0.id, node0);
        }
    }

    // step 2: relabel
    assign_distance_to_sink_in_residual(graph, residual_nodes, vec![graph.sink], 0)
}
