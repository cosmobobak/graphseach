use crate::graph::{Graph, WeightedGraph};

pub trait GraphSearcher<G: Graph> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node>;
    fn search(graph: &G, root: G::Node) -> Option<G::Node>;
    fn nodes_visited(&self) -> usize;
    fn is_visited(&self, node: G::Node) -> bool;
    fn path(&self) -> Option<Vec<G::Node>>;
}

pub trait WeightedGraphSearcher<G: WeightedGraph> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node>;
    fn search(graph: &G, root: G::Node) -> Option<G::Node>;
    fn nodes_visited(&self) -> usize;
    fn is_visited(&self, node: G::Node) -> bool;
    fn path(&self) -> Option<Vec<G::Node>>;
}