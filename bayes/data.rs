
#[derive(Debug, Copy, Clone)]
struct Data {
    num_var: usize,
    num_record: usize,
    records: Vec<String>, /* concatenation of all records */
    random: Random
};

trait DataT {

    fn new(num_var: usize, num_case: usize, random: Random) -> DataT;

    // free()
    
    fn generate(&self, seed: Long, max_num_parent: usize, percent_parent: usize) -> NetT;

    fn get_record(&self, index: usize) -> Option<&String>;

    fn sort(&mut self, start: usize, num: usize, offset: usize);

    fn find_split(&self, start: usize, num: usize, offset: usize) -> usize;

}
