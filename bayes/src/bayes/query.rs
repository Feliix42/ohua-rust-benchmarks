use std::cmp::Ordering;

pub(crate) enum Val { 
    Zero, 
    One, 
    WildCard 
}

pub(crate) struct Query {
    index: usize,
    val: Val
}

impl Query {
    fn new(index: usize, val: Val) -> Query {
        Query {index, val}
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Query) -> Ordering {
        match self.index - other.index {
            x if x>0 => Ordering::Greater,
            0 => Ordering::Equal,
            _ => Ordering::Less
        }
    }
}
