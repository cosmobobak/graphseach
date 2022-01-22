use std::collections::HashSet;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ExampleNode {
    id: usize
}

impl ExampleNode {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ExampleEdge {
    from_id: usize,
    to_id: usize
}

impl ExampleEdge {
    pub fn new(from_id: usize, to_id: usize) -> Self {
        Self { from_id, to_id }
    }
}

pub(crate) struct ExampleGraph {
    nodes: Vec<ExampleNode>,
    edges: Vec<ExampleEdge>,
    goal_id: usize,
    visited: HashSet<usize>
}

impl ExampleGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            goal_id: 0,
            visited: HashSet::new()
        }
    }
}

pub trait Graph {
    type Node: Copy;
    type Edge: Copy;

    fn add(&mut self, node: Self::Node);
    fn add_edge(&mut self, edge: Self::Edge);
    fn children(&self, node: Self::Node) -> Vec<Self::Node>;
    fn is_goal(&self, node: Self::Node) -> bool;
    fn is_visited(&self, node: Self::Node) -> bool;
    fn mark_visited(&mut self, node: Self::Node);
    fn unmark_visited(&mut self, node: Self::Node);
    fn count_visited(&self) -> usize;
    fn set_goal(&mut self, node: Self::Node);
    fn goal(&self) -> Self::Node;
}

impl Graph for ExampleGraph {
    type Node = ExampleNode;
    type Edge = ExampleEdge;

    fn add(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }

    fn children(&self, node: Self::Node) -> Vec<Self::Node> {
        self.nodes
            .iter()
            .filter(move |n| self.edges.iter().any(|e| e.from_id == node.id && e.to_id == n.id))
            .copied()
            .collect()
    }

    fn mark_visited(&mut self, node: Self::Node) {
        self.visited.insert(node.id);
    }

    fn is_visited(&self, node: Self::Node) -> bool {
        self.visited.contains(&node.id)
    }

    fn is_goal(&self, node: Self::Node) -> bool {
        node.id == self.goal_id
    }

    fn set_goal(&mut self, node: Self::Node) {
        self.goal_id = node.id;
    }

    fn goal(&self) -> Self::Node {
        Self::Node::new(self.goal_id)
    }

    fn unmark_visited(&mut self, node: Self::Node) {
        self.visited.remove(&node.id);
    }

    fn count_visited(&self) -> usize {
        self.visited.len()
    }
}

pub(crate) fn get_example_graph() -> ExampleGraph {
    // https://upload.wikimedia.org/wikipedia/commons/thumb/d/da/Binary_search_tree.svg/1200px-Binary_search_tree.svg.png
    let mut graph = ExampleGraph::new();
    graph.add(ExampleNode::new(8));
    graph.add(ExampleNode::new(3));
    graph.add(ExampleNode::new(10));
    graph.add(ExampleNode::new(1));
    graph.add(ExampleNode::new(6));
    graph.add(ExampleNode::new(14));
    graph.add(ExampleNode::new(4));
    graph.add(ExampleNode::new(7));
    graph.add(ExampleNode::new(13));

    graph.add_edge(ExampleEdge::new(8, 3));
    graph.add_edge(ExampleEdge::new(8, 10));
    graph.add_edge(ExampleEdge::new(3, 1));
    graph.add_edge(ExampleEdge::new(3, 6));
    graph.add_edge(ExampleEdge::new(10, 14));
    graph.add_edge(ExampleEdge::new(6, 4));
    graph.add_edge(ExampleEdge::new(6, 7));
    graph.add_edge(ExampleEdge::new(14, 13));

    graph.set_goal(ExampleNode::new(7));

    graph
}