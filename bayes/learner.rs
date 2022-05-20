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
    
    fn createTaskList (&self, id: usize, num_chunks: usize) {
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
            queries.put(Query::new(v, 0));
            local_base_log_likelihood +=
                compute_specific_local_log_likelihood(&self.ad_tree,
                                                      &queries,
                                                      &parent_queries);

            queries[0].value = 1;// FIXME
            localBaseLogLikelihood +=
                compute_specific_local_log_likelihood(&self.ad_tree,
                                                      &queries,
                                                      &parent_queries);

        localBaseLogLikelihoods[v] = localBaseLogLikelihood;
        baseLogLikelihood += localBaseLogLikelihood;

    } /* foreach variable */

    TM_BEGIN();
    float globalBaseLogLikelihood =
        TM_SHARED_READ_F(learnerPtr->baseLogLikelihood);
    TM_SHARED_WRITE_F(learnerPtr->baseLogLikelihood,
                      (baseLogLikelihood + globalBaseLogLikelihood));
    TM_END();

    /*
     * For each variable, find if the addition of any edge _to_ it is better
     */

    status = PVECTOR_PUSHBACK(parentQueryVectorPtr, (void*)&parentQuery);
    assert(status);

    for (v = v_start; v < v_stop; v++) {

        /*
         * Compute base log likelihood for this variable
         */

        queries[0].index = v;
        long bestLocalIndex = v;
        float bestLocalLogLikelihood = localBaseLogLikelihoods[v];

        status = PVECTOR_PUSHBACK(queryVectorPtr, (void*)&queries[1]);
        assert(status);

        long vv;
        for (vv = 0; vv < numVar; vv++) {

            if (vv == v) {
                continue;
            }
            parentQuery.index = vv;
            if (v < vv) {
                queries[0].index = v;
                queries[1].index = vv;
            } else {
                queries[0].index = vv;
                queries[1].index = v;
            }

            float newLocalLogLikelihood = 0.0;

            queries[0].value = 0;
            queries[1].value = 0;
            parentQuery.value = 0;
            newLocalLogLikelihood +=
                computeSpecificLocalLogLikelihood(adtreePtr,
                                                  queryVectorPtr,
                                                  parentQueryVectorPtr);

            queries[0].value = 0;
            queries[1].value = 1;
            parentQuery.value = ((vv < v) ? 0 : 1);
            newLocalLogLikelihood +=
                computeSpecificLocalLogLikelihood(adtreePtr,
                                                  queryVectorPtr,
                                                  parentQueryVectorPtr);

            queries[0].value = 1;
            queries[1].value = 0;
            parentQuery.value = ((vv < v) ? 1 : 0);
            newLocalLogLikelihood +=
                computeSpecificLocalLogLikelihood(adtreePtr,
                                                  queryVectorPtr,
                                                  parentQueryVectorPtr);

            queries[0].value = 1;
            queries[1].value = 1;
            parentQuery.value = 1;
            newLocalLogLikelihood +=
                computeSpecificLocalLogLikelihood(adtreePtr,
                                                  queryVectorPtr,
                                                  parentQueryVectorPtr);

            if (newLocalLogLikelihood > bestLocalLogLikelihood) {
                bestLocalIndex = vv;
                bestLocalLogLikelihood = newLocalLogLikelihood;
            }

        } /* foreach other variable */

        PVECTOR_POPBACK(queryVectorPtr);

        if (bestLocalIndex != v) {
            float logLikelihood = numRecord * (baseLogLikelihood +
                                                + bestLocalLogLikelihood
                                                - localBaseLogLikelihoods[v]);
            float score = penalty + logLikelihood;
            learner_task_t* taskPtr = &tasks[v];
            taskPtr->op = OPERATION_INSERT;
            taskPtr->fromId = bestLocalIndex;
            taskPtr->toId = v;
            taskPtr->score = score;
            TM_BEGIN();
            status = TMLIST_INSERT(taskListPtr, (void*)taskPtr);
            TM_END();
            assert(status);
        }

    } /* for each variable */

    PVECTOR_FREE(queryVectorPtr);
    PVECTOR_FREE(parentQueryVectorPtr);

#ifdef TEST_LEARNER
    list_iter_t it;
    list_iter_reset(&it, taskListPtr);
    while (list_iter_hasNext(&it, taskListPtr)) {
        learner_task_t* taskPtr = (learner_task_t*)list_iter_next(&it, taskListPtr);
        printf("[task] op=%i from=%li to=%li score=%lf\n",
               taskPtr->op, taskPtr->fromId, taskPtr->toId, taskPtr->score);
    }
#endif /* TEST_LEARNER */

    TM_THREAD_EXIT();
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


