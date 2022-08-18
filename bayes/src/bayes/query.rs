use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub(crate) enum Val { 
    Zero, 
    One, 
    WildCard 
}

#[derive(Clone)]
pub(crate) struct Query {
    pub(crate) index: usize,
    pub(crate) val: Val
}

impl Query {
    pub(crate) fn new(index: usize, val: Val) -> Query {
        Query {index, val}
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Query {}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.index - other.index {
            x if x>0 => Ordering::Greater,
            0 => Ordering::Equal,
            _ => Ordering::Less
        }
    }
}
