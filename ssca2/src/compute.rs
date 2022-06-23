use crate::types::*;
use crate::generator::GraphSDG;

/// Originally implemented in `createPartition.c`, this function yields 2 pointers to the start and
/// end.
pub fn create_partition(min: usize, max: usize, id: usize, n: usize) -> (usize, usize) {
    let range = max - min;
    let chunk = std::cmp::max(1, (range + n / 2) / n); // rounded
    let start = min + chunk * id;
    let stop = if id == (n - 1) {
        max
    } else {
        std::cmp::min(max, start + chunk)
    };

    (start, stop)
}

/// Kernel 1, compuites the graph data structure
pub fn compute_graph(graph: &mut Graph, mut sdg_graph: GraphSDG) {
    let thread_id = 0;
    let thread_count = 1;

    // First, determine the number of vertices by scanning the tuple `start_vertex` list

    let num_edges_placed = sdg_graph.num_edges_placed;
    let (i_start, i_stop) = create_partition(0, num_edges_placed as usize, thread_id, thread_count);

    let mut max_num_vertices = 0;
    for i in i_start..i_stop {
        if sdg_graph.start_vertex[i] > max_num_vertices {
            max_num_vertices = sdg_graph.start_vertex[i];
        }
    }

    // FIXME(feliix42): This makes no sense in single-threaded runtimes
    let mut global_max_num_vertices = 0;
    global_max_num_vertices = std::cmp::max(max_num_vertices, global_max_num_vertices) + 1;

    max_num_vertices = global_max_num_vertices;

    // TODO(feliix42): This may be useless in our parallelism scenario
    if thread_id == 0 {
        graph.num_vertices = max_num_vertices;
        graph.num_edges = num_edges_placed;
        graph.int_weight = sdg_graph.int_weight;
        graph.str_weight = sdg_graph.str_weight;

        for i in 0..num_edges_placed {
            let idx = (num_edges_placed - i - 1) as usize;
            if graph.int_weight[idx] < 0 {
                graph.num_str_edges = (graph.int_weight[idx].abs() + 1) as usize;
                break;
            }
        }

        // initialize the arrays for the in-/ and out-vertices
        // NOTE(feliix42): In the original version, this was done in parallel
        graph.out_degree = vec![0; graph.num_vertices];
        graph.in_degree = vec![0; graph.num_vertices];
    }

    //let (i_start, i_stop) = create_partition(0, graph.num_vertices, thread_id, thread_count);
    //for i in i_start..i_stop {
        //graph.out_degree[i] = 0;
        //graph.out_vertex_idx[i] = 0;
    //}

    let mut out_vertex_list_size = 0;
    let mut i0 = usize::MAX;

    // TODO: cont on #232, computeGraph.c
}
