use std::cmp::Reverse;
use std::collections::{HashSet, HashMap, BinaryHeap};

use crate::graph::HeuristicGraph;
use crate::{graph::WeightedGraph, graphsearcher::ComplexGraphSearcher};

use crate::heapelement::HeapElement;

pub struct BestFirstSearch<G: WeightedGraph> {
    visited: HashSet<G::Node>,
    parents: HashMap<G::Node, G::Node>,
    max_frontier: usize,
    solution: Option<G::Node>
}

impl<G: WeightedGraph> BestFirstSearch<G> {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            parents: HashMap::new(),
            max_frontier: 1,
            solution: None
        }
    }

    pub fn max_frontier(&self) -> usize {
        self.max_frontier
    }
}

impl<G: WeightedGraph> Default for BestFirstSearch<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: WeightedGraph + HeuristicGraph> ComplexGraphSearcher<G> for BestFirstSearch<G> {
    fn search_tracked(&mut self, graph: &G, root: G::Node) -> Option<G::Node> {
        self.visited.clear();
        self.parents.clear();
        self.max_frontier = 1;

        self.visited.insert(root);
        let mut frontier = BinaryHeap::new();
        frontier.push(Reverse(HeapElement::new(root, graph.heuristic(root))));

        while let Some(best_next_node) = frontier.pop() {
            let best_next_node = *best_next_node.0.node();
            for child in graph.children(best_next_node) {
                if !self.is_visited(child) {
                    if graph.is_goal(child) {
                        self.solution = Some(child);
                        return Some(child);
                    } 
                    
                    self.visited.insert(child);
                    self.parents.insert(child, best_next_node);
                    frontier.push(Reverse(HeapElement::new(
                        child, 
                        graph.heuristic(child))));
                }
            }
            self.max_frontier = std::cmp::max(self.max_frontier, frontier.len());
        }
        None
    }

    fn search(graph: &G, root: G::Node) -> Option<G::Node> {
        let mut visited = HashSet::new();
        let mut frontier = BinaryHeap::new();

        visited.insert(root);
        frontier.push(HeapElement::new(root, 0));

        while let Some(best_next_node) = frontier.pop() {
            let best_next_node = *best_next_node.node();
            for child in graph.children(best_next_node) {
                if !visited.contains(&child) {
                    if graph.is_goal(child) {
                        return Some(child);
                    }
                    
                    visited.insert(child);
                    frontier.push(HeapElement::new(
                        child, 
                        graph.heuristic(child)));
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
    use crate::examplegraph::{get_example_graph, ExampleGraph};
    use crate::graph::Graph;
    use crate::graphsearcher::ComplexGraphSearcher;

    #[test]
    fn test_best_first_search() {
        let graph = get_example_graph();
        let mut searcher = BestFirstSearch::new();
        let solution = searcher.search_tracked(&graph, ExampleGraph::root());
        assert!(graph.is_goal(solution.unwrap()));
    }
}