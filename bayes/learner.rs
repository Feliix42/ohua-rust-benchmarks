struct Task {
    op: Operation,
    from_id: usize,
    to_id: usize,
    score: f32
}

struct Learner {
    ad_tree: AdTree,
    net: Net,
    local_base_log_likelihoods: Vec<f32>,
    base_log_likelihood: f32,
    tasks: Vec<Task>,
    num_total_parent: u64,
}


trait LearnerT {
    fn new(data: &Data, ad_tree: AdTree, num_thread: usize) -> Learner
    fn run(&mut self);
    fn score(&self) -> f32;
}

enum Cmp { Eq(i64), Gt, Lt }

impl Task {

    fn compare(&self, other: &Task) -> Cmp {
        if self.score < other.score {
            Cmp::Lt
        } else {
            if self.score > other.score {
                Cmp::Gt
            } else {
                Cmp::Eq(self.to_id - self.to_id)
            }
        }
    }
}

impl Learner {

}

impl LearnerT for Learner {

    fn new(data: &Data, ad_tree: AdTree, num_thread: usize) -> Leaner {
        Learner {
            ad_tree,
            net: Net::new(data.num_var),
            local_base_log_likelihoods: Vec::with_capacity(data.num_var),
            base_log_likelihood: 0.0,
            tasks: Vec::with_capacity(data.num_var),
            num_total_parent: 0
        }
    }
    
    /* =============================================================================
    * learner_run
    * -- Call AdTree::make before this
    * =============================================================================
    */
    fn run(&mut self) {
        // data parallel operations across a pool of threads
        // thread_start(&createTaskList, (void*)learnerPtr);
        // thread_start(&learnStructure, (void*)learnerPtr);
        
        //for t in 0..self.
        // FIXME
        unimplemented!()
    }


    /* =============================================================================
     * learner_score
     * -- Score entire network
     * =============================================================================
     */
    fn score(&self, ad_tree: AdTree, net: Net) -> f32
    {
        //vector_t* queryVectorPtr = vector_alloc(1);
        //assert(queryVectorPtr);
        //vector_t* parentQueryVectorPtr = vector_alloc(1);
        //assert(parentQueryVectorPtr);
        let mut queries = Vec::with_capacity(1);
        let mut parent_queries = Vec::with_capacity(1);

        //long numVar = adtreePtr->numVar;
        //query_t* queries = (query_t*)malloc(numVar * sizeof(query_t));
        //assert(queries);
        //long v;
        for v in 0..(ad_tree.num_var) {
            queries.push(Query::new(v, Val::WildCard));
        }

        let mut num_total_parent = 0;
        let mut log_likelihood = 0.0;

        for v in 0..(ad_tree.num_var) {
            let parent_id_list = net.get_parent_id_list(v);
            num_total_parent += parent_id_list.len();

            populate_query_vectors(&net,
                                 v,
                                 //queries,
                                 &queries,
                                 &parent_queries);
            let local_log_likelihood = 
                compute_local_log_likelihood(
                    v,
                    &ad_tree,
                    &net,
                    //queries,
                    &queries,
                    &parent_queries);
            log_likelihood += local_log_likelihood;
        }

        let num_record = ad_tree.num_record;
        let penalty = -0.5 * num_total_parent * log(num_record); //(float)(-0.5 * (double)numTotalParent * log((double)numRecord));
        let score = penalty + num_record * log_likelihood;

        score
    }

   
    fn create_task_list (&mut self, id: usize, num_chunks: usize) {
        //TM_THREAD_ENTER();

        //long myId = thread_getId();
        //long numThread = thread_getNumThread();

        //bool_t status;

        //adtree_t* adtreePtr = learnerPtr->adtreePtr;
        //float* localBaseLogLikelihoods = learnerPtr->localBaseLogLikelihoods;
        //learner_task_t* tasks = learnerPtr->tasks;

        // query_t queries[2];
        // vector_t* queryVectorPtr = PVECTOR_ALLOC(2);
        // assert(queryVectorPtr);
        let mut queries = Vec::with_capacity(2);
        // status = vector_pushBack(queryVectorPtr, (void*)&queries[0]);
        // assert(status);

        // query_t parentQuery;
        // vector_t* parentQueryVectorPtr = PVECTOR_ALLOC(1);
        // assert(parentQueryVectorPtr);
        let mut parent_queries = Vec::with_capacity(1);

        //long numVar = adtreePtr->numVar;
        //long numRecord = adtreePtr->numRecord;
        let mut base_log_likelihood = 0.0;
        let penalty = -0.5 * log(self.ad_tree.num_record); /* only add 1 edge */

        //long v;

        let (v_start, v_stop) = create_partition(0, self.ad_tree.num_var, id, num_chunks);

        /*
         * Compute base log likelihood for each variable and total base loglikelihood
        */

        for v in v_start..v_stop {

            let mut local_base_log_likelihood = 0.0;
            queries.push(Query::new(v, 0));
            local_base_log_likelihood +=
                compute_specific_local_log_likelihood(&self.ad_tree,
                                                      &queries,
                                                      &parent_queries);
            {
                let mut q = queries.get_mut(0).expect("Invariant broken");
                q.value = 1;
            }
            local_base_log_likelihood +=
                compute_specific_local_log_likelihood(&self.ad_tree,
                                                      &queries,
                                                      &parent_queries);
            base_log_likelihood += local_base_log_likelihood;
            self.local_base_log_likelihoods.put(local_base_log_likelihood);

        } /* foreach variable */

        //TM_BEGIN();
        //float globalBaseLogLikelihood =
        //    TM_SHARED_READ_F(learnerPtr->baseLogLikelihood);
        //TM_SHARED_WRITE_F(learnerPtr->baseLogLikelihood,
        //                  (baseLogLikelihood + globalBaseLogLikelihood));
        //TM_END();
        let global_base_log_likelihood = self.base_log_likelihood;
        self.base_log_likelihood = base_log_likelihood + global_base_log_likelihood;

        /*
         * For each variable, find if the addition of any edge _to_ it is better
         */

        //status = PVECTOR_PUSHBACK(parentQueryVectorPtr, (void*)&parentQuery);
        //assert(status);
    
        for v in v_start..v_stop {
            /*
             * Compute base log likelihood for this variable
             */
    
           //queries[0].index = v;
            let best_local_index = v;
            let bestLocalLogLikelihood = local_base_log_likelihoods[v];

            //status = PVECTOR_PUSHBACK(queryVectorPtr, (void*)&queries[1]);
            //assert(status);

            for vv in 0..num_var {
                if (vv == v) {
                    //continue;
                } else {
                    let (qi0, qi1) = if (v < vv) { (v,vv) } else { (vv,v) };

                    // FIXME in-place updates if performance suffers

                    queries.push(Query::new(qi0, 0));
                    queries.push(Query::new(qi1, 0));
                    parent_queries.push(Query::new(vv, 0));
                    let new_local_log_likelihood =
                        compute_specific_local_log_likelihood(
                            &ad_tree,
                            &queries,
                            &parent_queries);
                    queries.pop();
                    queries.pop();
                    parent_queries.pop();

                    queries.push(Query::new(qi0, 0));
                    queries.push(Query::new(qi1, 1));
                    parent_queries.push(
                        Query::new(vv, if vv < v { 0 } else { 1 });
                    new_local_log_likelihood +=
                        compute_specific_local_log_likelihood(
                            &ad_tree,
                            &queries,
                            &parent_queries);
                    queries.pop();
                    queries.pop();
                    parent_queries.pop();

                    queries.push(Query::new(qi0, 1));
                    queries.push(Query::new(qi1, 0));
                    parent_queries.push(
                        Query::new(vv, if vv < v { 1 } else { 0 });
                    new_local_log_likelihood +=
                        compute_specific_local_log_likelihood(
                            &ad_tree,
                            &queries,
                            &parent_queries);
                    queries.pop();
                    queries.pop();
                    parent_queries.pop();

                    queries.push(Query::new(qi0, 1));
                    queries.push(Query::new(qi1, 1));
                    parent_queries.push(Query::new(vv, 1);
                    new_local_log_likelihood +=
                        compute_specific_local_log_likelihood(
                            &ad_tree,
                            &queries,
                            &parent_queries);

                    if new_local_log_likelihood > best_local_log_likelihood {
                        best_local_index = vv;
                        best_local_log_likelihood = new_local_log_likelihood;
                    } else {
                        // no update
                    }
                }
            } /* foreach other variable */

            queries.pop();

            if best_local_index != v {
                let log_likelihood = num_record * (base_log_likelihood +
                                                    + best_local_log_likelihood
                                                    - local_base_log_likelihoods[v]);
                let score = penalty + log_likelihood;
                let mut task = tasks.get_mut(v);
                task.op = OPERATION_INSERT;
                task.from_id = best_local_index;
                task.to_id = v;
                task.score = score;
                //TM_BEGIN();
                //status = TMLIST_INSERT(taskListPtr, (void*)taskPtr);
                //TM_END();
                //assert(status);
            } else {
                // no task update
            }

        } /* for each variable */

        //PVECTOR_FREE(queryVectorPtr);
        //PVECTOR_FREE(parentQueryVectorPtr);

        //#ifdef TEST_LEARNER
        //    list_iter_t it;
        //    list_iter_reset(&it, taskListPtr);
        //    while (list_iter_hasNext(&it, taskListPtr)) {
        //        learner_task_t* taskPtr = (learner_task_t*)list_iter_next(&it, taskListPtr);
        //        printf("[task] op=%i from=%li to=%li score=%lf\n",
        //               taskPtr->op, taskPtr->fromId, taskPtr->toId, taskPtr->score);
        //    }
        //#endif /* TEST_LEARNER */
        //
        //    TM_THREAD_EXIT();
    }
}

fn compute_specific_local_log_likelihood(
    ad_tree: &AdTree,
    queries: &Vec<Query>,
    parent_queries: &Vec<Query>
    ) -> f32 {
    let count = ad_tree.get_count(queries);
    if count == 0 {
        0.0
    } else {
        let probability = count / ad_tree.num_record;
        let parent_count = add_tree.get_count(parent_queries);

        assert(parent_count >= count);
        assert(parent_count > 0);

        probability * log(count/parent_count)
    }
}

fn create_partition (min:usize, max: usize, id: usize, n: usize) -> (usize, usize)
{
    long range = max - min;
    long chunk = max(1, ((range + n/2) / n)); /* rounded */
    long start = min + chunk * id;
    long stop;
    if (id == (n-1)) {
        stop = max;
    } else {
        stop = min(max, (start + chunk));
    }

    (start, stop)
}

/* =============================================================================
 * computeLocalLogLikelihoodHelper
 * -- Recursive helper routine
 * =============================================================================
 */
fn compute_local_log_likelihood_helper (
    i: usize,
    //num_parent: usize,
    ad_tree: &AdTree,
    //query_t* queries,
    queries: &mut Vec<Query>,
    parent_queries: &Vec<Query>) -> f32
{
    match parent_queries.get(i) {
        None => compute_specific_local_log_likelihood(
                    &adtree,
                    &queries,
                    &parent_queries),
        Some(parent_query) => {
            //queries[parentIndex].value = 0;
            {
                let mut q = queries.get_mut(parent_query.index).expect("invariant broken");
                q.value = 0;
            }
            let local_log_likelihood = 
                compute_local_log_likelihood_helper(
                    (i + 1),
                    parent_queries.len(), //numParent,
                    &ad_tree,
                    //queries,
                    &queries,
                    &parent_queries);

            //queries[parentIndex].value = 1;
            {
                let mut q = queries.get_mut(parent_query.index).expect("invariant broken");
                q.value = 1;
            }
            local_log_likelihood += 
                compute_local_log_likelihood_helper(
                    (i + 1),
                    parent_queries.len(), //numParent,
                    &ad_tree,
                    //queries,
                    &queries,
                    &parent_queries);

            queries[parentIndex].value = QUERY_VALUE_WILDCARD;

            local_log_likelihood
        }
    }
}


/* =============================================================================
 * computeLocalLogLikelihood
 * -- Populate the query vectors before passing as args
 * =============================================================================
 */
fn compute_local_log_likelihood (
    id: usize,
    ad_tree: &AdTree,
    net: &Net,
    // query_t* queries,
    queries: &Vec<Query>,
    parent_queries: &Vec<Query>) -> f32
{
    //long numParent = vector_getSize(parentQueryVectorPtr);
    //float localLogLikelihood = 0.0;

    // queries[id].value = 0;
    {
        let mut q = queries.get_mut(id).expect("invariant broken");
        q.value = Val::Zero;
    }
    let local_log_likelihood = 
        compute_local_log_likelihood_helper(
            0,
            //numParent,
            &ad_tree,
            //queries,
            &queries,
            &parent_queries);

    // queries[id].value = 1;
    {
        let mut q = queries.get_mut(id).expect("invariant broken");
        q.value = Val::One;
    }
    localLogLikelihood += 
        compute_local_log_likelihood_helper(
            0,
            //numParent,
            &ad_tree,
            //queries,
            &queries,
            &parent_queries);

    // queries[id].value = QUERY_VALUE_WILDCARD;
    {
        let mut q = queries.get_mut(parent_query.index).expect("invariant broken");
        q.value = Val::WildCard;
    }

    local_log_likelihood
}

/* =============================================================================
 * populateParentQuery
 * -- Modifies contents of parentQueryVectorPtr
 * =============================================================================
 */
fn populate_parent_query_vector (
    net: &Net,
    id: usize,
    // query_t* queries,
    // vector_t* parentQueryVectorPtr
    ) -> Vec<Query>
{
    //vector_clear(parentQueryVectorPtr);

    list_t* parentIdListPtr = net_getParentIdListPtr(netPtr, id);
    list_iter_t it;
    list_iter_reset(&it, parentIdListPtr);
    while (list_iter_hasNext(&it, parentIdListPtr)) {
        long parentId = (long)list_iter_next(&it, parentIdListPtr);
        bool_t status = vector_pushBack(parentQueryVectorPtr,
                                        (void*)&queries[parentId]);
        assert(status);
    }
}

/* =============================================================================
 * populateQueryVectors
 * -- Modifies contents of queryVectorPtr and parentQueryVectorPtr
 * =============================================================================
 */
fn populate_query_vectors(
    net: &Net,
    id: usize,
    //query_t* queries,
  //  queries: Vec<Query>,
  //                    vector_t* parentQueryVectorPtr
    ) -> (Vec<Query>, Vec<Query>)
{
    populateParentQueryVector(netPtr, id, queries, parentQueryVectorPtr);

    bool_t status;
    status = vector_copy(queryVectorPtr, parentQueryVectorPtr);
    assert(status);
    status = vector_pushBack(queryVectorPtr, (void*)&queries[id]);
    assert(status);
    vector_sort(queryVectorPtr, &compareQuery);

