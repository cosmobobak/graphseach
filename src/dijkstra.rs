use std::collections::{BinaryHeap, HashMap, hash_map::Entry::Vacant};

use crate::graphsearcher::GraphSearcher;
use crate::heapelement::HeapElement;
use crate::graph::WeightedGraph;
use std::fmt::Debug;

pub struct Dijkstra<G: WeightedGraph> {
    distances: HashMap<G::Node, i64>,
    parents: HashMap<G::Node, G::Node>,
    max_frontier: usize,
    solution: Option<G::Node>,
}

impl<G: WeightedGraph> Debug for Dijkstra<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dijkstra")
    }
}

impl<G: WeightedGraph> Dijkstra<G> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            distances: HashMap::new(),
            parents: HashMap::new(),
            max_frontier: 1,
            solution: None,
        }
    }

    pub fn max_frontier(&self) -> usize {
        self.max_frontier
    }
}

impl<G: WeightedGraph> Default for Dijkstra<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: WeightedGraph> GraphSearcher<G> for Dijkstra<G> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node> {
        self.distances.clear();
        self.parents.clear();
        self.max_frontier = 1;
        let mut frontier = BinaryHeap::new();

        self.distances.insert(root, 0);
        frontier.push(HeapElement::new(root, 0));

        while let Some(best_next_node) = frontier.pop() {
            let best_next_node = *best_next_node.node();
            let d_to_next = self.distances[&best_next_node];
            for child in graph.children(best_next_node) {
                if let Vacant(e) = self.distances.entry(child) {
                    self.parents.insert(child, best_next_node);

                    if graph.is_goal(child) {
                        self.solution = Some(child);
                        return Some(child);
                    }

                    e.insert(d_to_next + graph.edge_weight(best_next_node, child));
                    frontier.push(HeapElement::new(
                        child,
                        self.distances[&child],
                    ));
                }
            }
            self.max_frontier = std::cmp::max(self.max_frontier, frontier.len());
        }
        None
    }

    fn search(graph: &G, root: G::Node) -> Option<G::Node> {
        let mut distances = HashMap::new();
        let mut frontier = BinaryHeap::new();

        distances.insert(root, 0);
        frontier.push(HeapElement::new(root, 0));

        while let Some(best_next_node) = frontier.pop() {
            let best_next_node = *best_next_node.node();
            let d_to_next = distances[&best_next_node];
            for child in graph.children(best_next_node) {
                if let Vacant(e) = distances.entry(child) {
                    if graph.is_goal(child) {
                        return Some(child);
                    }

                    e.insert(d_to_next + graph.edge_weight(best_next_node, child));
                    frontier.push(HeapElement::new(
                        child,
                        distances[&child],
                    ));
                }
            }
        }
        None
    }

    fn nodes_visited(&self) -> usize {
        self.distances.len()
    }

    fn is_visited(&self, node: G::Node) -> bool {
        self.distances.contains_key(&node)
    }

    fn path(&self) -> Option<Vec<G::Node>> {
        let solution = self.solution?;
        let mut path = Vec::new();
        let mut n = solution;
        path.push(n);
        while let Some(&parent) = self.parents.get(&n) {
            path.push(parent);
            n = parent;
        }
        path.reverse();
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examplegraph::{get_example_graph, ExampleNode};
    use crate::graph::Graph;
    use crate::graphsearcher::GraphSearcher;

    #[test]
    fn basic() {
        let graph = get_example_graph();
        let mut searcher = Dijkstra::new();
        let found = searcher.search_tracked(&graph, graph.root());
        assert!(found.is_some());
        assert!(graph.is_goal(found.unwrap()));
        assert_eq!(
            searcher.path().unwrap(),
            &[
                ExampleNode::new(8),
                ExampleNode::new(3),
                ExampleNode::new(6),
                ExampleNode::new(7)
            ]
        );
    }
}
