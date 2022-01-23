#![warn(clippy::module_name_repetitions)]

use std::{hash::Hash, fmt::Display};

pub trait Graph {
    type Node: Copy + Eq + Hash + Display;
    type Edge: Copy + Eq;

    fn root() -> Self::Node;
    fn children(&self, node: Self::Node) -> Vec<Self::Node>;
    fn is_goal(&self, node: Self::Node) -> bool;
}