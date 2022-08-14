
pub struct Graph {
    pub num_vertices: usize,
    pub num_edges: usize,

    pub num_directed_edges: usize,
    pub num_undirected_edges: usize,

    pub num_int_edges: usize,
    pub num_str_edges: usize,

    pub out_degree: Vec<i64>,
    pub out_vertex_idx: Vec<usize>,
    pub out_vertex_list: Vec<usize>,
    pub paral_edge_idx: Vec<usize>,

    pub in_degree: Vec<i64>,
    pub in_vertex_idx: Vec<usize>,
    pub in_vertex_list: Vec<usize>,

    pub int_weight: Vec<i64>,
    pub str_weight: Vec<u8>,
}

impl Graph {
    pub fn prefix_sums(&mut self) {
        // ignoring all multi-threading stuff for a moment
        for j in 0..self.num_vertices {
            self.out_vertex_idx[j] = self.out_degree[j-1] as usize + self.out_vertex_idx[j-1];
        }
    }
}

