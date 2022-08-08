use crate::bayes::net::{Net, Operation};
use crate::bayes::adtree::AdTree;
use crate::bayes::data::Data;
use crate::bayes::query::{Val, Query};

use std::cmp::Ordering;
use rand::RngCore;

struct Task {
    op: Operation,
    from_id: usize,
    to_id: usize,
    score: f32
}

pub(crate) struct Learner {
    ad_tree: AdTree,
    net: Net,
    local_base_log_likelihoods: Vec<f32>,
    base_log_likelihood: f32,
    tasks: Vec<Task>,
    num_total_parent: u64,
}


trait LearnerT {
    fn new<T:RngCore>(data: &Data<T>, ad_tree: AdTree, num_thread: usize) -> Learner;
    fn run(&mut self);
    fn score(&self) -> f32;
}

// enum Cmp { Eq(i64), Gt, Lt }

// impl Task {
//     fn compare(&self, other: &Task) -> Cmp {
//         if self.score < other.score {
//             Cmp::Lt
//         } else {
//             if self.score > other.score {
//                 Cmp::Gt
//             } else {
//                 Cmp::Eq(self.to_id - other.to_id)
//             }
//         }
//     }
// }

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score - other.score {
            x if x>0 => Ordering::Greater,
            0 => Ordering::Equal,
            _ => Ordering::Less
        }
    }
}

impl Learner {
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
        let penalty = -0.5 * (self.ad_tree.num_record as f64).log(); /* only add 1 edge */

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
            let best_local_log_likelihood = self.local_base_log_likelihoods[v];

            //status = PVECTOR_PUSHBACK(queryVectorPtr, (void*)&queries[1]);
            //assert(status);

            for vv in 0..self.num_var {
                if vv == v {
                    //continue;
                } else {
                    let (qi0, qi1) = if v < vv { (v,vv) } else { (vv,v) };

                    // FIXME in-place updates if performance suffers

                    queries.push(Query::new(qi0, 0));
                    queries.push(Query::new(qi1, 0));
                    parent_queries.push(Query::new(vv, 0));
                    let new_local_log_likelihood =
                        compute_specific_local_log_likelihood(
                            &self.ad_tree,
                            &queries,
                            &parent_queries);
                    queries.pop();
                    queries.pop();
                    parent_queries.pop();

                    queries.push(Query::new(qi0, 0));
                    queries.push(Query::new(qi1, 1));
                    parent_queries.push(
                        Query::new(vv, if vv < v { 0 } else { 1 }));
                    new_local_log_likelihood +=
                        compute_specific_local_log_likelihood(
                            &self.ad_tree,
                            &queries,
                            &parent_queries);
                    queries.pop();
                    queries.pop();
                    parent_queries.pop();

                    queries.push(Query::new(qi0, 1));
                    queries.push(Query::new(qi1, 0));
                    parent_queries.push(
                        Query::new(vv, if vv < v { 1 } else { 0 }));
                    new_local_log_likelihood +=
                        compute_specific_local_log_likelihood(
                            &self.ad_tree,
                            &queries,
                            &parent_queries);
                    queries.pop();
                    queries.pop();
                    parent_queries.pop();

                    queries.push(Query::new(qi0, 1));
                    queries.push(Query::new(qi1, 1));
                    parent_queries.push(Query::new(vv, 1));
                    new_local_log_likelihood +=
                        compute_specific_local_log_likelihood(
                            &self.ad_tree,
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
                let log_likelihood = self.num_record * (base_log_likelihood
                                                    + best_local_log_likelihood
                                                    - self.local_base_log_likelihoods[v]);
                let score = penalty + log_likelihood;
                let mut task = self.tasks.get_mut(v);
                task.op = Operation::Insert;
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

    /* =============================================================================
     * learnStructure
     *
     * Note it is okay if the score is not exact, as we are relaxing the greedy
     * search. This means we do not need to communicate baseLogLikelihood across
     * threads.
     * =============================================================================
     */
    fn learn_structure(&mut self, global_operation_quality_factor: f32)
    {
        // TM_THREAD_ENTER();

        //net_t* netPtr = learnerPtr->netPtr;
        //adtree_t* adtreePtr = learnerPtr->adtreePtr;
        //long numRecord = adtreePtr->numRecord;
        //float* localBaseLogLikelihoods = learnerPtr->localBaseLogLikelihoods;
        //list_t* taskListPtr = learnerPtr->taskListPtr;
    
        let mut visited = Vec::with_capacity(self.ad_tree.num_var);
        visited.fill(false);
        let work_queue = Vec::new();
    
        //long numVar = adtreePtr->numVar;
        let mut queries = Vec::with_capacity(self.ad_tree.num_var);
        
        for v in 0 ..(self.ad_tree.num_var) {
            queries.push(Query::new(v, Val::WildCard));
        }

        let base_penalty:f32 = -0.5 * self.ad_tree.num_record.log();

        // vector_t* queryVectorPtr = PVECTOR_ALLOC(1);
        // assert(queryVectorPtr);
        let queries0 = Vec::with_capacity(1);
        // vector_t* parentQueryVectorPtr = PVECTOR_ALLOC(1);
        // assert(parentQueryVectorPtr);
        let parent_queries = Vec::with_capacity(1);
        // vector_t* aQueryVectorPtr = PVECTOR_ALLOC(1);
        // assert(aQueryVectorPtr);
        // vector_t* bQueryVectorPtr = PVECTOR_ALLOC(1);
        // assert(bQueryVectorPtr);

        //findBestTaskArg_t arg;
        //arg.learnerPtr           = learnerPtr;
        //arg.queries              = queries;
        //arg.queryVectorPtr       = queryVectorPtr;
        //arg.parentQueryVectorPtr = parentQueryVectorPtr;
        //arg.bitmapPtr            = visitedBitmapPtr;
        //arg.workQueuePtr         = workQueuePtr;
        //arg.aQueryVectorPtr      = aQueryVectorPtr;
        //arg.bQueryVectorPtr      = bQueryVectorPtr;

        while let Some(task) = self.tasks.pop() {
            let op = task.op;
            let from_id = task.from_id;
            let to_id = task.to_id;

            /*
             * Check if task is still valid
             */
            let is_task_valid = true;
            match op {
                Operation::Insert => {
                    if self.net.has_edge(&from_id, &to_id) ||
                       self.net.is_path(&to_id, &from_id, &visited, &work_queue)
                    {
                        is_task_valid = false;
                    }
                },
                Operation::Remove => {
                    /* Can never create cycle, so always valid */
                },
                Operation::Reverse => {
                    /* Temporarily remove edge for check */
                    self.net.apply_operation(Operation::Remove, &from_id, &to_id);
                    if self.net.is_path(&from_id, &to_id, &visited, &work_queue)
                    {
                        is_task_valid = false;
                    }
                    self.net.apply_operation(Operation::Insert, &from_id, &to_id);
                }
            }

//#ifdef TEST_LEARNER
//        printf("[task] op=%i from=%li to=%li score=%lf valid=%s\n",
//               taskPtr->op, taskPtr->fromId, taskPtr->toId, taskPtr->score,
//               (isTaskValid ? "yes" : "no"));
//        fflush(stdout);
//#endif

            /*
             * Perform task: update graph and probabilities
             */
    
            if is_task_valid {
                self.net.apply_operation(op, &from_id, &to_id);
            }
    
            let mut delta_log_likelihood = 0.0;
    
            if is_task_valid {

                let new_base_log_likelihood =
                    match op {
                        Operation::Insert => {
                            let (a,b) = populate_query_vectors(&self.net,
                                               &to_id,
                                               &queries);
                            queries0 = a;
                            parent_queries = b;
                            let new_base_log_likelihood =
                                compute_local_log_likelihood(&to_id,
                                                      &self.ad_tree,
                                                      &self.net,
                                                      &queries0,
                                                      &parent_queries);
                            let to_local_base_log_likelihood =
                                local_base_log_likelihoods[to_id];
                            delta_log_likelihood +=
                                to_local_base_log_likelihood - new_base_log_likelihood;
                            local_base_log_likelihoods[to_id] = new_base_log_likelihood;
                            let num_total_parent = self.num_total_parent;
                            self.num_total_parent = num_total_parent + 1;
                            new_base_log_likelihood
                        },
                        Operation::Remove => {
                            let (a,b) = populate_query_vectors(&self.net,
                                                   &from_id,
                                                   &queries);
                            queries0 = a;
                            parent_queries = b;
                            let new_base_log_likelihood =
                                compute_local_log_likelihood(&from_id,
                                                          &self.adtree,
                                                          &self.net,
                                                          &queries0,
                                                          &parent_queries);
                            let from_local_base_log_likelihood =
                                local_base_log_likelihoods[from_id];
                            delta_log_likelihood +=
                                from_local_base_log_likelihood - new_base_log_likelihood;
                            local_base_log_likelihoods[fromId] =
                                              new_base_log_likelihood;
                            let num_total_parent = self.num_total_parent;
                            self.num_total_parent = num_total_parent - 1;
                            new_base_log_likelihood
                        },
                        Operation::Reverse => {
                            let (a,b) = populate_query_vectors(&self.net,
                                                   &from_id,
                                                   &queries);
                            queries0 = a;
                            parent_queries = b;
                            let new_base_log_likelihood =
                                compute_local_log_likelihood(&from_id,
                                                          &self.adtree,
                                                          &self.net,
                                                          &queries0,
                                                          &parent_queries);
                            let from_local_base_log_likelihood =
                                local_base_log_likelihoods[fromId];
                            delta_log_likelihood +=
                                from_local_base_log_likelihood - new_base_log_likelihood;
                            local_base_log_likelihoods[fromId] = new_base_log_likelihood;
                            let (a,b) = populate_query_vectors(&self.net,
                                                   &to_id,
                                                   &queries);
                            queries0 = a;
                            parent_queries = b;
                            let new_base_log_likelihood =
                                compute_local_log_likelihood(&to_Id,
                                                          &self.adtree,
                                                          &self.net,
                                                          &queries0,
                                                          &parent_queries);
                            let to_local_base_log_likelihood =
                                local_base_log_likelihoods[toId];
                            delta_log_likelihood +=
                                to_local_base_log_likelihood - new_base_log_likelihood;
                            local_base_log_likelihoods[toId] = new_base_log_likelihood;
                            new_base_log_likelihood
                        }
                }; /* switch op */
            } /* if isTaskValid */

            /*
             * Update/read globals
             */
    
            let old_base_log_likelihood = self.base_log_likelihood;
            let new_base_log_likelihood = old_base_log_likelihood + delta_log_likelihood;
            self.base_log_likelihood = new_base_log_likelihood;
            let base_log_likelihood = new_base_log_likelihood;
            let num_total_parent = self.num_total_parent;
    
            /*
             * Find next task
             */
    
            let base_score = (num_total_parent * base_penalty)
                               + (num_record * base_log_likelihood);
   
            let new_task = self.find_best_insert_task(
                &queries,
                &queries0,
                &parent_queries,
                &visited_bitmap,
                &work_queue,
                &a_query_vector,
                &b_query_vector
                );
    
            let best_task = 
                if ((new_task.from_id != new_task.toId) &&
                    (new_task.score > (base_score / operation_quality_factor)))
                {
                    Some(new_task)
                } else {
                    None
                };
    
    //#ifdef LEARNER_TRY_REMOVE
    //        TM_BEGIN();
    //        newTask = TMfindBestRemoveTask(TM_ARG  &arg);
    //        TM_END();
    //
    //        if ((newTask.fromId != newTask.toId) &&
    //            (newTask.score > (bestTask.score / operationQualityFactor)))
    //        {
    //            bestTask = newTask;
    //        }
    //#endif /* LEARNER_TRY_REMOVE */
    //
    //#ifdef LEARNER_TRY_REVERSE
    //        TM_BEGIN();
    //        newTask = TMfindBestReverseTask(TM_ARG  &arg);
    //        TM_END();
    //
    //        if ((newTask.fromId != newTask.toId) &&
    //            (newTask.score > (bestTask.score / operationQualityFactor)))
    //        {
    //            bestTask = newTask;
    //        }
    //#endif /* LEARNER_TRY_REVERSE */
            match best_task {
                None => (),
                Some(t) => self.tasks[to_id] = t
            }
    //#ifdef TEST_LEARNER
    //            printf("[new]  op=%i from=%li to=%li score=%lf\n",
    //                   bestTask.op, bestTask.fromId, bestTask.toId, bestTask.score);
    //            fflush(stdout);
    //#endif
        } /* while (tasks) */
    }

    fn find_best_insert_task(&self,
        to_id : usize,
        queries : &Vec<Query>,
        mut queries0 : &mut Vec<Query>,
    //    parent_queries : &Vec<Query>,
        num_total_parent : usize,
        base_penalty : f32,
        base_log_likelihood : f32,
        mut invalid_bitmap : &mut Vec<bool>,
        work_queue : &Vec<Task>,
    //    base_parent_queries : &Vec<Query>,
    //    base_queries :  &Vec<Query>
        ) -> Task
    {
        let parent_queries = populate_parent_query_vector(&self.net, &to_id, &queries);
    
        /*
         * Create base query and parentQuery
         */
    
        let mut base_parent_queries = parent_queries.clone(); 
        let mut base_queries0 = base_parent_queries.clone();
        base_queries0.push(queries[to_id]);
        queries0.sort_by(|a,b| a.cmp(&b)); // FIXME Why does he sort the incoming vector here???
    
        /*
         * Search all possible valid operations for better local log likelihood
         */
    
        let best_from_id = to_id; /* flag for not found */
        let old_local_log_likelihood = self.local_base_log_likelihoods[to_id];
        let best_local_log_likelihood = old_local_log_likelihood;
    
        self.net.find_desendants(&to_id, invalid_bitmap, work_queue);
        let mut from_id = -1;
    
        let parent_id_list = self.net.get_parent_id_list(to_id);
    
        let max_num_edge_learned = self.global_max_num_edge_learned;
    
        if (max_num_edge_learned < 0) ||
            (parent_id_list.size() <= max_num_edge_learned)
        {
            for parent_id in parent_id_list {
                invalid_bitmap[parent_id] = true;
            }
    
            let from_id = invalid_bitmap.first(|a| !a);
            while from_id >= 0 {
    //        while ((fromId = bitmap_findClear(invalidBitmapPtr, (fromId + 1))) >= 0) {
    
                if from_id == to_id {
                    // nothing to do
                } else {
                    base_queries0 = queries0.clone();
                    queries0.push(queries[from_id]);
                    queries0.sort_by(|a,b| a.cmp(&b));
                    base_parent_queries = parent_queries.clone();
                    parent_queries.push(queries[from_id]);
                    parent_queries.sort_by(|a,b| a.cmp(&b));
    
                    let new_local_log_likelihood =
                        compute_local_log_likelihood(to_id,
                                                  &self.adtree,
                                                  &self.net,
                                                  &queries,
                                                  &queries0,
                                                  &parent_queries);
    
                    if new_local_log_likelihood > best_local_log_likelihood {
                        best_local_log_likelihood = new_local_log_likelihood;
                        best_from_id = from_id;
                    }
    
                    from_id = invalid_bitmap.first(|a| !a);
                }
    
            } /* foreach valid parent */
    
        } /* if have not exceeded max number of edges to learn */
    
        /*
         * Return best task; Note: if none is better, fromId will equal toId
         */
    
        let mut  best_task = Task {
            op : Operation::Insert,
            from_id : best_from_id,
            to_id : to_id,
            score : 0.0 
        };
    
        if best_from_id != to_id {
            let num_record = self.adtree.num_record;
            let num_parent = parent_id_list.size() + 1;
            let penalty =
                (num_total_parent + num_parent * self.global_insert_penalty) * base_penalty;
            let log_likelihood = num_record * (base_log_likelihood
                                               + best_local_log_likelihood
                                               - old_local_log_likelihood);
            let best_score = penalty + log_likelihood;
            best_task.score  = best_score;
        }
    
        best_task
    }

}

impl LearnerT for Learner {

    fn new<T:RngCore>(data: &Data<T>, ad_tree: AdTree, num_thread: usize) -> Self {
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
        let penalty = -0.5 * num_total_parent * (num_record as f64).log(); //(float)(-0.5 * (double)numTotalParent * log((double)numRecord));
        let score = penalty + num_record * log_likelihood;

        score
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
        let parent_count = ad_tree.get_count(parent_queries);

        assert!(parent_count >= count);
        assert!(parent_count > 0);

        probability * (count/parent_count).log()
    }
}

fn create_partition (min:usize, max: usize, id: usize, n: usize) -> (usize, usize)
{
    let range = max - min;
    let chunk = max(1, (range + n/2) / n); /* rounded */
    let start = min + chunk * id;
    let stop;
    if id == (n-1) {
        stop = max;
    } else {
        stop = min(max, start + chunk);
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
                    &ad_tree,
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
                    i + 1,
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
                    i + 1,
                    parent_queries.len(), //numParent,
                    &ad_tree,
                    //queries,
                    &queries,
                    &parent_queries);

            queries[parentIndex].value = Val::WildCard;

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
    local_log_likelihood += 
        compute_local_log_likelihood_helper(
            0,
            //numParent,
            &ad_tree,
            //queries,
            &queries,
            &parent_queries);

    // queries[id].value = QUERY_VALUE_WILDCARD;
    {
        let mut q = queries.get_mut(id).expect("invariant broken");
        q.value = Val::WildCard;
    }

    local_log_likelihood
}

/* =============================================================================
 * populateParentQuery
 * =============================================================================
 */
fn populate_parent_query_vector (
    net: &Net,
    id: usize,
    queries: &Vec<Query>,
    ) -> Vec<Query>
{
    //vector_clear(parentQueryVectorPtr);
    let mut parent_queries = Vec::new();

    let parent_ids = net.get_parent_id_list(id);
    for parent_id in parent_ids {
        let status = parent_queries.push(&queries[parent_id]);
        assert!(status);
    }

    parent_queries
}

/* =============================================================================
 * populateQueryVectors
 * =============================================================================
 */
fn populate_query_vectors(
    net: &Net,
    id: usize,
    queries: &Vec<Query>,
    ) -> (Vec<Query>, Vec<Query>)
{
    let parent_queries = populate_parent_query_vector(net, id, queries);
    let mut queries0 = parent_queries.clone();
    queries0.push(&queries[id]);
    queries0.sort_by(|a,b| a.compare(&b));

    (queries0, parent_queries)
}


