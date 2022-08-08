use rand::RngCore;
use crate::bayes::net::Net;

#[derive(Copy, Debug, Clone)]
enum DataConfig {
    Precision,
    Init
}

impl DataConfig {
    fn val(self) -> usize {
        match self {
            DataConfig::Precision => 100,
            DataConfig::Init => 2
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Data<T: RngCore> {
    num_var: usize,
    num_record: usize,
    records: Vec<Vec<usize>>, /* concatenation of all records */
    random: T,
}

pub(crate) trait DataT {
    fn new<T: RngCore>(num_var: usize, num_record: usize, random: T) -> Self;

    fn generate(&self, seed: usize, max_num_parent: usize, percent_parent: usize) -> Net;

    fn get_record(&self, index: usize) -> Option<&Vec<usize>>;

//    fn sort(&mut self, start: usize, num: usize, offset: usize);

//    fn find_split(&self, start: usize, num: usize, offset: usize) -> usize;
}

impl <T:RngCore> DataT for Data<T> {
    
    fn new(num_var: usize, num_record: usize, random: T) -> Self {
        let mut records = Vec::new(num_record);
        for _ in 0..num_record {
            let mut vars = Vec::new(num_var);
            for _ in 0..num_var {
                vars.push(0);
            }
            records.push(vars);
        }

        Data {
            num_var, 
            num_record, 
            records,
            random
        }
    }

    /* =============================================================================
     * data_generate
     * -- Binary variables of random PDFs
     * -- If seed is <0, do not reseed
     * -- Returns random network
     * =============================================================================
     */
    fn generate(&mut self, seed: Option<usize>, max_num_parent: usize, percent_parent: usize) -> Net {
        match seed {
            Some(s) => self.random.set_seed(s),
            None => ()
        }

        /*
         * Generate random Bayesian network
         */

        let mut net = Net::new(&self.num_var);
        net.generate_random_edges(max_num_parent, percent_parent, &self.random);

        /*
         * Create a threshold for each of the possible permutation of variable
         * value instances
         */

        let mut thresholds_table = Vec::new();
        for v in 0..self.num_var {
            let parent_id_list = net.get_parent_id_list(v);
            let num_threshold = 1 << parent_id_list.len();
            let mut thresholds = Vec::new(num_threshold);
            for t in 0..num_threshold {
                let threshold = self.random.generate % (DataConfig::Precision.val() + 1);
                thresholds.push(threshold);
            }
            thresholds_table.push(thresholds);
        }

        /*
         * Create variable dependency ordering for record generation
         */

        let order = Vec::new(self.num_var);
        let num_order = 0;

        let mut work_queue = Vec::new();

        let mut dependency_vector = Vec::new(1);

        let mut ordered_bitmap = Vec::new(self.num_var);
        for _ in 0..self.num_var { ordered_bitmap.push(false) }

        let mut done_bitmap = Vec::new(self.num_var);
        for _ in 0..self.num_var { done_bitmap.push(false) }
    
        while let Some(v) = done_bitmap.iter().position(|&&x| !x) {
            let child_id_list = net.get_child_id_list(v);
            if child_id_list.len() == 0 {
    
                /*
                 * Use breadth-first search to find net connected to this leaf
                 */
    
                work_queue.clear();
                work_queue.push(v);
                while let id = work_queue.take_first() {
                    done_bitmap[id] = true;
                    dependency_vector.push(id);
                    let parent_id_list = net.get_parent_id_list(id);
                    for parent_id in parent_id_list {
                        work_queue.push(parent_id);
                    }
                }
    
                /*
                 * Create ordering
                 */
    
                for id in dependency_vector.drain().rev() {
                    if !ordered_bitmap[id] {
                        ordered_bitmap[id] = true;
                        order[num_order] = id;
                        num_order = num_order + 1;
                    }
                }
    
            }
        }
        assert!(num_order == self.num_var);

        /*
         * Create records
         */
    
        for r in 0..self.num_record {
            for record in self.records {
                for o in 0..num_order {
                    let v = order[o];
                    let parent_id_list = net.get_parent_id_list(v);
                    let mut index = 0;
                    for parent_id in parent_id_list {
                        let value = self.records[parent_id];
                        assert!(value != DataConfig::Init);
                        index = (index << 1) + value.val();
                    }
                    let rnd = self.random.generate() % DataConfig::Precision.val();
                    let threshold = thresholds_table[v][index];
                    record[v] = if rnd < threshold { 1 } else { 0 };
                }
            }
        }

        net
    }


    /* =============================================================================
     * data_getRecord
     * -- Returns NULL if invalid index
     * =============================================================================
     */
    fn get_record (&self, index:usize) -> Option<&Vec<usize>> {
        self.get(index)
    }

}

// TODO port the test cases
