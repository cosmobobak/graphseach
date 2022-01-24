
use crate::graph::Graph;

pub fn perft<G: Graph>(graph: &G, node: G::Node, depth: usize) -> usize {
    let children = graph.children(node);
    println!("{:?}", children);
    let no_children = children.len() == 0;
    if depth == 0 || no_children {
        return 1;
    }
    children.iter().map(|c| perft(graph, *c, depth - 1)).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::{graph::Graph, examplegraph::{get_example_graph, ExampleGraph}, perft::perft};

    #[test]
    fn depth1() {
        let graph = get_example_graph();
        let node_count = perft(&graph, ExampleGraph::root(), 1);
        assert_eq!(node_count, 2);
    }
}