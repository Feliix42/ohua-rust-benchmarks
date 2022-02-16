use functions::*;
use std::collections::{HashMap, HashSet};

/**
The global relabeling heuristic up dates the distance function by computing shortest path
distances in the residual graph from all nodes to the sink.
 */
fn assign_distance_to_sink_in_residual(
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

/**
Note that in Galois, EVERY function to the state (graph) has to declare whether the access is UNPROTECTED or WRITE.
That is, it is left to the developer to do the synchronization!
Even more so, the iterators have unclear semantics and there are places in the code where the developer
has to explicitly "lock" state!
 */
fn nondet_discharge(
    mut graph: Graph,
    mut counter: Counter,
    initial: HashSet<NodeID>,
    preflow: PreflowPush,
) -> Graph {
    //let mut counter = Counter::new();

    //    // per thread <-- original code comment!
    //    const int relabel_interval =
    //        global_relabel_interval / galois::getActiveThreads();
    //
    //    galois::for_each(
    //        galois::iterate(initial),
    //        [&counter, relabel_interval, this](GNode& src, auto& ctx) {

    let mut updates = Vec::new();
    let mut relabel_count = Counter::default();
    for src in initial {
        let (relabel_c, results) = graph.discharge(src);
        updates.push(results);
        relabel_count.add(relabel_c);

        // There is certainly no way that we can really enforce this and neither can Galois code!
        // But the condition below does not work on equality. Hence, we do preserve the semantics of the algorithm specification.
        // The only difference being that the parallel version will be different from the sequential version.

        // This paper states that relabeling is done for every node: https://dl.acm.org/doi/pdf/10.1145/1594835.1504181
        // This happens as the last operation in discharge.
        // The paper for the algorithm: https://dl.acm.org/doi/pdf/10.1145/48014.61051

        //        if (global_relabel_interval > 0 &&
        //            counter.getLocal() >= relabel_interval) { // local check <-- taken from the source code: counter.getLocal() gets a thread-local value!
        //            should_global_relabel_new = true;
        //            //ctx.breakLoop(); ??? SEMANTICS?! --> just a trap into the scheduler for an early exit
        //            //return;
        //        }
    }

    let should_global_relabel = counter.detect_global_relabel(relabel_count, &preflow);
    let mut wl_new = graph.update(updates);
    let wl_new0 = graph.global_relabel(should_global_relabel);

    wl_new = wl_new.union(&wl_new0).map(|x| *x).collect();

    if wl_new.is_empty() {
        graph
    } else {
        nondet_discharge(graph, counter, wl_new, preflow)
    }
}

#[allow(dead_code)]
fn run(mut graph: Graph, preflow: PreflowPush) -> Graph {
    let initial = graph.initialize_preflow();
    let result_graph = nondet_discharge(graph, Counter::default(), initial, preflow);
    result_graph
}
