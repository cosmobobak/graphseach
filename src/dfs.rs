use crate::graph::Graph;

pub fn dfs<G: Graph>(graph: &mut G, root: G::Node) -> Option<G::Node> {
    graph.mark_visited(root);
    if graph.is_goal(root) {
        return Some(root);
    }
    for neighbor in graph.children(root) {
        if let Some(node) = dfs(graph, neighbor) {
            return Some(node);
        }
    }
    None
}

fn depth_limited_dfs<G: Graph>(graph: &mut G, root: G::Node, depth: usize) -> Option<G::Node> {
    graph.mark_visited(root);
    if depth == 0 {
        return None;
    }
    if graph.is_goal(root) {
        return Some(root);
    }
    for neighbor in graph.children(root) {
        if let Some(node) = depth_limited_dfs(graph, neighbor, depth - 1) {
            return Some(node);
        }
    }
    None
}

pub fn iterative_deepening_dfs<G: Graph>(graph: &mut G, root: G::Node) -> Option<G::Node> {
    for depth in 0.. {
        if let Some(node) = depth_limited_dfs(graph, root, depth) {
            return Some(node);
        }
    }
    None // unreachable
}

#[cfg(test)]
mod tests {
    use crate::{graph::{self, Graph}, dfs};

    #[test]
    fn check_dfs() {
        let mut graph = graph::get_example_graph();
        let goal = graph.goal();
        assert_eq!(dfs::dfs(&mut graph, goal), Some(goal));
    }

    #[test]
    fn check_iterative_deepening_dfs() {
        let mut graph = graph::get_example_graph();
        let goal = graph.goal();
        assert_eq!(dfs::iterative_deepening_dfs(&mut graph, goal), Some(goal));
    }
}