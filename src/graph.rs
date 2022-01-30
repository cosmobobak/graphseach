#![warn(clippy::module_name_repetitions)]

use std::{hash::Hash, fmt::Display, fmt::Debug};

pub trait Graph {
    type Node: Copy + Eq + Hash + Display + Debug;
    type Edge: Copy + Eq + Hash + Display + Debug;

    fn root() -> Self::Node;
    fn children(&self, node: Self::Node) -> Vec<Self::Node>;
    fn edges(&self, node: Self::Node) -> Vec<Self::Edge>;
    fn is_goal(&self, node: Self::Node) -> bool;
}

#[allow(clippy::module_name_repetitions)]
pub trait WeightedGraph: Graph {
    fn edge_weight(&self, from: Self::Node, to: Self::Node) -> i64;

    fn path_cost(&self, path: &[Self::Node]) -> i64 {
        path.iter().zip(path.iter().skip(1)).map(|(a, b)| self.edge_weight(*a, *b)).sum()
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait HeuristicGraph: Graph {
    fn heuristic(&self, node: Self::Node) -> i64;
}