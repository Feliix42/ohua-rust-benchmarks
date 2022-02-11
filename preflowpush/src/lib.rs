
// Preflow Push measured here: https://dl.acm.org/doi/pdf/10.1145/2644865.2541964
// Global relabeling described here: http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf


// This paper has the best description:
// http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf
// see page 3

static ALPHA: u32 = 6;

static BETA: u32 = 3;

type NodeID = u32;

#[derive(Debug)]
struct Node {
    // constant node properties
    id : NodeID,
    distance_to_sink : u32,

    // recomputed node properties
    excess: i64,
    height: i64,
    current: NodeID, // the current candidate for a pushing operation
}

impl Default for Node {
    fn default() -> Node {
        Node{id: 0, distance_to_sink: 0, excess: 0, height: 1, current: 0} // excess unset in original code
    }
}

struct Edge {
    dst: NodeID,
    data: i32, // value u
}

struct Graph {
    nodes: HashMap<NodeID,Node>,
    // Each node appears in the set of forward edges and in the set of backward edges:
    // http://i.stanford.edu/pub/cstr/reports/cs/tr/94/1523/CS-TR-94-1523.pdf
    // The value u is associated with each edge

    // forward set {v,w}
    fegdes: HashMap<NodeID, Vec<Edge>>,
    // backward set {w,v }
    bedges: HashMap<NodeID, Vec<Edge>>,
}

struct PreflowPush {
    // graph specification
    graph: Graph,
    sink: NodeID,
    source: NodeID,
    // configuration flag
    global_relabel_interval: i64,
    // global algorithm state
    should_global_relabel: bool,
}


fn reduceCapacity(fedge: &mut Edge, bedge: &mut Edge, amount:i64) {
    fedge.data -= amount;
    bedge.data += amount;
}

impl Graph {

    fn assign_distance_to_sink(&mut self, sink:NodeID) {
        let sink_node = self.nodes.get(sink).unwrap();
        let d = sink_node.distance_to_sink + 1;
        for edge in self.bedges.get(sink).get().unwrap_or(vec![]) {
            let mut src_node = self.nodes.get_mut(edge.dst).unwrap();
            src_node.distance_to_sink = d;
            self.assign_distance_to_sink(edge.dst);
        }
    }

    fn relabel(&self, node: &mut Node) {
        let mut minHeight = i64::MAX;
        let mut minEdge:i64 = 0;

        let mut current:i64 = 0;
        for ii in  self.fedges.get(node.id).unwrap() {
            let dst = self.get(ii.srcOrDst);
            let cap = ii.data;
            if (cap > 0) {
                let dnode_props = dst;
                if (dnode.height < minHeight) {
                    minHeight = dnode.height;
                    minEdge   = current;
                }
            }
            current+=1;
        }

        assert!(minHeight != i64::MAX);
        minHeight+=1;

        let num_nodes = self.nodes.len() as i64;
        if (minHeight < num_nodes) {
            node.height  = minHeight;
            node.current = minEdge;
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
    fn discharge(&mut self, src:NodeID, sink: NodeID) -> (bool, HashSet<NodeID>, Option<(NodeID, Graph)>) {
        let ctxt      = HashSet::new<NodeID>();

        let node  = self.nodes.get_mut(src).unwrap();
        let relabeled = false;

        if (node.excess == 0 || node.height >= (graph.nodes.len() as i64)) {
            return (false, ctxt, Nothing);
        }

        let mut graph0 = Graph::default();
        let mut node0  = node.clone();
        graph0.nodes.put(node0.id, node0);

        while (true) {
            let mut finished        = false;
            let current             = node0.props.current;

            //auto ii = graph.edge_begin(src, flag);
            //auto ee = graph.edge_end(src, flag);

            //std::advance(ii, node.current);

            for edge in self.fedges.get_mut(node.id).unwrap().iter().skip(node0.current) { //(; ii != ee; ++ii, ++current) {
                let dst = edge.dst;
                let cap = edge.data;
                if (cap == 0) { // || current < node.current)
                    continue;
                }

                let dnode = graph.get_mut(dst).unwrap();
                if (node0.height - 1 != dnode.height) {
                    continue;
                }

                let dnode0 = dnode.clone();
                graph0.nodes.put(dnode0.id, dnode0);

                // Push flow
                let amount = std::cmp::min(node0.excess, cap);
                let edge0 = edge.clone();
                graph0.fedges.insert(node.id, edge0);
                reduceCapacity(edge0, amount);

                // Only add once
                if (dst != sink && dst != src && dnode.excess == 0) {
                    ctx.push(dst);
                }

                assert!(node0.excess >= amount);
                node0.excess -= amount;
                dnode0.excess += amount;

                if (node.excess == 0) {
                    finished     = true;
                    node0.current = current;
                    break;
                }
            }

            if (finished) {
                break;
            }

            relabel(src);
            relabeled = true;

            if (node0.height == (graph.nodes.len() as i64)) {
                break;
            }

            // prevHeight = node.height;
        }

        (relabeled, ctxt, Some((src,graph0)))
    }

    /**
    Simple update function that detects collisions and requests a recomputation.
    Essentially implements a transaction!
     */
    fn update(&mut self, updates:Vec<(NodeID,Graph)>) -> HashSet<NodeID> {
        let mut redo = HashSet::new();
        let mut updated = HashSet::new();
        for update in updates {
            let (srcID,graph) = update;
            let touched = update.edges.iter().map(|edge| edge.dst).collect(HashSet::new()).insert(srcID);
            let noConflicts = updated.is_disjoint(touched);
            if(noConflicts){
                // success: replace in graph
                graph.retain(|nID, val| self.insert(nID, val) );
            } else {
                // failure: request redo
                redo.insert(srcID);
            }
            updated.union(touched);
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
}


fn initializePreflow(graph:&mut Graph, source: NodeID) -> HashSet<NodeID> {
    let initial = HashSet::new();
    for mut edge in graph.get_mut(source).unwrap().edges {
        let mut dst = graph.get_mut(ii.dst).unwrap();
        let cap = edge.data_forward;
        reduceCapacity(ii, cap);
        let mut node = dst.props;
        node.excess += cap;
        if (cap > 0) {
            initial.add(dst);
        }
    }
    initial
}

fn reset_node(node: Node, src: NodeID, sink: NodeID, heigth: i64) -> (NodeID, Node) {;
    node.current = 0;
    if (src == sink) {
        node.height = 0;
    } else {
        node.height = height;
    }
    (node.id, node)
}


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

A proper parallel implementation goes along the lines of the following recursive code:

fn bfs(current:HashSet<Node>, graph:Graph) -> Graph {
  let graph_ro = Arc::new(graph);
  let new_nodes = Vec::new();
  for n in current {
    let n_new = compute(n,graph_ro);
    new_nodes.push(n_new);
  }
  graph.update(new_nodes);
  let next = get_next(current, graph_ro);
  if next.is_empty() {
    graph
  } else {
    bfs(next, graph)
  }
}

 */
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

/**
Note that in Galois, EVERY function to the state (graph) has to declare whether the access is UNPROTECTED or WRITE.
That is, it is left to the developer to do the synchronization!
Even more so, the iterators have unclear semantics and there are places in the code where the developer
has to explicitly "lock" state!
 */
fn nondet_discharge(graph:Graph, initial: HashSet<NodeID>, global_relabel_interval: i64) -> Graph {
    let mut counter = Counter::new();

//    // per thread <-- original code comment!
//    const int relabel_interval =
//        global_relabel_interval / galois::getActiveThreads();
//
//    galois::for_each(
//        galois::iterate(initial),
//        [&counter, relabel_interval, this](GNode& src, auto& ctx) {

    let should_global_relabel_new = should_global_relabel;
    for src in initial {
        let mut increment:i64 = 1;
        //graph.acquire(src); --> locking for graph
        let (shouldIncr, wl_new, updates) = graph.discharge(src);
        let redos = graph.update(updates);
        //wl_new.union(redos); // FIXME wl_new must be part of updates!
        let increment = if shouldIncr { 1 + BETA } else { 1 };
        counter += increment;

        // There is certainly no way that we can really enforce this and neither can Galois code!
        // But the condition below does not work on equality. Hence, we do preserve the semantics of the algorithm specification.
        // The only difference being that the parallel version will be different from the sequential version.

        // This paper states that relabeling is done for every node: https://dl.acm.org/doi/pdf/10.1145/1594835.1504181
        // This happens as the last operation in discharge.
        // The paper for the algorithm: https://dl.acm.org/doi/pdf/10.1145/48014.61051
        if (global_relabel_interval > 0 &&
            counter.getLocal() >= relabel_interval) { // local check <-- taken from the source code: counter.getLocal() gets a thread-local value!
            should_global_relabel_new = true;
            // TODO
            //ctx.breakLoop(); ??? SEMANTICS?! --> just a trap into the scheduler for an early exit
            //return;
        }
    }

    // TODO do global relabel here.
    // This can also be performed in a data parallel fashion.
    global_relabel(counter, graph);

    if (wl_new.is_empty()) {
        graph
    } else {
        nondet_discharge(graph, wl_new)
    }
}

fn run(source: NodeID){
    let mut captured_graph = ...;
    let global_relabel_interval = ...;
    let initial = initializePreflow(captured_graph, source);
    let result_graph = nondet_discharge(captured_graph, initial, global_relabel_interval);
    // TODO do something with it here!
}




