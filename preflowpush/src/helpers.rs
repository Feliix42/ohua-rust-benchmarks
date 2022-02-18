use crate::functions::{Graph, NodeID};
use std::collections::HashSet;

pub fn combine(h1: HashSet<NodeID>, h2: HashSet<NodeID>) -> (bool, HashSet<NodeID>) {
    let res: HashSet<NodeID> = if h1.len() > h2.len() {
        h1.into_iter().chain(h2.into_iter()).collect()
    } else {
        h2.into_iter().chain(h1.into_iter()).collect()
    };

    (!res.is_empty(), res)
}

pub fn splitup(inp: Vec<(u64, (NodeID, Graph))>) -> (u64, Vec<(NodeID, Graph)>) {
    let mut lst = Vec::with_capacity(inp.len());
    let mut counter = 0;

    for (c, s) in inp.into_iter() {
        counter += c;
        lst.push(s);
    }

    (counter, lst)
}
