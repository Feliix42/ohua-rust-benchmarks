




static ALPHA: u32 = 6;

static BETA: u32 = 3;

type NodeID = u32;

#[derive(Debug)]
struct Node {
    id : NodeID,
    excess: i64,
    height: i64,
    current: i64,
}

impl Default for Node {
    fn default() -> Node {
        Node{id: 0, height: 1, current:0, excess:0} // excess unset in original code
    }
}

struct Edge {
    srcOrDst: NodeID
    data: i32,
}

struct Graph {
    nodes: Vec<Node>,
    fegdes: HashMap<NodeID, HashMap<NodeID,Edge>>,
    bedges: HashMap<NodeID, HashMap<NodeID,Edge>>,
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

// TODO can be run in parallel but requires some rewriting to become data parallel.
fn update_heights(graph:Graph) -> Graph {
//    galois::for_each(
//        galois::iterate({sink}),
    //        [&, this](const GNode& src, auto& ctx) {

    // FIXME This is an iteration from sink to source!
    // In Galois, they acquire locks for all values to be updated before doing anything. (See code below.)
    for src in graph.values() {
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

    for edge in src.edges {
        let dst = edge.dst;
        let rdata = edge.data_backward;
        if (rdata > 0) {
            let dnode = graph.get(dst).unwrap(); // this->graph.getData(dstdd, galois::MethodFlag::UNPROTECTED);
            let new_height = src.props.height + 1; // this->graph.getData(src, galois::MethodFlag::UNPROTECTED).height + 1;
//              if (useCAS) {
//                int oldHeight = 0;
//                while (newHeight < (oldHeight = node.height)) {
//                  if (__sync_bool_compare_and_swap(&node.height, oldHeight,
//                                                   newHeight)) {
//                    ctx.push(dst);
//                    break;
//                  }
//                }
//              } else {
                if (newHeight < dnode.height) {
                  dnode.height = newHeight;
                  ctx.push(dst);
                }
              }
            }
          } // end for
        }
//        galois::wl<WL>(), galois::disable_conflict_detection(),
//        galois::loopname("updateHeights"));

}

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

    new_graph.updateHeights();

    galois::do_all(
        galois::iterate(graph),
        [&incoming, this](const GNode& src) {
          Node& node =
              this->graph.getData(src, galois::MethodFlag::UNPROTECTED);
          if (src == this->sink || src == this->source ||
              node.height >= (int)this->graph.size()) {
            return;
          }
          if (node.excess > 0) {
            incoming.push_back(src);
          }
        },
        galois::loopname("FindWork"));
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




