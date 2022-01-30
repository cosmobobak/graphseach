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
}