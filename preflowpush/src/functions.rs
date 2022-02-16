use algos::global_relabel_do;
use std::collections::hash_map::HashMap;
use std::collections::HashSet;

// Preflow Push measured here: https://dl.acm.org/doi/pdf/10.1145/2644865.2541964
// Global relabeling described here: http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf

// This paper has the best description:
// http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf
// see page 3

//static ALPHA: u32 = 6;

static BETA: u64 = 3;

pub type NodeID = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    // constant node properties
    pub id: NodeID,
    pub distance_to_sink: u32,

    // recomputed node properties
    pub excess: i32,
    pub height: u64,
    current: usize, // the current candidate for a pushing operation
}

impl Default for Node {
    fn default() -> Node {
        Node {
            id: 0,
            distance_to_sink: 0,
            excess: 0,
            height: 1,
            current: 0,
        } // excess unset in original code
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Edge {
    dst: NodeID,
    data: i32, // value u
}

impl Edge {
    #[allow(dead_code)]
    fn get_forward_data(&self) -> i32 {
        self.data
    }

    #[allow(dead_code)]
    fn get_backward_data(&self) -> i32 {
        -self.data
    }

    fn reduce_capacity(&mut self, amount: i32) {
        self.data -= amount;
    }
}

pub struct Graph {
    pub nodes: HashMap<NodeID, Node>,
    // Each node appears in the set of forward edges and in the set of backward edges:
    // http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf
    // The value u is associated with each edge

    // forward set {v,w}
    fedges: HashMap<NodeID, Vec<Edge>>,
    // backward set {w,v }
    pub bedges: HashMap<NodeID, Vec<NodeID>>,

    pub sink: NodeID,
    pub source: NodeID,
}

impl Default for Graph {
    fn default() -> Graph {
        Graph {
            nodes: HashMap::new(),
            fedges: HashMap::new(),
            bedges: HashMap::new(),
            sink: 0,
            source: 0,
        }
    }
}

pub struct PreflowPush {
    // configuration flag
    global_relabel_interval: u64,
    // global algorithm state
    should_global_relabel: bool,
}

impl Node {
    pub fn reset_height_current(&mut self, sink: &NodeID, height: u64) {
        self.current = 0;
        if self.id == *sink {
            self.height = 0;
        } else {
            self.height = height;
        }
    }
}

impl Graph {
    pub fn global_relabel(&mut self, should_relabel: bool) -> HashSet<NodeID> {
        if should_relabel {
            let mut residual_nodes = global_relabel_do(self);
            let mut reactivate = HashSet::new();
            residual_nodes.drain().for_each(|(id, residual_node)| {
                self.nodes.insert(id, residual_node);
                if id != self.sink && id != self.source {
                    reactivate.insert(id);
                }
            });
            reactivate
        } else {
            HashSet::new()
        }
    }

    fn relabel(&self, node: &mut Node) {
        let mut min_height = u64::MAX;
        let mut min_edge: usize = 0;

        let mut current: usize = 0;
        for edge in self.fedges.get(&node.id).unwrap() {
            let dst = self.nodes.get(&edge.dst).unwrap();
            let cap = edge.data;
            if cap > 0 {
                if dst.height < min_height {
                    min_height = dst.height;
                    min_edge = current;
                }
            }
            current += 1;
        }

        assert!(min_height != u64::MAX);
        min_height += 1;

        let num_nodes = self.nodes.len() as u64;
        if min_height < num_nodes {
            node.height = min_height;
            node.current = min_edge;
        } else {
            node.height = num_nodes;
        }
    }

    /**
    The function mutates:
      - the local node
      - the data of the edges
      - the data of the destination nodes
    Vital insight: operations are all commutative!
    Hence, this can be done in parallel and would return as a result a graph that
    contains only these changed nodes an edges.
    The update function would then merge these changes into the existing graph.
    */
    pub fn discharge(&self, src: NodeID) -> (u64, (NodeID, Graph)) {
        let graph_size = self.nodes.len();
        let node = self.nodes.get(&src).unwrap();

        let mut graph0 = Graph::default();
        if node.excess == 0 || node.height >= (graph_size as u64) {
            return (0, (src, graph0));
        }

        // preparation
        let mut node0 = node.clone();
        let mut fedges0 = self.fedges.get(&node0.id).unwrap().clone();
        let mut dnodes = Vec::new();
        for fedge in fedges0.iter_mut().skip(node0.current) {
            let dst = fedge.dst;
            let dnode = self.nodes.get(&dst).unwrap();
            dnodes.push(dnode.clone());
        }

        loop {
            let mut finished = false;
            let current = node0.current;

            for (fedge, mut dnode) in fedges0
                .iter_mut()
                .zip(dnodes.iter_mut())
                .skip(node0.current)
            {
                let cap = fedge.data;
                if cap == 0 {
                    // || current < node.current)
                    continue;
                }

                if node0.height - 1 != dnode.height {
                    continue;
                }

                // Push flow
                let amount = std::cmp::min(node0.excess, cap);

                fedge.reduce_capacity(amount);

                // Add to worklist. moved to update.
                // Only add once
                //                if (dst != sink && dst != src && dnode.excess == 0) {
                //                    ctxt.push(dst);
                //                }

                assert!(node0.excess >= amount);
                node0.excess -= amount;
                dnode.excess += amount;

                if node0.excess == 0 {
                    finished = true;
                    node0.current = current;
                    break;
                }
            }

            // we discharge until at least one node has no excess anymore.
            if finished {
                break;
            }

            self.relabel(&mut node0);

            if node0.height == (graph_size as u64) {
                break;
            }

            // prevHeight = node.height;
        }

        graph0.fedges.insert(node0.id, fedges0);
        graph0.nodes.insert(node0.id, node0);
        dnodes.drain(..).for_each(|n| {
            graph0.nodes.insert(n.id, n);
        });

        (BETA, (src, graph0))
    }

    /**
    Simple update function that detects collisions and requests a recomputation.
    Essentially implements a transaction!
     */
    pub fn update(&mut self, updates: Vec<(NodeID, Graph)>) -> HashSet<NodeID> {
        let mut redo = HashSet::new();
        let mut updated = HashSet::new();
        for update in updates {
            let (src, mut graph) = update;
            let mut touched = HashSet::new();
            graph.fedges.keys().for_each(|u| {
                touched.insert(*u);
            });
            touched.insert(src);
            let no_conflicts = updated.is_disjoint(&touched);

            // 1st reason to end up in new worklist: collision
            if no_conflicts {
                // success: replace in graph
                graph.nodes.drain().for_each(|(n, val)| {
                    self.nodes.insert(n, val);
                });
                graph.fedges.drain().for_each(|(e, val)| {
                    self.fedges.insert(e, val);
                });
                graph.bedges.drain().for_each(|(e, val)| {
                    self.bedges.insert(e, val);
                });

                // 2nd reason to end up in new worklist: excess prop
                for (nid, node) in graph.nodes {
                    if nid != self.sink && nid != self.source && nid != src && node.excess == 0 {
                        redo.insert(nid);
                    }
                }
            } else {
                // failure: request redo
                redo.insert(src);
            }
            updated = updated
                .union(&touched)
                .map(|x| *x)
                .collect::<HashSet<NodeID>>();
        }
        redo
    }

    /**
    Note that there is also an interesting version that exploits the commutativity of the operations!
    It would essentially capture the essence of the update: subtraction or addition.
    And afterwards it would do the application in the graph on update.
    This requires no recomputations triggered by the update function.
    However, it moves certain parts of the computation to the sequential reduction (update) stage.
    The trade-off between the two versions is again interesting:
    - The above code works well for very few collisions.
    - The other code works best when there are a lot of collissions.
    It would be super novel to switch between these two versions at runtime!
     */
    pub fn initialize_preflow(&mut self) -> HashSet<NodeID> {
        let mut initial = HashSet::new();
        for fedge in self.fedges.get_mut(&self.source).unwrap() {
            let mut dnode = self.nodes.get_mut(&fedge.dst).unwrap();
            let cap = fedge.data;
            fedge.reduce_capacity(cap);
            dnode.excess += cap;
            if cap > 0 {
                initial.insert(fedge.dst);
            }
        }
        initial
    }
}

/*
// Challenge: How would one do a BFS through a graph in Ohua?
// The challenge is that some nodes maybe hit twice.
// I believe the way to do this in a data-parallel fashion is to calculate the order before-hand. But that is not all. There is more to do to accomplish this.
fn update_heights_original(graph:&mut Graph, sink:Node) -> Vec<NodeID> {
    let mut ctx = Vec::new();

//    galois::for_each(
//        galois::iterate({sink}), --> start at the sink
    //        [&, this](const GNode& src, auto& ctx) {

    // This is an iteration from sink to source!
    // In Galois, they acquire locks for all values to be updated before doing anything. (See code below.)
    for src in backwards_iterator(sink, graph) {
    // this whole code below exists only to aquire locks!!!

//          if (version != nondet) {
//
//            if (ctx.isFirstPass()) {
//              for (auto ii :
//                   this->graph.edges(src, galois::MethodFlag::WRITE)) {
//                GNode dst = this->graph.getEdgeDst(ii);
//                int64_t rdata =
//                    this->graph.getEdgeData(reverseDirectionEdgeIterator[*ii]);
//                if (rdata > 0) {
//                  this->graph.getData(dst, galois::MethodFlag::WRITE);
//                }
//              }
//            }
//
//            if (version == detDisjoint && ctx.isFirstPass()) {
//              return;
//            } else {
//              this->graph.getData(src, galois::MethodFlag::WRITE);  <-- locking the data structure
//              ctx.cautiousPoint();
//            }
//          }

//          for (auto ii :
//               this->graph.edges(src, useCAS ? galois::MethodFlag::UNPROTECTED
//                                             : galois::MethodFlag::WRITE)) {

        for edge in graph.fedges.get(src).unwrap_or(vec![]) {
            let dst = edge.dst;
            let rdata = edge.data;
            // restrict global relabeling to the residual arcs and residual nodes
            // an arc is considered residual if its capacity is large than 0.
            if (rdata > 0) {
                let dnode = graph.nodes.get_mut(dst).unwrap(); // this->graph.getData(dstdd, galois::MethodFlag::UNPROTECTED);
                let new_height = src.props.height + 1; // this->graph.getData(src, galois::MethodFlag::UNPROTECTED).height + 1;
//              if (useCAS) {
//                int oldHeight = 0;
//                while (newHeight < (oldHeight = node.height)) {
//                  if (__sync_bool_compare_and_swap(&node.height, oldHeight,
//                                                   newHeight)) {
//                    ctx.push(dst); --> push the newly updated node into the loop (this is a while loop essentially!)
//                    break;
//                  }
//                }
                //              } else {
                // the below is a condition that was added by the Galois guys to
                // make this update idempotent. it has the effect that the first update, i.e., which is the shortest path to the sink, wins!
                if (new_height < dnode.height) {
                    dnode.height = new_height;
                    // ctx.push(dst); --> push the newly updated node into the loop (this is a while loop essentially!)
                }
            }
        }
    } // end for
//        }
//        galois::wl<WL>(), galois::disable_conflict_detection(),
//        galois::loopname("updateHeights"));
    ctx
}

fn update_heights(graph:&Graph, sink:Node) -> Vec<NodeID> {
    let mut ctx = Vec::new();
    let mut residual_nodes = Vec::new();
    for src in backwards_iterator(sink, graph) {
        for edge in graph.fedges.get(src).unwrap_or(vec![]) {
            let dst = edge.dst;
            let rdata = edge.data;
            // restrict global relabeling to the residual arcs and residual nodes
            // an arc is considered residual if its capacity is large than 0.
            if (rdata > 0) {
                let dnode = graph.nodes.get_mut(dst).unwrap();
                let new_height = src.props.height + 1;
                // the below is a condition that was added by the Galois guys to
                // make this update idempotent. it has the effect that the first update, i.e., which is the shortest path to the sink, wins!
                // but it seems to be broken because due to the reset of the nodes before, all nodes have the same
                // height! The only way that the height could ever change is by overcoming the conditional guard!
                if (new_height < dnode.height) {
                    dnode.height = new_height;
                }
            }
        }
    }
    ctx
}
 */

/**
In the description of the paper and in the Galois implementation, the global relabel algorithm is a breadth first traversal on the graph starting from the sink.
Why a BFS?
Well, because all the updates to the nodes essentially depend on the graph state before the global relabelling.
The updates to the graphs are actually forward directed. That is, from a source node, we update all its downstream neighbours.
Another way to look at this is to give this algorithm the old graph and instead of doing a backward traversal, we will just create a completely new set of nodes. That is, we perform this operation in functional style!
The result of this: the whole computation is fully data parallel!!!

The only remaining challenge is the fusion of duplicate nodes, as nodes maybe hit multiple times in the BFS.
The algorithm in fact takes always the result from the shortest path to the sink.
The distance to the sink is a property of the node itself and never changes. Hence the merge function is totally trivial.

After some more thinking, I realized that the algorithm of the Galois guys does actually not work at all.
It resets all nodes first to the same height. Afterwards it performs the updateHeights function.
But the update to the destination node is guarded by a condition that says:

     src.height + 1 < dst.height

This guarding is not present in the original description of the algorithm in the paper. I get to the reason why it is there later.
The point is that this condition can never be true because obviously we just reset the node to all have the same height (=num nodes in the graph).

The reason for that guard is interesting:
The challenge in the BFS traversal is that one could hit the same node twice because there are two or more paths to the sink starting from this
node. That is, there are multiple updates to this node. The real challenge in parallelizing this traversal now is to handle these multiple updates to the same node. In other words, the challenge is to detect the order in which the updates are to be applied.
In the original paper (http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf), the idea was to always take the value from the shortest path. The guard of the Galois guys was supposed to ensure this.
Read: "The global relabeling heuristic updates the distance function by computing shortest path distances in the residual graph from all nodes to the sink."
Their idea was that whenever the destination has a higher height than the src then it clearly accumulated more updates (+1) on the BFS traversal.

On yet another note:
Even the calculation seems wrong. It needs to update the source node, not the destination node. The final goal is that the height reflects
the number of nodes on the shortest path to the sink where the sink has height 0.
Here is the definition from the paper:
"
The distance labeling d : V -> N satisfies the following conditions: d(t) = 0 and for every residual arc (v,w), d(v) =< d(w) + 1.
A residual arc (v,w) is admissible if d(v) = d(w) + 1.
"
Where t is the sink.


After talking to my student Lisza, I realized that they essentially mirror the graph.
That is the reason why the sink has actually outgoing edges. They are essentially the backwards edges.
That makes the computation correct.


An Ohua implementation goes along the lines of the following recursive code:

fn bfs(current:HashSet<Node>, graph:Graph) -> Graph {
  let graph_ro = Arc::new(graph);
  let new_nodes = Vec::new();
  for n in current {
    let n_new = compute(n,graph_ro);
    new_nodes.push(n_new);
  }
  graph.update(new_nodes);
  let next = get_next(current, graph_ro); // this step needs to exclude the nodes that we already visited.
  if next.is_empty() {
    graph
  } else {
    bfs(next, graph)
  }
}

The BFS traversal and the core algorithm need to be compiled individually.
 */

/*
fn global_relabel(counter: Counter, graph: Graph, src: NodeID, sink: NodeID) -> Graph {
    let mut new_graph = Graph::new();
    let l = graph.len();

    for node in graph.values() {
//    galois::do_all(
//        galois::iterate(graph),
//        [&](const GNode& src) {
//          Node& node   = graph.getData(src, galois::MethodFlag::UNPROTECTED);
        let (n_id,n) = reset_node(node);
        new_graph.insert(n_id, n);
    }

    //
    new_graph.updateHeights();

    // TODO
      // this just reactivates nodes with execess capacity into the work list
//    galois::do_all(
//        galois::iterate(graph),
//        [&incoming, this](const GNode& src) {
//          Node& node =
//              this->graph.getData(src, galois::MethodFlag::UNPROTECTED);
//          if (src == this->sink || src == this->source ||
//              node.height >= (int)this->graph.size()) {
//            return;
//          }
//          if (node.excess > 0) {
//            incoming.push_back(src);
//          }
//        },
//        galois::loopname("FindWork"));
}
*/

pub struct Counter {
    c: u64,
}

impl Default for Counter {
    fn default() -> Counter {
        Counter { c: 0 }
    }
}

impl Counter {
    pub fn add(&mut self, x: u64) {
        self.c = x
    }

    pub fn detect_global_relabel(&mut self, relabels: Counter, config: &PreflowPush) -> bool {
        let new_c = self.c + relabels.c;
        if new_c > config.global_relabel_interval && config.should_global_relabel {
            self.c = 0;
            true
        } else {
            self.c = new_c;
            false
        }
    }
}
