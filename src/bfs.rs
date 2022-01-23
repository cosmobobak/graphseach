use std::collections::{VecDeque, HashSet};

use crate::{graph::Graph, graphsearcher::GraphSearcher};

pub struct BFS<G: Graph> {
    visited: HashSet<G::Node>,
    max_frontier: usize
}

impl<G: Graph> BFS<G> {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            max_frontier: 1
        }
    }

    #[must_use] 
    pub fn max_frontier(&self) -> usize {
        self.max_frontier
    }

    fn mark_visited(&mut self, node: G::Node) {
        self.visited.insert(node);
    }
}

impl<G: Graph> Default for BFS<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: Graph> GraphSearcher<G> for BFS<G> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node> {
        let mut queue = VecDeque::new();
        self.mark_visited(root);
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if graph.is_goal(node) {
                return Some(node);
            }
            for neighbor in graph.children(node) {
                if !self.is_visited(neighbor) {
                    self.mark_visited(neighbor);
                    queue.push_back(neighbor);
                }
            }
            self.max_frontier = std::cmp::max(self.max_frontier, queue.len());
        }
        None
    }

    fn search(graph: &G, root: G::Node) -> Option<G::Node> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(root);
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if graph.is_goal(node) {
                return Some(node);
            }
            for neighbor in graph.children(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
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

#[cfg(test)]
mod tests {
    use crate::{graph::Graph, bfs::BFS, examplegraph::{self, ExampleGraph}};
    use crate::graphsearcher::GraphSearcher;

    #[test]
    fn check_bfs() {
        let graph = examplegraph::get_example_graph();
        let found = BFS::search(&graph, ExampleGraph::root());
        assert!(found.is_some());
        assert!(graph.is_goal(found.unwrap()));
    }
}

