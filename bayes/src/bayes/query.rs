use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Val { 
    Zero, 
    One, 
    WildCard 
}

#[derive(Clone, Debug)]
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

pub(crate) trait QueryT: Ord {
    fn index(&self) -> usize;
    fn val(&self) -> &Val;
    fn update_val(&mut self, new_val:Val);
    fn clon(&self) -> Query;
}

impl QueryT for Query {
    fn index(&self) -> usize { self.index }
    fn val(&self) -> &Val { &self.val }
    fn update_val(&mut self, new_val:Val) { self.val = new_val;  }
    fn clon(&self) -> Query { self.clone() }
}

impl QueryT for &mut Query {
    fn index(&self) -> usize { self.index }
    fn val(&self) -> &Val { &self.val }
    fn update_val(&mut self, new_val:Val) { self.val = new_val;  }
    fn clon(&self) -> Query { Query::new(self.index, self.val.clone()) }
}

/*
impl Deref for Query {
    type Target = Query;

    fn deref(&self) -> &Query {
        self
    }
}
*/
