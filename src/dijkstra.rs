use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

use crate::graph::WeightedGraph;
use crate::graphsearcher::GraphSearcher;
use crate::heapelement::HeapElement;

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

    pub const fn max_frontier(&self) -> usize {
        self.max_frontier
    }
}

impl<G: WeightedGraph> Default for Dijkstra<G> {
    fn default() -> Self {
        Self::new()
    }
}

// procedure uniform_cost_search(start) is
//     node ← start
//     frontier ← priority queue containing node only
//     explored ← empty set
//     do
//         if frontier is empty then
//             return failure
//         node ← frontier.pop()
//         if node is a goal state then
//             return solution(node)
//         explored.add(node)
//         for each of node's neighbors n do
//             if n is not in explored and not in frontier then
//                 frontier.add(n)
//             else if n is in frontier with higher cost
//                 replace existing node with n

impl<G: WeightedGraph> GraphSearcher<G> for Dijkstra<G> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node> {
        self.distances.clear();
        self.parents.clear();
        self.max_frontier = 1;

        let mut frontier = BinaryHeap::new();
        frontier.push(HeapElement::new(root, 0));

        self.distances.insert(root, 0);

        while let Some(HeapElement { node, cost }) = frontier.pop() {
            if graph.is_goal(node) {
                self.solution = Some(node);
                return Some(node);
            }
            for child in graph.children(node) {
                let cost_to_child = cost + graph.edge_weight(node, child);
                if cost_to_child < self.distances.get(&child).copied().unwrap_or(i64::MAX) {
                    self.parents.insert(child, node);
                    self.distances.insert(child, cost_to_child);
                    frontier.push(HeapElement::new(child, cost_to_child));
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

        while let Some(HeapElement { node, cost }) = frontier.pop() {
            if graph.is_goal(node) {
                return Some(node);
            }
            for child in graph.children(node) {
                let cost_to_child = cost + graph.edge_weight(node, child);
                if cost_to_child < distances.get(&child).copied().unwrap_or(i64::MAX) {
                    distances.insert(child, cost_to_child);
                    frontier.push(HeapElement::new(child, cost_to_child));
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
