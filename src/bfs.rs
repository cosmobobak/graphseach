use std::collections::VecDeque;

use crate::graph::Graph;

pub fn bfs<G: Graph>(graph: &mut G, root: G::Node) -> Option<G::Node> {
    let mut queue = VecDeque::new();
    graph.mark_visited(root);
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        if graph.is_goal(node) {
            return Some(node);
        }
        for neighbor in graph.children(node) {
            if !graph.is_visited(neighbor) {
                graph.mark_visited(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{graph::{self, Graph}, bfs};

    #[test]
    fn check_bfs() {
        let mut graph = graph::get_example_graph();
        let goal = graph.goal();
        assert_eq!(bfs::bfs(&mut graph, goal), Some(goal));
    }
}

