use crate::bayes::net::{Net, NetT};
use crate::bayes::query::Val;
use rand::{Rng, RngCore, SeedableRng};
use std::collections::VecDeque;

#[derive(Copy, Debug, Clone)]
enum DataConfig {
    Precision,
    Init,
}

impl DataConfig {
    fn val(self) -> usize {
        match self {
            DataConfig::Precision => 100,
            DataConfig::Init => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Data<T: RngCore + SeedableRng> {
    pub(crate) num_var: usize,
    pub(crate) num_record: usize,
    pub(crate) records: Vec<Vec<Val>>, /* concatenation of all records */
    pub(crate) random: T,
}

pub(crate) trait DataT<T: RngCore + SeedableRng> {
    fn new(num_var: usize, num_record: usize, random: T) -> Self;

    fn generate(&mut self, seed: Option<u64>, max_num_parent: usize, percent_parent: usize) -> Net;

    fn get_record(&self, index: usize) -> Option<&Vec<Val>>;

    fn sort(&mut self, start: usize, num: usize, offset: usize);

    fn find_split(&self, start: usize, num: usize, offset: usize) -> usize;
}

impl<T: RngCore + SeedableRng> DataT<T> for Data<T> {
    fn new(num_var: usize, num_record: usize, random: T) -> Self {
        let records = vec![vec![Val::Zero; num_var]; num_record];

        Data {
            num_var,
            num_record,
            records,
            random,
        }
    }

    /* =============================================================================
     * data_generate
     * -- Binary variables of random PDFs
     * -- If seed is <0, do not reseed
     * -- Returns random network
     * =============================================================================
     */
    fn generate(&mut self, seed: Option<u64>, max_num_parent: usize, percent_parent: usize) -> Net {
        match seed {
            Some(s) => {
                self.random = <T as SeedableRng>::seed_from_u64(s);
            }
            None => (),
        }

        /*
         * Generate random Bayesian network
         */

        let mut net = Net::new(self.num_var);
        net.generate_random_edges(max_num_parent, percent_parent, &mut self.random);

        /*
         * Create a threshold for each of the possible permutation of variable
         * value instances
         */

        let mut thresholds_table = Vec::new();
        for v in 0..self.num_var {
            let parent_id_list = net.get_parent_id_list(v);
            let num_threshold = 1 << parent_id_list.len();
            let mut thresholds = Vec::with_capacity(num_threshold);
            for _t in 0..num_threshold {
                let threshold = self.random.gen::<usize>() % (DataConfig::Precision.val() + 1);
                thresholds.push(threshold);
            }
            thresholds_table.push(thresholds);
        }

        /*
         * Create variable dependency ordering for record generation
         */

        let mut order = Vec::with_capacity(self.num_var);

        let mut work_queue = VecDeque::new();

        let mut dependency_vector = Vec::with_capacity(1);

        let mut ordered_bitmap = Vec::with_capacity(self.num_var);
        for _ in 0..self.num_var {
            ordered_bitmap.push(false)
        }

        let mut done_bitmap = Vec::with_capacity(self.num_var);
        for _ in 0..self.num_var {
            done_bitmap.push(false)
        }

        while let Some(v) = done_bitmap.iter().position(|x| !x) {
            let child_id_list = net.get_child_id_list(v);
            if child_id_list.len() == 0 {
                /*
                 * Use breadth-first search to find net connected to this leaf
                 */

                work_queue.clear();
                work_queue.push_back(v);
                while let Some(id) = work_queue.pop_front() {
                    done_bitmap[id] = true;
                    dependency_vector.push(id);
                    let parent_id_list = net.get_parent_id_list(id);
                    for parent_id in parent_id_list {
                        work_queue.push_back(*parent_id);
                    }
                }

                /*
                 * Create ordering
                 */

                for id in dependency_vector.drain(..).rev() {
                    if !ordered_bitmap[id] {
                        ordered_bitmap[id] = true;
                        order.push(id);
                    }
                }
            }
        }
        assert!(order.len() == self.num_var);

        /*
         * Create records
         */

        assert!(self.num_record == self.records.len());
        for record in self.records.iter_mut() {
            for o in 0..order.len() {
                let v = order[o];
                let parent_id_list = net.get_parent_id_list(v);
                let mut index = 0;
                for _parent_id in parent_id_list {
                    //let value = self.records[*parent_id];
                    // assert!(value != DataConfig::Init);
                    index = index << 1; // + value.val();
                                        // I (Sebastian) am not positive whether the
                                        // above shifting code still works as expected.
                }
                let rnd = self.random.gen::<usize>() % DataConfig::Precision.val();
                let threshold = thresholds_table[v][index];
                record[v] = if rnd < threshold { Val::One } else { Val::Zero };
            }
        }

        net
    }

    fn get_record(&self, index: usize) -> Option<&Vec<Val>> {
        self.records.get(index)
    }

    fn sort(&mut self, start: usize, num: usize, _offset: usize) {
        assert!(start <= self.num_record);
        assert!(num <= self.num_record);
        assert!(start + num <= self.num_record);
        // just always sort all! this may have performance implications.
        self.records.sort_unstable();
    }

    fn find_split(&self, start: usize, num: usize, offset: usize) -> usize {
        assert!(offset < self.num_var);

        let mut low = start;
        let mut high = start + num - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if self.records[mid][offset] == Val::Zero {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        low - start
    }
}

// TODO port the test cases
