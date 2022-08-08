use rand::RngCore;

use crate::bayes::query::{Query,Val};
use crate::bayes::data::{DataT,Data};

struct Node {
    index: usize,
    value: u64,
    count: usize,
    vary: Vec<Vary>,
}

struct Vary {
    index: usize,
    most_common_value: u64,
    zero: Option<Node>,
    one: Option<Node>,
}

pub struct AdTree {
    num_var: usize,
    num_record: usize,
    root: Node,
}

trait AdTreeT {
    fn new() -> AdTree;

    /* =============================================================================
     * adtree_make
     * -- Records in dataPtr will get rearranged
     * =============================================================================
     */
    fn make<T:RngCore>(&mut self, data: &Data<T>);

    /* =============================================================================
     * adtree_getCount
     * -- queryVector must consist of queries sorted by id
     * =============================================================================
     */
    fn get_count(&self, queries: &Vec<Query>) -> usize;
}

impl Node {
    fn new(index: usize, value: u64, count: usize, vary: Vec<Vary>) -> Node {
        Node{ index, value, count, vary }
    }

    fn make<T:DataT>(
        parent_index: usize,
        index: usize,
        start: usize,
        num_record: usize,
        value: u64,
        data: &T) -> Node {
        
        let vary = Vec::with_capacity(data.num_var - index + 1);
        for v in (index + 1)..data.num_var {
            vary.put(Vary::make(parent_index, v, start, num_record, &data));
        }

         Node::new(index, value, num_record, vary)
    }

    fn get_count (&self,
          i: usize,
          q: usize,
          queries: Vec<Query>,
          last_query_index: usize,
           adtree: &AdTree
          ) -> usize {
        
        if self.index >= last_query_index {
            self.count
        } else {
            match queries.get(q) {
                None => self.count,
                Some(query) => {
                    assert!(query.index <= last_query_index);
                    let vary = 
                        self.vary
                            .get(query.index - self.index -1)
                            .expect("invariant: can find a vary");
        
                    if query.value == vary.most_common_value {
                
                        /*
                         * We do not explicitly store the counts for the most common value.
                         * We can calculate it by finding the count of the query without
                         * the current (superCount) and subtracting the count for the
                         * query with the current toggled (invertCount).
                         */
                        let num_query = queries.len();
                        let super_count = {
                            let super_queries = Vec::with_capacity(num_query -1);
                
                            for qq in 0..num_query {
                                if qq != q {
                                    super_queries.put(queries.get(qq).expect("invariant"));
                                }
                            }
                            // FIXME this looks like an endless loop because it starts at the top
                            // again!
                            adtree.get_count(&super_queries)
                        };
                
                        let invert_count =
                            match query.value {
                                Val::Zero => {
                                    // FIXME this is no good. it changes the value just for the call below!
                                    query.value = Val::One;
                                    self.get_count(i,
                                               q,
                                               &queries,
                                               last_query_index,
                                               &adtree);
                                    query.value = Val::Zero;
                                }, 
                                Val::One => {
                                     // FIXME this is no good. it changes the value just for the call below!
                                    query.value = Val::Zero;
                                    self.get_count(i,
                                               q,
                                               &queries,
                                               last_query_index,
                                               &adtree);
                                    query.value = Val::One;
                                }
                            };

                        super_count - invert_count
                    } else {
                        match query.value {
                            Val::Zero => 
                                vary.zero_node.get_count(
                                      i + 1,
                                      q + 1,
                                      &queries,
                                      last_query_index,
                                      &adtree),
                            Val::One =>
                                vary.one_node.getCount(
                                      i + 1,
                                      q + 1,
                                      &queries,
                                      last_query_index,
                                      &adtree)
                        }
                    }
                } 
            }
       }
    }
}

impl Vary {
    fn new(index: usize,
        most_common_value: usize,
        zero: Option<Node>,
        one: Option<Node>) -> Vary {
        Vary{ index, most_common_value, zero, one }
    }

    fn make<T:DataT>(parent_index: usize,
          index: usize,
          start: usize,
          num_record: usize,
          mut data: T) -> Vary {
        if (parent_index + 1 != index) && (num_record > 1) {
            data.sort(start, num_record, index);
        }

        let num0 = data.find_split(start, num_record, index);
        let num1 = num_record - num0;

        let most_common_value = if num0 >= num1 { 0 } else { 1 };

        let zero = 
            match num0 == 0 || most_common_value == 0 {
                true => None,
                false => Some(Node::make(index, index, start, num0, 0, &data))
            };
        let one =
            match num1 == 0 || most_common_value == 1 {
                True => None,
                False => Some(Node::make(index, index, start + num0, num1, 1, &data))
            };
       
        Vary::new(index, most_common_value, zero, one)
    }
}

impl AdTreeT for AdTree {
    fn new(num_var: usize, num_record: usize, root: Node) -> AdTree {
        AdTree{ num_var, num_record, root }
    }

    fn make<T:DataT>(mut data: &mut T) -> AdTree {
        let num_record = data.num_record;
        let num_var = data.num_var;

        data.sort(0, num_record, 0);
        let root = Node::make(-1, -1, 0, num_record, &data);
        AdTree::new(num_var, num_record, root)
    }

    fn get_count(&self, queries: &Vec<Query>) -> usize {
        let num_query = queries.len();
        let last_query_index = 
            match queries.last() {
                None => -1,
                Some(last_query) => last_query.index
            };
        self.root.get_count(-1, 0, &queries, last_query_index, &self)
    }
}


#[cfg(test)]
mod test {

    fn count_data (data: &Data, queries: &Vec<Query>) -> usize {
        let mut count = 0;
        for r in 0..(data.num_record) {
            let record = data.get_record(r);
            let mut is_match = true;
            for query in queries.iter() {
                match query.value {
                    Val::WildCard => true,
                    _ => 
                        if query.value != record[query.index] {
                            is_match = false;
                            break            
                        } else {
                            //
                        }
                }
            }
            count = if is_match { count + 1 } else { count };
        }
        count
    }

    fn test_count(
        ad_tree: &AdTree,
        data: &Data,
        queries: &mut Vec<Query>,
        index: usize,
        num_var: usize) {

        if (index >= num_var) {
            // just nothing
        } else {
            let count1 = ad_tree.get_count(&queries);
            let count2 = count_data(&data, &queries);
            assert!(count1 == count2);

            for i in 1..num_var {
                queries.put(Query::new(index + i, 0));
                test_count(&ad_tree, &data, &queries, index + i, num_var);
                queries.pop();

                queries.put(Query::new(index + i, 1));
                test_count(&ad_tree, &data, &queries, index + i, num_var);
                queries.pop();
            }
        }
    }


    fn test_counts(ad_tree: &AdTree, data: &Data){
        let queries = Vec::with_capacity(data.num_var);
        //for (v = -1; v < numVar; v++) {
        for v in 0..num_var {
            test_count(&ad_tree, &data, &mut queries, v, data.num_var);
        }
    }


    fn test(num_var: usize, num_record: usize) {
        let random = Random::new();
        let data = Data::new(num_var, num_record, &random);
        data.generate(0, 10, 10);

        let copy_data = data.clone();

        let ad_tree = AdTree::make(&copy_data);

        test_counts(adtreePtr, dataPtr);
    }

    #[test]
    fn test1() {
        test(3, 8);
    }

    #[test]
    fn test2() {
        test(4, 64);
    }

    #[test]
    fn test3() {
        test(8, 256);
    }

    #[test]
    fn test4() {
        test(12, 256);
    }

    #[test]
    fn test5() {
        test(48, 1024);
    }
}
