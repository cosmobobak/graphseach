use std::{fmt::Display, hash::Hash};

use crate::graph::{Graph, HeuristicGraph, WeightedGraph};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ExampleNode {
    id: usize,
}

impl ExampleNode {
    pub const fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Display for ExampleNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ExampleEdge {
    from_id: usize,
    to_id: usize,
}

impl ExampleEdge {
    #[cfg(test)]
    pub const fn new(from_id: usize, to_id: usize) -> Self {
        Self { from_id, to_id }
    }
}

impl Display for ExampleEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge({} -> {})", self.from_id, self.to_id)
    }
}

pub struct ExampleGraph {
    nodes: Vec<ExampleNode>,
    edges: Vec<ExampleEdge>,
    goal_id: usize,
}

impl ExampleGraph {
    #[cfg(test)]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            goal_id: 0,
        }
    }

    #[cfg(test)]
    fn add(&mut self, node: ExampleNode) {
        self.nodes.push(node);
    }

    #[cfg(test)]
    fn add_edge(&mut self, edge: ExampleEdge) {
        self.edges.push(edge);
    }

    #[cfg(test)]
    fn set_goal(&mut self, node: ExampleNode) {
        self.goal_id = node.id;
    }
}

impl Graph for ExampleGraph {
    type Node = ExampleNode;
    type Edge = ExampleEdge;

    fn root(&self) -> Self::Node {
        ExampleNode::new(8)
    }

    fn children(&self, node: Self::Node) -> Vec<Self::Node> {
        self.nodes
            .iter()
            .filter(move |n| {
                self.edges
                    .iter()
                    .any(|e| e.from_id == node.id && e.to_id == n.id)
            })
            .copied()
            .collect()
    }

    fn edges(&self, node: Self::Node) -> Vec<Self::Edge> {
        self.edges
            .iter()
            .filter(move |e| e.from_id == node.id)
            .copied()
            .collect()
    }

    fn is_goal(&self, node: Self::Node) -> bool {
        node.id == self.goal_id
    }
}

impl WeightedGraph for ExampleGraph {
    fn edge_weight(&self, _from: Self::Node, _to: Self::Node) -> i64 {
        1
    }
}

impl HeuristicGraph for ExampleGraph {
    fn heuristic(&self, _node: Self::Node) -> i64 {
        1
    }
}

#[cfg(test)]
pub fn get_example_graph() -> ExampleGraph {
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
