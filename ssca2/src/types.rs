
pub struct Graph {
    pub num_vertices: usize,
    pub num_edges: usize,

    num_directed_edges: u64,
    num_undirected_edges: u64,

    pub num_int_edges: usize,
    pub num_str_edges: usize,

    pub out_degree: Vec<i64>,
    pub out_vertex_idx: Vec<usize>,
    out_vertex_list: Vec<u64>,
    paral_edge_idx: usize,

    pub in_degree: Vec<i64>,
    in_vertex_idx: usize,
    in_vertex_list: Vec<u64>,

    pub int_weight: Vec<i64>,
    pub str_weight: Vec<u8>,
}

