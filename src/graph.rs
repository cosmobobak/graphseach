use std::collections::HashSet;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    id: usize
}

impl Node {
    pub fn new(id: usize) -> Self {
        Node { id }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Edge {
    from_id: usize,
    to_id: usize
}

impl Edge {
    pub fn new(from_id: usize, to_id: usize) -> Self {
        Edge { from_id, to_id }
    }
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

    pub fn set_goal(&mut self, node: Node) {
        self.goal_id = node.id;
    }

    pub fn goal(&self) -> Node {
        Node::new(self.goal_id)
    }
}

pub fn get_example_graph() -> Graph {
    let mut graph = Graph::new();
    graph.add(Node::new(8));
    graph.add(Node::new(3));
    graph.add(Node::new(10));
    graph.add(Node::new(1));
    graph.add(Node::new(6));
    graph.add(Node::new(14));
    graph.add(Node::new(4));
    graph.add(Node::new(7));
    graph.add(Node::new(13));

    graph.add_edge(Edge::new(8, 3));
    graph.add_edge(Edge::new(8, 10));
    graph.add_edge(Edge::new(3, 1));
    graph.add_edge(Edge::new(3, 6));
    graph.add_edge(Edge::new(10, 14));
    graph.add_edge(Edge::new(6, 4));
    graph.add_edge(Edge::new(6, 7));
    graph.add_edge(Edge::new(14, 13));

    graph.set_goal(Node::new(7));

    graph
}