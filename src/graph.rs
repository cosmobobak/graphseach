use std::collections::HashSet;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    id: usize
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Edge {
    from_id: usize,
    to_id: usize
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    goal_id: usize,
    visited: HashSet<usize>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            goal_id: 0,
            visited: HashSet::new()
        }
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn neighbors(&self, node: Node) -> Vec<Node> {
        self.nodes
            .iter()
            .filter(move |n| self.edges.iter().any(|e| e.from_id == node.id && e.to_id == n.id))
            .copied()
            .collect()
    }

    pub fn visit(&mut self, node: Node) {
        self.visited.insert(node.id);
    }

    pub fn visited(&self, node: Node) -> bool {
        self.visited.contains(&node.id)
    }

    pub fn is_goal(&self, node: Node) -> bool {
        node.id == self.goal_id
    }
}