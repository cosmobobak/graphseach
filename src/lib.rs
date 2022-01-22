#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

pub mod graph;
pub mod bfs;
pub mod dfs;
pub mod graphsearcher;

pub fn gamut<G: graph::Graph>(game: &G) {
    use crate::graphsearcher::GraphSearcher;
    let mut breadthfirst = bfs::BFS::new();
    let mut depthfirst = dfs::DFS::new();
    let mut it_deep = dfs::IterDeepening::new();
    println!(
        "bfs finds the solution {} \n bfs expands {} nodes. \n the largest frontier maintained was {} nodes.", 
        breadthfirst.search_tracked(game, G::root())
            .expect("bfs failed to find a solution"),
        breadthfirst.nodes_visited(),
        breadthfirst.max_frontier()
    );
    println!(
        "dfs finds the solution {} \n dfs expands {} nodes.", 
        depthfirst.search_tracked(game, G::root())
            .expect("dfs failed to find a solution"),
        depthfirst.nodes_visited()
    );
    println!(
        "iterative deepening dfs finds the solution {} \n iterative deepening dfs expands {} nodes.", 
        it_deep.search_tracked(game, G::root())
            .expect("iterative deepening dfs failed to find a solution"),
        it_deep.nodes_visited()
    );
}