mod bfs;
mod graph;
mod dfs;

use graph::Node;

fn main() {
    let mut graph = graph::get_example_graph();
    println!("{:?}", bfs::bfs(&mut graph, Node::new(8)));
    println!("{:?}", dfs::dfs(&mut graph, Node::new(8)));
    println!("{:?}", dfs::iterative_deepening_dfs(&mut graph, Node::new(8)));
}
