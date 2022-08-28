use rand::{Rng, RngCore};
use std::collections::VecDeque;

#[derive(Clone)]
enum NodeMark {
    Init,
    Done,
    Test,
}

struct Node {
    id: usize,
    // maybe these want to be HashSets
    parent_ids: Vec<usize>,
    child_ids: Vec<usize>,
    mark: NodeMark,
}

pub struct Net {
    nodes: Vec<Node>,
}

#[derive(Clone)]
pub enum Operation {
    Insert,
    Remove,
    Reverse,
}

pub(crate) trait NetT {
    fn new(num_node: usize) -> Self;
    fn apply_operation(&mut self, op: Operation, from_id: usize, to_id: usize);
    fn has_edge(&self, from_id: usize, to_id: usize) -> bool;
    fn is_path(
        &self,
        from_id: usize,
        to_id: usize,
        visited: &mut Vec<bool>,
        work_queue: &mut VecDeque<usize>,
    ) -> bool;
    fn is_cycle(&mut self) -> bool;
    fn get_parent_id_list(&self, id: usize) -> &Vec<usize>;
    fn get_child_id_list(&self, id: usize) -> &Vec<usize>;

    /* =============================================================================
     * net_findAncestors
     * -- Contents of bitmapPtr set to 1 if parent, else 0
     * -- Returns false if id is not root node (i.e., has cycle back id)
     * =============================================================================
     */
    fn find_ancestors(&self, id: usize, work_queue: &mut VecDeque<usize>) -> Option<Vec<bool>>;

    /* =============================================================================
     * net_findDescendants
     * -- Contents of bitmapPtr set to 1 if descendants, else 0
     * -- Returns false if id is not root node (i.e., has cycle back id)
     * =============================================================================
     */
    fn find_descendants(&self, id: usize, work_queue: &mut VecDeque<usize>) -> Option<Vec<bool>>;

    /* =============================================================================
     * net_generateRandomEdges
     * =============================================================================
     */
    fn generate_random_edges<T: RngCore>(
        &mut self,
        max_num_parent: usize,
        percent_parent: usize,
        random: &mut T,
    );
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id,
            parent_ids: Vec::new(),
            child_ids: Vec::new(),
            mark: NodeMark::Init, // uninitialized in original code
        }
    }
}

impl Net {
    fn insert_edge(&mut self, from_id: usize, to_id: usize) {
        let child_node = self.nodes.get_mut(to_id).expect("invariant broken");
        child_node.parent_ids.push(from_id);
        let parent_node = self.nodes.get_mut(from_id).expect("invariant broken");
        parent_node.child_ids.push(to_id);
    }

    fn remove_edge(&mut self, from_id: usize, to_id: usize) {
        let child_node = self.nodes.get_mut(to_id).expect("invariant broken");
        child_node.parent_ids.remove(from_id);
        let parent_node = self.nodes.get_mut(from_id).expect("invariant broken");
        parent_node.child_ids.remove(to_id);
    }

    fn reverse_edge(&mut self, from_id: usize, to_id: usize) {
        self.remove_edge(from_id, to_id);
        self.insert_edge(to_id, from_id);
    }

    fn is_cycle0(&mut self, id: usize) -> bool {
        let m = {
            let node = self.nodes.get_mut(id).expect("invariant broken");
            match node.mark {
                NodeMark::Init => {
                    node.mark = NodeMark::Test;
                }
                _ => (),
            }
            node.mark.clone()
        }; // release mutable borrow on `node`

        let result = match m {
            NodeMark::Init => {
                let l = {
                    let node = self.nodes.get(id).expect("invariant broken");
                    node.child_ids.len()
                };
                let mut result = false;
                for i in 0..l {
                    let node = self.nodes.get(id).expect("invariant broken");
                    let child_id = node.child_ids[i];
                    if self.is_cycle0(child_id) {
                        result = true;
                        break;
                    } else {
                        // continue
                    }
                }
                result
            }
            NodeMark::Test => true,
            NodeMark::Done => false,
        }; // release the immutable borrow on `node`

        match m {
            NodeMark::Init => {
                if !result {
                    let node = self.nodes.get_mut(id).expect("invariant broken");
                    node.mark = NodeMark::Done;
                } else {
                    // the original code only sets this when `false`
                } // release mutable borrow on `node`
            }
            _ => (),
        }

        result
    }
}

impl NetT for Net {
    fn new(num_node: usize) -> Net {
        let mut nodes = Vec::new();
        for i in 0..num_node {
            nodes.push(Node::new(i));
        }
        Net { nodes }
    }

    fn apply_operation(&mut self, op: Operation, from_id: usize, to_id: usize) {
        match op {
            Operation::Insert => self.insert_edge(from_id, to_id),
            Operation::Remove => self.remove_edge(from_id, to_id),
            Operation::Reverse => self.reverse_edge(from_id, to_id),
        }
    }

    fn has_edge(&self, from_id: usize, to_id: usize) -> bool {
        self.nodes
            .get(to_id)
            .expect("invariant broken")
            .parent_ids
            .contains(&from_id)
    }

    // I changed this function considerably because the STAMP version of it
    // was updating pointer structures although this claims to answer a simple
    // boolean question.
    fn is_path(
        &self,
        from_id: usize,
        to_id: usize,
        // FIXME are the below parameters only there for reuse/optimization?
        visited: &mut Vec<bool>,
        work_queue: &mut VecDeque<usize>,
    ) -> bool {
        assert!(visited.len() == self.nodes.len());

        work_queue.clear();
        visited.fill(false);

        work_queue.push_back(from_id);

        let mut result = false;
        while let Some(id) = work_queue.pop_front() {
            if id == to_id {
                work_queue.clear();
                result = true;
            } else {
                visited.insert(id, true);
                let node = self.nodes.get(id).expect("invariant broken");
                for child_id in &node.child_ids {
                    if visited.get(*child_id).is_none() {
                        work_queue.push_back(*child_id)
                    } else {
                        // already visited
                    }
                }
            }
        }

        result
    }

    // This is yet another function that seems to answer a totally harmless
    // boolean question. Yet it alters the state of the nodes.
    // It seems like the author keeps state that is directly tied to a particular
    // function, probably to avoid re-allocations.
    fn is_cycle(&mut self) -> bool {
        let _num_node = self.nodes.len();
        for mut node in self.nodes.iter_mut() {
            node.mark = NodeMark::Init; //NET_NODE_MARK_INIT;
        }

        let mut result = false;
        for n_id in 0..self.nodes.len() {
            match self.nodes[n_id].mark {
                NodeMark::Init => {
                    if self.is_cycle0(n_id) {
                        result = true;
                        break;
                    } else {
                        // continue the loop
                    }
                }
                NodeMark::Done => (),
                NodeMark::Test => assert!(false),
            }
        }

        result
    }

    fn get_parent_id_list(&self, id0: usize) -> &Vec<usize> {
        &self.nodes.get(id0).expect("invariant broken").parent_ids
    }

    fn get_child_id_list(&self, id0: usize) -> &Vec<usize> {
        &self.nodes.get(id0).expect("invariant broken").child_ids
    }

    /* =============================================================================
     * net_findAncestors
     * -- Contents of bitmapPtr set to 1 if ancestor, else 0
     * -- Returns false if id is not root node (i.e., has cycle back id)
     * =============================================================================
     */
    fn find_ancestors(
        &self,
        id0: usize,
        // FIXME are these parameters here only for reuse/optimization?
        //mut ancestor: &mut Vec<bool>,
        work_queue: &mut VecDeque<usize>,
    ) -> Option<Vec<bool>> {
        //assert!(ancestor.len() == self.nodes.len());

        // TODO if this reinitialization becomes a performance
        // problem then tie this state to the struct!
        let mut ancestor = Vec::with_capacity(self.nodes.len());
        ancestor.fill(false);
        work_queue.clear();

        for parent_id in &self.nodes.get(id0).expect("invariant broken").parent_ids {
            ancestor.insert(*parent_id, true);
            work_queue.push_back(*parent_id);
        }

        let mut result = true;
        while let Some(parent_id) = work_queue.pop_front() {
            if parent_id == id0 {
                work_queue.clear();
                result = false;
            } else {
                for grand_parent_id in &self
                    .nodes
                    .get(parent_id)
                    .expect("invariant broken")
                    .parent_ids
                {
                    if !ancestor.get(*grand_parent_id).expect("invariant broken") {
                        ancestor.insert(*grand_parent_id, true);
                        work_queue.push_back(*grand_parent_id);
                    }
                }
            }
        }

        if result {
            Some(ancestor)
        } else {
            None
            // in the original implementation, the caller nevertheless gets the altered ancestor
            // vector up to this point.
        }
    }

    /* =============================================================================
     * net_findDescendants
     * -- Contents of bitmapPtr set to 1 if descendants, else 0
     * -- Returns false if id is not root node (i.e., has cycle back id)
     * =============================================================================
     */
    fn find_descendants(
        &self,
        id0: usize,
        //mut descendant: &mut Vec<bool>,
        work_queue: &mut VecDeque<usize>,
    ) -> Option<Vec<bool>> {
        //assert!(descendant.len() == self.nodes.len());

        // see comment in find_ancestors
        let mut descendant = Vec::with_capacity(self.nodes.len());
        descendant.fill(false);
        work_queue.clear();

        for child_id in &self.nodes.get(id0).expect("invariant broken").child_ids {
            descendant.insert(*child_id, true);
            work_queue.push_back(*child_id);
        }

        let mut result = true;
        while let Some(child_id) = work_queue.pop_front() {
            if child_id == id0 {
                work_queue.clear();
                result = false;
            } else {
                for grand_child_id in &self
                    .nodes
                    .get(child_id)
                    .expect("invariant broken")
                    .child_ids
                {
                    if !descendant.get(*grand_child_id).expect("invariant broken") {
                        descendant.insert(*grand_child_id, true);
                        work_queue.push_back(*grand_child_id);
                    }
                }
            }
        }

        if result {
            Some(descendant)
        } else {
            None // see comment in find_ancestors
        }
    }

    fn generate_random_edges<T: RngCore>(
        &mut self,
        max_num_parent: usize,
        percent_parent: usize,
        random: &mut T,
    ) {
        let num_node = self.nodes.len();
        let mut visited = Vec::with_capacity(num_node);
        visited.fill(false);
        let mut work_queue = VecDeque::new();

        for n in 0..num_node {
            for p in 0..max_num_parent {
                let value = random.gen::<usize>() % 100;
                if value < percent_parent {
                    let parent = random.gen::<usize>() % num_node;
                    if (parent != n)
                        && !self.has_edge(parent, n)
                        && !self.is_path(n, parent, &mut visited, &mut work_queue)
                    {
                        self.insert_edge(parent, n);
                    }
                }
            }
        }

        assert!(!self.is_cycle());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test1() {
        let num_node = 100;
        {
            let mut net = Net::new(num_node);
            let mut visited = Vec::with_capacity(num_node);
            visited.fill(false);
            let mut work_queue = VecDeque::new();

            assert!(!net.is_cycle());

            let a_id = 31;
            let b_id = 14;
            let c_id = 5;
            let d_id = 92;

            net.apply_operation(Operation::Insert, a_id, b_id);
            assert!(net.is_path(a_id, b_id, &mut visited, &mut work_queue));
            assert!(!net.is_path(b_id, a_id, &mut visited, &mut work_queue));
            assert!(!net.is_path(a_id, c_id, &mut visited, &mut work_queue));
            assert!(!net.is_path(a_id, d_id, &mut visited, &mut work_queue));
            assert!(!net.is_cycle());

            net.apply_operation(Operation::Insert, b_id, c_id);
            net.apply_operation(Operation::Insert, a_id, c_id);
            net.apply_operation(Operation::Insert, d_id, a_id);
            assert!(!net.is_cycle());
            net.apply_operation(Operation::Insert, c_id, d_id);
            assert!(net.is_cycle());
            net.apply_operation(Operation::Reverse, c_id, d_id);
            assert!(!net.is_cycle());
            net.apply_operation(Operation::Reverse, d_id, c_id);
            assert!(net.is_cycle());
            assert!(net.is_path(a_id, d_id, &mut visited, &mut work_queue));
            net.apply_operation(Operation::Remove, c_id, d_id);
            assert!(!net.is_path(a_id, d_id, &mut visited, &mut work_queue));

            // let ancestor = Vec::with_capacity(num_node);
            // ancestor.fill(false);
            let ancestor = net.find_ancestors(c_id, /*&mut ancestor,*/ &mut work_queue);
            assert!(ancestor.is_some());
            assert!(ancestor.as_ref().unwrap().get(a_id).unwrap());
            assert!(ancestor.as_ref().unwrap().get(b_id).unwrap());
            assert!(ancestor.as_ref().unwrap().get(d_id).unwrap());
            assert!(ancestor.as_ref().unwrap().len() == 3);

            // let descendant = Vec::with_capacity(num_node);
            // descendant.fill(false);
            let descendant = net.find_descendants(a_id, /*&mut descendant,*/ &mut work_queue);
            assert!(descendant.is_some());
            assert!(descendant.as_ref().unwrap().get(b_id).unwrap());
            assert!(descendant.as_ref().unwrap().get(c_id).unwrap());
            assert!(descendant.as_ref().unwrap().len() == 2);
        }
        {
            let mut random = rand::rngs::StdRng::seed_from_u64(0);
            let mut net = Net::new(num_node);
            net.generate_random_edges(10, 10, &mut random);
        }
    }
}
