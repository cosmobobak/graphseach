use crate::graph::Graph;

pub trait GraphSearcher<G: Graph> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node>;
    fn search(graph: &G, root: G::Node) -> Option<G::Node>;
    fn nodes_visited(&self) -> usize;
    fn is_visited(&self, node: G::Node) -> bool;
}