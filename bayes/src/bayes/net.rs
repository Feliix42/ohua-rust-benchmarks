use rand::{RngCore,Rng};
use std::collections::VecDeque;

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
    mark: NodeMark
}

pub struct Net {
    nodes: Vec<Node>,
}

pub enum Operation {
    Insert,
    Remove,
    Reverse,
}

trait NetT {
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
        &self,
        max_num_parent: usize,
        percent_parent: usize,
        random: &T,
    );
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id,
            parent_ids: Vec::new(),
            child_ids: Vec::new(),
            mark: NodeMark::Init // uninitialized in original code 
        }
    }
}

impl Net {
    fn insert_edge(&mut self, from_id: usize, to_id: usize) {
        let mut child_node = self.nodes.get_mut(to_id).expect("invariant broken");
        child_node.parent_ids.push(from_id);
        let mut parent_node = self.nodes.get(from_id).expect("invariant broken");
        parent_node.child_ids.push(to_id);
    }

    fn remove_edge(&mut self, from_id: usize, to_id: usize) {
        let mut child_node = self.nodes.get_mut(to_id).expect("invariant broken");
        child_node.parent_ids.remove(from_id);
        let mut parent_node = self.nodes.get(from_id).expect("invariant broken");
        parent_node.child_ids.remove(to_id);
    }

    fn reverse_edge(&mut self, from_id: usize, to_id: usize) {
        self.remove_edge(from_id, to_id);
        self.insert_edge(to_id, from_id);
    }

    fn is_cycle0(&self, node: &Node) -> bool {
        match node.mark {
            NodeMark::Init => {
                node.mark = NodeMark::Test;
                let mut result = false;
                for child_id in node.child_ids {
                    let child_node = self.nodes.get(child_id).expect("invariant broken");
                    if self.is_cycle0(&child_node) {
                        result = true;
                        break;
                    } else {
                        // continue
                    }
                }
                if !result {
                    node.mark = NodeMark::Done;
                } else {
                    // the original code only sets this when `false`
                }
                result
            }
            NodeMark::Test => true,
            NodeMark::Done => false,
        }
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
        mut visited: &mut Vec<bool>,
        mut work_queue: &mut VecDeque<usize>,
    ) -> bool {
        assert!(visited.len() == self.nodes.len());

        work_queue.clear();
        visited.fill(false);

        work_queue.push_back(from_id);

        let result = false;
        while let Some(id) = work_queue.pop_front() {
            if id == to_id {
                work_queue.clear();
                result = true;
            } else {
                visited.insert(id, true);
                let node = self.nodes.get(id).expect("invariant broken");
                for child_id in node.child_ids {
                    if visited.get(child_id).is_none() {
                        work_queue.push_back(child_id)
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
        let num_node = self.nodes.len();
        for node in self.nodes {
            node.mark = NodeMark::Init; //NET_NODE_MARK_INIT;
        }

        let result = false;
        for node in self.nodes {
            match node.mark {
                NodeMark::Init => {
                    if self.is_cycle0(&node) {
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
        mut work_queue: &mut VecDeque<usize>,
    ) -> Option<Vec<bool>> {
        //assert!(ancestor.len() == self.nodes.len());

        // TODO if this reinitialization becomes a performance
        // problem then tie this state to the struct!
        let mut ancestor = Vec::with_capacity(self.nodes.len());
        ancestor.fill(false);
        work_queue.clear();

        for parent_id in self.nodes.get(id0).expect("invariant broken").parent_ids {
            ancestor.insert(parent_id, true);
            work_queue.push_back(parent_id);
        }

        let result = true;
        while let Some(parent_id) = work_queue.pop_front() {
            if parent_id == id0 {
                work_queue.clear();
                result = false;
            } else {
                for grand_parent_id in self
                    .nodes
                    .get(parent_id)
                    .expect("invariant broken")
                    .parent_ids
                {
                    if !ancestor.get(grand_parent_id).expect("invariant broken") {
                        ancestor.insert(grand_parent_id, true);
                        work_queue.push_back(grand_parent_id);
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
        mut work_queue: &mut VecDeque<usize>,
    ) -> Option<Vec<bool>>{
        //assert!(descendant.len() == self.nodes.len());

        // see comment in find_ancestors
        let descendant = Vec::with_capacity(self.nodes.len());
        descendant.fill(false);
        work_queue.clear();

        for child_id in self.nodes.get(id0).expect("invariant broken").child_ids {
            descendant.insert(child_id, true);
            work_queue.push_back(child_id);
        }

        let result = true;
        while let Some(child_id) = work_queue.pop_front() {
            if child_id == id0 {
                work_queue.clear();
                result = false;
            } else {
                for grand_child_id in self
                    .nodes
                    .get(child_id)
                    .expect("invariant broken")
                    .child_ids
                {
                    if !descendant.get(grand_child_id).expect("invariant broken") {
                        descendant.insert(grand_child_id, true);
                        work_queue.push_back(grand_child_id);
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

    fn generate_random_edges<T:RngCore>(&self, max_num_parent: usize, percent_parent: usize, random: &T) {
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

    #[test]
    fn test1() {
        let num_node = 100;
        {
            let mut net = Net::new(num_node);
            let mut visited = Vec::with_capacity(num_node);
            visited.fill(false);
            let work_queue = Vec::new();

            assert!(!net.is_cycle());

            let aId = 31;
            let bId = 14;
            let cId = 5;
            let dId = 92;

            net.apply_operation(Operation::Insert, aId, bId);
            assert!(net.is_path(aId, bId, &mut visited, &mut work_queue));
            assert!(!net.is_path(bId, aId, visited, work_queue));
            assert!(!net.is_path(aId, cId, visited, work_queue));
            assert!(!net.is_path(aId, dId, visited, work_queue));
            assert!(!net.is_cycle());

            net.apply_operation(Operation::Insert, bId, cId);
            net.apply_operation(Operation::Insert, aId, cId);
            net.apply_operation(Operation::Insert, dId, aId);
            assert!(!net.is_cycle());
            net.apply_operation(Operation::Insert, cId, dId);
            assert!(net.is_cycle());
            net.apply_operation(Operation::Reverse, cId, dId);
            assert!(!net.is_cycle());
            net.apply_operation(Operation::Reverse, dId, cId);
            assert!(net.is_cycle());
            assert!(net.is_path(aId, dId, &mut visited, &mut work_queue));
            net.apply_operation(Operation::Removew, cId, dId);
            assert!(!net.is_path(aId, dId, &mut visited, &mut work_queue));

            let ancestor = Vec::with_capacity(num_node);
            ancestor.fill(false);
            let status = net.find_ancestors(cId, &mut ancestor, &mut work_queue);
            assert!(status);
            assert!(ancestor.get(aId).unwrap());
            assert!(ancestor.get(bId).unwrap());
            assert!(ancestor.get(dId).unwrap());
            assert!(ancestor.filter(|x| x).len() == 3);

            let descendant = Vec::with_capacity(num_node);
            descendant.fill(false);
            let status = net.find_descendants(aId, &mut descendant, &mut work_queue);
            assert!(status);
            assert!(descendant.get(bId).unwrap());
            assert!(descendant.get(cId).unwrap());
            assert!(descendant.filter(|x| x).len() == 2);
        }
        {
            let random = Random::new();
            let net = Net::new(num_node);
            net.generate_random_edges(10, 10, &random);
        }
    }
}
