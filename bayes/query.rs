
enum Val { 
    Zero, 
    One, 
    WildCard 
}

struct Query {
    index: usize,
    val: Val
}

impl Query {
    fn new(index: usize, val: Val) -> Query {
        Query {index, val}
    }

    fn compare(&self, other: &Query) -> Ordering {
        match self.index - other.index {
            x if x>0 => Ordering.Greater,
            0 => Ordering.Equal,
            _ => Ordering.Less
        }
    }
}
