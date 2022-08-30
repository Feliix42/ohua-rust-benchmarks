use rand::{RngCore, SeedableRng};

use crate::bayes::data::{Data, DataT};
use crate::bayes::query::{QueryT, Val};

pub(crate) struct RootNode {
    count: usize,
    vary: Vec<Vary>,
}

struct TreeNode {
    index: usize,
    value: u64, // this could be just a bool!
    count: usize,
    vary: Vec<Vary>,
}

struct Vary {
    index: usize,
    most_common_value: Val,
    zero: Option<TreeNode>,
    one: Option<TreeNode>,
}

pub struct AdTree {
    pub(crate) num_var: usize,
    pub(crate) num_record: usize,
    root: RootNode,
}

pub(crate) trait AdTreeT {
    fn new(num_var: usize, num_record: usize, root: RootNode) -> Self;

    /* =============================================================================
     * adtree_make
     * -- Records in dataPtr will get rearranged
     * =============================================================================
     */
    fn make<T: RngCore + SeedableRng>(data: &mut Data<T>) -> Self;

    /* =============================================================================
     * adtree_getCount
     * -- queryVector must consist of queries sorted by id
     * =============================================================================
     */
    fn get_count<T: QueryT>(&self, queries: &mut Vec<T>) -> usize;
}

impl RootNode {
    fn new(count: usize, vary: Vec<Vary>) -> RootNode {
        RootNode { count, vary }
    }

    fn make<T: RngCore + SeedableRng>(num_record: usize, data: &mut Data<T>) -> RootNode {
        let mut vary = Vec::with_capacity(data.num_var);
        for v in 0..data.num_var {
            vary.push(Vary::make(v, 0, num_record, data));
        }

        RootNode::new(num_record, vary)
    }

    fn get_count<T: QueryT>(
        &self,
        //  i: usize,
        q: usize,
        // it is a pity that I have to make `queries` mutable because all
        // I actually want to change are the elements but not the vector itself.
        queries: &mut Vec<T>,
        last_query_index: Option<usize>,
        adtree: &AdTree,
    ) -> usize {
        match last_query_index {
            None => 0,
            Some(last_query_index0) => {
                if queries.get(q).is_none() {
                    self.count
                } else {
                    let (index, val, num_query) = {
                        let query = queries.get(q).expect("impossible");
                        (query.index(), query.val().clone(), queries.len())
                    };
                    assert!(index <= last_query_index0);
                    let vary0 = self.vary.get(index).expect("invariant: can find a vary");

                    if val == vary0.most_common_value {
                        /*
                         * We do not explicitly store the counts for the most common value.
                         * We can calculate it by finding the count of the query without
                         * the current (superCount) and subtracting the count for the
                         * query with the current toggled (invertCount).
                         */
                        let super_count = {
                            let mut super_queries = Vec::with_capacity(num_query - 1);

                            for qq in 0..num_query {
                                if qq != q {
                                    super_queries.push(queries.get(qq).expect("invariant").clon());
                                }
                            }
                            // FIXME this looks like an endless loop because it starts at the top
                            // again!
                            // (but it has a different set of queries this time.)
                            adtree.get_count(&mut super_queries)
                        };

                        let invert_count = match val {
                            Val::Zero => {
                                {
                                    let query = queries.get_mut(q).expect("impossible");
                                    // FIXME this is no good. it changes the value just for the call below!
                                    query.update_val(Val::One);
                                }
                                let c = self.get_count(q, queries, last_query_index, adtree);
                                {
                                    let query = queries.get_mut(q).expect("impossible");
                                    query.update_val(Val::Zero);
                                }
                                c
                            }
                            _ => {
                                {
                                    let query = queries.get_mut(q).expect("impossible");
                                    // FIXME this is no good. it changes the value just for the call below!
                                    query.update_val(Val::Zero);
                                }
                                let c = self.get_count(q, queries, last_query_index, adtree);
                                {
                                    let query = queries.get_mut(q).expect("impossible");
                                    query.update_val(Val::One);
                                }
                                c
                            }
                        };

                        super_count - invert_count
                    } else {
                        match val {
                            Val::Zero => vary0.zero.as_ref().map_or(0, |n| {
                                n.get_count(0, q + 1, queries, last_query_index, adtree)
                            }),
                            Val::One => vary0.one.as_ref().map_or(0, |n| {
                                n.get_count(0, q + 1, queries, last_query_index, adtree)
                            }),
                            Val::WildCard => panic!("Hit WildCard. Not supported."),
                        }
                    }
                }
            }
        }
    }
}

impl TreeNode {
    fn new(index: usize, value: u64, count: usize, vary: Vec<Vary>) -> TreeNode {
        TreeNode {
            index,
            value,
            count,
            vary,
        }
    }

    fn make<T: RngCore + SeedableRng>(
        // parent_index: usize,
        index: usize,
        start: usize,
        num_record: usize,
        value: u64,
        data: &mut Data<T>,
    ) -> TreeNode {
        let mut vary = Vec::with_capacity(data.num_var - index + 1);
        for v in (index + 1)..data.num_var {
            vary.push(Vary::make(
                //parent_index,
                v, start, num_record, data,
            ));
        }

        TreeNode::new(index, value, num_record, vary)
    }

    // TODO abstract over the type of the Node
    fn get_count<T: QueryT>(
        &self,
        i: usize,
        q: usize,
        queries: &mut Vec<T>,
        last_query_index: Option<usize>,
        adtree: &AdTree,
    ) -> usize {
        match last_query_index {
            None => self.count,
            Some(last_query_index0) => {
                if self.index > last_query_index0 {
                    self.count
                } else {
                    if queries.get(q).is_none() {
                        self.count
                    } else {
                    let (index, val, num_query) = {
                        let query = queries.get(q).expect("impossible");
                        (query.index(), query.val().clone(), queries.len())
                    };
                           assert!(index <= last_query_index0);
                            let vary0 = self
                                .vary
                                .get(index - self.index - 1)
                                .expect("invariant: cannot find a vary");

                            if val == vary0.most_common_value {
                                /*
                                 * We do not explicitly store the counts for the most common value.
                                 * We can calculate it by finding the count of the query without
                                 * the current (superCount) and subtracting the count for the
                                 * query with the current toggled (invertCount).
                                 */
                                let super_count = {
                                    let mut super_queries =
                                        Vec::with_capacity(num_query - 1);

                                    for qq in 0..num_query {
                                        if qq != q {
                                            /*
                                             * Note on cloning:
                                             * This is totally ok because the recursion never
                                             * changes the structure for good. It just alters it
                                             * for the recursive call and then resets it to the
                                             * initial value.
                                             */
                                            super_queries
                                                .push(queries.get(qq).expect("invariant").clon());
                                        }
                                    }
                                    // FIXME this looks like an endless loop because it starts at the top
                                    // again!
                                    adtree.get_count(&mut super_queries)
                                };

                                let invert_count = match val {
                                    Val::Zero => {
                                        // FIXME this is no good. it changes the value just for the call below!
                                        // due to these mutable operations we have to always
                                        // clone because the population in the learner would borrow
                                        // the queries to queries0 and parent_queries. sadly,
                                        // the code wants to run *this* function on both of the
                                        // query vectors!
                                        {
                                            let query = queries.get_mut(q).expect("impossible");
                                            query.update_val(Val::One);
                                        }
                                        let c =
                                            self.get_count(i, q, queries, last_query_index, adtree);
                                        {
                                            let query = queries.get_mut(q).expect("impossible");
                                            query.update_val(Val::Zero);
                                        }
                                        c
                                    }
                                    _ => {
                                        // FIXME this is no good. it changes the value just for the call below!
                                        {
                                            let query = queries.get_mut(q).expect("impossible");
                                            query.update_val(Val::Zero);
                                        }
                                        let c =
                                            self.get_count(i, q, queries, last_query_index, adtree);
                                        {
                                            let query = queries.get_mut(q).expect("impossible");
                                            query.update_val(Val::One);
                                        }
                                        c
                                    }
                                };

                                super_count - invert_count
                            } else {
                                match val {
                                    Val::Zero => vary0.zero.as_ref().map_or(0, |n| {
                                        n.get_count(i + 1, q + 1, queries, last_query_index, adtree)
                                    }),
                                    Val::One => vary0.one.as_ref().map_or(0, |n| {
                                        n.get_count(i + 1, q + 1, queries, last_query_index, adtree)
                                    }),
                                    Val::WildCard => panic!("Hit WildCard. Not supported."),
                                }
                            }
                        }
                }
            }
        }
    }
}

impl Vary {
    fn new(
        index: usize,
        most_common_value: Val,
        zero: Option<TreeNode>,
        one: Option<TreeNode>,
    ) -> Vary {
        Vary {
            index,
            most_common_value,
            zero,
            one,
        }
    }

    fn make<T: RngCore + SeedableRng>(
        // parent_index: usize, this turned out to be never updated!
        index: usize,
        start: usize,
        num_record: usize,
        data: &mut Data<T>,
    ) -> Vary {
        //let parent_index = -1; // this was set AdTree::make
        if
        //(parent_index + 1 != index)
        0 != index && (num_record > 1) {
            data.sort(start, num_record, index);
        }

        let num0 = data.find_split(start, num_record, index);
        let num1 = num_record - num0;

        let most_common_value = if num0 >= num1 { Val::Zero } else { Val::One };

        let zero = match num0 == 0 || most_common_value == Val::Zero {
            true => None,
            false => Some(TreeNode::make(index, start, num0, 0, data)),
        };
        let one = match num1 == 0 || most_common_value == Val::One {
            true => None,
            false => Some(TreeNode::make(index, start + num0, num1, 1, data)),
        };

        Vary::new(index, most_common_value, zero, one)
    }
}

impl AdTreeT for AdTree {
    fn new(num_var: usize, num_record: usize, root: RootNode) -> AdTree {
        AdTree {
            num_var,
            num_record,
            root,
        }
    }

    fn make<T: RngCore + SeedableRng>(data: &mut Data<T>) -> AdTree {
        let num_record = data.num_record;
        let num_var = data.num_var;

        data.sort(0, num_record, 0);
        let root = RootNode::make(num_record, data);
        AdTree::new(num_var, num_record, root)
    }

    fn get_count<T: QueryT>(&self, queries: &mut Vec<T>) -> usize {
        //let num_query = queries.len();
        let last_query_index = match queries.last() {
            None => None, // -1 in original code
            Some(last_query) => Some(last_query.index()),
        };
        self.root.get_count(0, queries, last_query_index, &self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bayes::query::Query;

    fn count_data<T: RngCore + SeedableRng>(data: &mut Data<T>, queries: &Vec<Query>) -> usize {
        assert!(data.num_record == data.records.len());
        let mut count = 0;
        for record in data.records.iter() {
            let mut is_match = true;
            for query in queries {
                if query.val != Val::WildCard && query.val.clone() as usize != record[query.index] {
                    is_match = false;
                    break;
                } else {
                    // continue
                }
            }
            count = if is_match { count + 1 } else { count };
        }
        count
    }

    fn test_count<T: RngCore + SeedableRng>(
        ad_tree: &AdTree,
        mut data: &mut Data<T>,
        queries: &mut Vec<Query>,
        index: usize,
        num_var: usize,
    ) {
        if index >= num_var {
            // just nothing
        } else {
            let count1 = ad_tree.get_count(queries);
            let count2 = count_data(data, queries);
            assert!(count1 == count2);

            for i in 1..num_var {
                queries.push(Query::new(index + i, Val::Zero));
                test_count(&ad_tree, &mut data, queries, index + i, num_var);
                queries.remove(queries.len() - 1);

                queries.push(Query::new(index + i, Val::One));
                test_count(&ad_tree, &mut data, queries, index + i, num_var);
                queries.remove(queries.len() - 1);
            }
        }
    }

    fn test_counts<T: RngCore + SeedableRng>(ad_tree: AdTree, mut data: Data<T>) {
        let mut queries: Vec<Query> = Vec::with_capacity(data.num_var);
        //for (v = -1; v < numVar; v++) {
        let mut num_var = data.num_var;
        for v in 0..num_var {
            num_var = data.num_var;
            test_count(&ad_tree, &mut data, &mut queries, v, num_var);
        }
    }

    fn test(num_var: usize, num_record: usize) {
        let random = rand::rngs::StdRng::seed_from_u64(0);
        let mut data = Data::new(num_var, num_record, random);
        data.generate(Some(0), 10, 10);

        let mut copy_data = data.clone();

        let adtree = AdTree::make(&mut copy_data);

        test_counts(adtree, data);
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
