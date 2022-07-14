use rand::RngCore;

#[derive(Debug, Copy, Clone)]
pub struct Data<T: RngCore> {
    num_var: usize,
    num_record: usize,
    records: Vec<String>, /* concatenation of all records */
    random: T,
}

trait DataT {
    fn new<T: RngCore>(num_var: usize, num_case: usize, random: T) -> Self;

    fn generate(&self, seed: usize, max_num_parent: usize, percent_parent: usize) -> Self;

    fn get_record(&self, index: usize) -> Option<&String>;

    fn sort(&mut self, start: usize, num: usize, offset: usize);

    fn find_split(&self, start: usize, num: usize, offset: usize) -> usize;
}
