use functions::*;
use helpers;
use std::*;

/**
Note that in Galois, EVERY function to the state (graph) has to declare whether the access is UNPROTECTED or WRITE.
That is, it is left to the developer to do the synchronization!
Even more so, the iterators have unclear semantics and there are places in the code where the developer
has to explicitly "lock" state!
 */
pub fn nondet_discharge(
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

    let mut results = Vec::new2();
    let g2 = Arc::new1(graph.clone());
    // let mut relabel_count = Counter::default();
    for src in initial {
        let result = discharge(g2.clone(), src);

        results.push(result);
        // relabel_count.add(relabel_c);

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

    let (relabel_count, wl_new) = graph.update(results);

    let should_global_relabel = counter.detect_global_relabel(relabel_count, preflow);
    //let wl_new = graph.update(updates);
    let wl_new0 = graph.global_relabel(should_global_relabel);

    // wl_new = wl_new.union(&wl_new0).map(|x| *x).collect();
    let (not_empty, wl_new1) = helpers::combine(wl_new, wl_new0);

    if not_empty {
        // !wl_new.is_empty() {
        nondet_discharge(graph, counter, wl_new1, preflow)
    } else {
        graph
    }
}

pub fn run(g: Graph, preflow: PreflowPush) -> Graph {
    let mut graph = id(g);
    let initial = graph.initialize_preflow();
    let result_graph = nondet_discharge(graph, Counter::default(), initial, preflow);
    result_graph
}
