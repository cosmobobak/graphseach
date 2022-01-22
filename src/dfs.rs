use std::collections::HashSet;

use crate::{graph::Graph, graphsearcher::GraphSearcher};

pub struct DFS<G: Graph> {
    visited: HashSet<G::Node>
}

impl<G: Graph> DFS<G> {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            visited: HashSet::new()
        }
    }

    fn mark_visited(&mut self, node: G::Node) {
        self.visited.insert(node);
    }
}

impl<G: Graph> Default for DFS<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: Graph> GraphSearcher<G> for DFS<G> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node> {
        self.mark_visited(root);
        if graph.is_goal(root) {
            return Some(root);
        }
        for neighbor in graph.children(root) {
            if let Some(node) = self.search_tracked(graph, neighbor) {
                return Some(node);
            }
        }
        None
    }

    fn search(graph: &G, root: G::Node) -> Option<G::Node> {
        if graph.is_goal(root) {
            return Some(root);
        }
        for neighbor in graph.children(root) {
            if let Some(node) = Self::search(graph, neighbor) {
                return Some(node);
            }
        }
        None
    }

    fn nodes_visited(&self) -> usize {
        self.visited.len()
    }

    fn is_visited(&self, node: G::Node) -> bool {
        self.visited.contains(&node)
    }
}

pub struct IterDeepening<G: Graph> {
    visited: HashSet<G::Node>,
    counter: usize,
}

impl<G: Graph> IterDeepening<G> {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            counter: 0,
        }
    }

    fn dl_search_tracked(&mut self, graph: &G, root: G::Node, depth: usize) -> Option<G::Node> {
        self.mark_visited(root);
        if depth == 0 {
            return None;
        }
        if graph.is_goal(root) {
            return Some(root);
        }
        for neighbor in graph.children(root) {
            if let Some(node) = self.dl_search_tracked(graph, neighbor, depth - 1) {
                return Some(node);
            }
        }
        None
    }

    fn dl_search(graph: &G, root: G::Node, depth: usize) -> Option<G::Node> {
        if depth == 0 {
            return None;
        }
        if graph.is_goal(root) {
            return Some(root);
        }
        for neighbor in graph.children(root) {
            if let Some(node) = Self::dl_search(graph, neighbor, depth - 1) {
                return Some(node);
            }
        }
        None
    }

    fn mark_visited(&mut self, node: G::Node) {
        self.visited.insert(node);
        self.counter += 1;
    }
}

impl<G: Graph> Default for IterDeepening<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: Graph> GraphSearcher<G> for IterDeepening<G> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node> {
        for depth in 0.. {
            if let Some(node) = self.dl_search_tracked(graph, root, depth) {
                return Some(node);
            }
        }
        None // unreachable
    }

    fn search(graph: &G, root: G::Node) -> Option<G::Node> {
        for depth in 0.. {
            if let Some(node) = Self::dl_search(graph, root, depth) {
                return Some(node);
            }
        }
        None // unreachable
    }

    fn nodes_visited(&self) -> usize {
        self.counter
    }

    fn is_visited(&self, node: G::Node) -> bool {
        self.visited.contains(&node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{graph::{self, Graph, ExampleGraph}, dfs::{DFS, IterDeepening}, graphsearcher::GraphSearcher};

    #[test]
    fn check_dfs() {
        let graph = graph::get_example_graph();
        let found = DFS::search(&graph, ExampleGraph::root());
        assert!(found.is_some());
        assert!(graph.is_goal(found.unwrap()));
    }

    #[test]
    fn check_iterative_deepening_dfs() {
        let graph = graph::get_example_graph();
        let found = IterDeepening::search(&graph, ExampleGraph::root());
        assert!(found.is_some());
        assert!(graph.is_goal(found.unwrap()));
    }
}