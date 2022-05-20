
enum Val { 
    Zero, 
    One, 
    WildCard 
}

struct Query {
    // TODO
    index: usize,
    val: Val
}

impl Query {
    fn new(index: usize, val: Val) -> Query {
        Query {index, val}
    }

    fn compare(&self, other: &Query) -> i64 {
        self.index - other.index
    }
}
