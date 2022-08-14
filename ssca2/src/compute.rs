use crate::generator::GraphSDG;
use crate::types::*;

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

// TODO(feliix42): remove `Graph` from the function signature and return it instead.

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

    for i in i_start..i_stop {
        let mut k = i;
        if out_vertex_list_size == 0 && k != 0 {
            while i0 == usize::MAX {
                for j in 0..num_edges_placed {
                    if k == sdg_graph.start_vertex[j] {
                        i0 = j;
                        break;
                    }
                }
                k -= 1;
            }
        }

        if out_vertex_list_size == 0 && k == 0 {
            i0 = 0;
        }

        for j in i0..num_edges_placed {
            if i == graph.num_vertices - 1 {
                break;
            }

            if i != sdg_graph.start_vertex[j] {
                if j > 0 && i == sdg_graph.start_vertex[j - 1] && j - i0 >= 1 {
                    out_vertex_list_size += 1;
                    graph.out_degree[i] += 1;

                    for t in (i0 + 1)..j {
                        if sdg_graph.end_vertex[t] != sdg_graph.end_vertex[t - 1] {
                            out_vertex_list_size += 1;
                            graph.out_degree[i] += 1;
                        }
                    }
                }
                i0 = j;
                break;
            }
        }

        if i == graph.num_vertices - 1 && num_edges_placed - i0 >= 0 {
            out_vertex_list_size += 1;
            graph.out_degree[i] += 1;
            for t in (i0 + 1)..num_edges_placed {
                if sdg_graph.end_vertex[t] != sdg_graph.end_vertex[t - 1] {
                    out_vertex_list_size += 1;
                    graph.out_degree[i] += 1;
                }
            }
        }
    } // for i

    graph.prefix_sums();

    graph.num_directed_edges = out_vertex_list_size;
    graph.out_vertex_list = vec![0; out_vertex_list_size];
    graph.paral_edge_idx = vec![0; out_vertex_list_size];
    graph.out_vertex_list[0] = sdg_graph.end_vertex[0];

    // evaluate out_vertex_list
    i0 = usize::MAX;

    for i in i_start..i_stop {
        let mut k = i;
        while i0 == usize::MAX && k != 0 {
            for j in 0..num_edges_placed {
                if k == sdg_graph.start_vertex[j] {
                    i0 = j;
                    break;
                }
            }
            k -= 1;
        }

        if i0 == usize::MAX && k == 0 {
            // fallback when init fails
            i0 = 0;
        }

        for j in i0..num_edges_placed {
            if i == graph.num_vertices - 1 {
                break;
            }

            if i != sdg_graph.start_vertex[j] {
                if j > 0 && i == sdg_graph.start_vertex[j - 1] && j - i0 >= 1 {
                    let ii = graph.out_vertex_idx[i];
                    let mut r = 0;
                    graph.paral_edge_idx[ii] = i0;
                    graph.out_vertex_list[ii] = sdg_graph.end_vertex[i0];
                    r += 1;

                    for t in (i0 + 1)..j {
                        if sdg_graph.end_vertex[t] != sdg_graph.end_vertex[t - 1] {
                            graph.paral_edge_idx[ii + r] = t;
                            graph.out_vertex_list[ii + r] = sdg_graph.end_vertex[t];
                            r += 1;
                        }
                    }
                }
                i0 = j;
                break;
            }
        } // for j

        if i == graph.num_vertices - 1 {
            let mut r = 0;
            if num_edges_placed - i0 >= 0 {
                let ii = graph.out_vertex_idx[i];
                graph.paral_edge_idx[ii + r] = i0;
                graph.out_vertex_list[ii + r] = sdg_graph.end_vertex[i0];
                r += 1;

                for t in (i0 + 1)..num_edges_placed {
                    if sdg_graph.end_vertex[t] != sdg_graph.end_vertex[t - 1] {
                        graph.paral_edge_idx[ii + r] = t;
                        graph.out_vertex_list[ii + r] = sdg_graph.end_vertex[t];
                        r += 1;
                    }
                }
            }
        }
    } // for i

    // if thread_id == 0
    // normally, deallocate sdg_graph.start_vertex and sdg_graph.end_vertex here
    graph.in_degree = vec![0; graph.num_vertices];
    graph.in_vertex_idx = vec![0; graph.num_vertices];

    // a temporary array to store the implied edges
    let mut implied_edge_list = vec![0; graph.num_vertices * sdg_graph.max_cluster_size];

    // theoretically, call `create_partition` here. (unnecessary here)

    // an auxiliary array to store implied edges, in case we overshoot `max_cluster_size`
    let mut aux_array = vec![Vec::with_capacity(0); graph.num_vertices];

    let (i_start, i_stop) = create_partition(0, graph.num_vertices, thread_id, thread_count);

    for i in i_start..i_stop {
        // inspect adjacency list of vertex i
        for j in graph.out_vertex_idx[i]
            ..((graph.out_vertex_idx[i] as i64 + graph.out_degree[i]) as usize)
        {
            let v = graph.out_vertex_list[j];
            let mut k = graph.out_vertex_idx[v];

            while k < (graph.out_vertex_idx[v] as i64 + graph.out_degree[v]) as usize {
                if graph.out_vertex_list[k] == i {
                    break;
                }
                k += 1;
            }

            if k == (graph.out_vertex_idx[v] as i64 + graph.out_degree[v]) as usize {
                // Transaction!
                // add i to the implied edge list of v
                graph.in_degree[v] += 1;
                if graph.in_degree[v] < sdg_graph.max_cluster_size.try_into().unwrap() {
                    implied_edge_list
                        [((v * sdg_graph.max_cluster_size) as i64 + graph.in_degree[v]) as usize] =
                        i;
                } else {
                    // use auxiliary array to store the implied edge
                    // create an array if it's not present already
                    if graph.in_degree[v] % sdg_graph.max_cluster_size as i64 == 0 {
                        aux_array[v] = vec![0; sdg_graph.max_cluster_size];
                    }
                    aux_array[v]
                        [(graph.in_degree[v] % sdg_graph.max_cluster_size as i64) as usize] = i;
                }
            }
        }
    } // for i

    graph.prefix_sums();

    // if thread_id == 0
    graph.num_undirected_edges = (graph.in_vertex_idx[graph.num_vertices - 1] as i64
        + graph.in_degree[graph.num_vertices - 1]) as usize;
    graph.in_vertex_list = vec![0; graph.num_undirected_edges];

    for i in i_start..i_stop {
        for j in
            graph.in_vertex_idx[i]..((graph.in_vertex_idx[i] as i64 + graph.in_degree[i]) as usize)
        {
            if j - graph.in_vertex_idx[i] < sdg_graph.max_cluster_size {
                graph.in_vertex_list[j] =
                    implied_edge_list[i * sdg_graph.max_cluster_size + j - graph.in_vertex_idx[i]];
            } else {
                graph.in_vertex_list[j] =
                    aux_array[i][(j - graph.in_vertex_idx[i]) % sdg_graph.max_cluster_size];
            }
        }
    }
}
