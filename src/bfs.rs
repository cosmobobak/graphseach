use std::collections::VecDeque;

use crate::graph::{Graph, Node};

pub fn bfs(graph: &mut Graph, root: Node) -> Option<Node> {
    let mut queue = VecDeque::new();
    graph.visit(root);
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        if graph.is_goal(node) {
            return Some(node);
        }
        for neighbor in graph.neighbors(node) {
            if !graph.visited(neighbor) {
                graph.visit(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{graph, bfs};

    #[test]
    fn check_bfs() {
        let mut graph = graph::get_example_graph();
        let goal = graph.goal();
        assert_eq!(bfs::bfs(&mut graph, goal), Some(goal));
    }
}

