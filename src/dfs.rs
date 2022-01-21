use crate::graph::{Graph, Node};



fn dfs(graph: &Graph, root: Node) -> Option<Node> {
    if graph.is_goal(root) {
        return Some(root);
    }
    for neighbor in graph.neighbors(root) {
        if let Some(node) = dfs(graph, neighbor) {
            return Some(node);
        }
    }
    None
}

fn depth_limited_dfs(graph: &Graph, root: Node, depth: usize) -> Option<Node> {
    if depth == 0 {
        return None;
    }
    if graph.is_goal(root) {
        return Some(root);
    }
    for neighbor in graph.neighbors(root) {
        if let Some(node) = depth_limited_dfs(graph, neighbor, depth - 1) {
            return Some(node);
        }
    }
    None
}

fn iterative_deepening_dfs(graph: &Graph, root: Node) -> Option<Node> {
    for depth in 0.. {
        if let Some(node) = depth_limited_dfs(graph, root, depth) {
            return Some(node);
        }
    }
    None // unreachable
}