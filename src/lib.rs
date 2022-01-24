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
pub mod perft;
mod examplegraph;

pub fn gamut<G: graph::Graph>(game: &G) {
    use crate::graphsearcher::GraphSearcher;
    let mut breadthfirst = bfs::BFS::new();
    let mut depthfirst = dfs::DFS::new();
    let mut it_deep = dfs::IterDeepening::new();
    println!(
        "bfs finds the solution {} \n bfs expands {} nodes. \n bfs finds the path \n{}\n the largest frontier maintained was {} nodes.\n", 
        breadthfirst.search_tracked(game, G::root())
            .expect("bfs failed to find a solution"),
        breadthfirst.nodes_visited(),
        breadthfirst.path().iter().map(|s| format!("{}", s)).collect::<Vec<_>>().join("\n"),
        breadthfirst.max_frontier()
    );
    println!(
        "dfs finds the solution {} \n dfs expands {} nodes. \n dfs finds the path \n{}\n", 
        depthfirst.search_tracked(game, G::root())
            .expect("dfs failed to find a solution"),
        depthfirst.nodes_visited(),
        depthfirst.path().iter().map(|s| format!("{}", s)).collect::<Vec<_>>().join("\n")
    );
    println!(
        "iterative deepening dfs finds the solution {} \n iterative deepening dfs expands {} nodes. \n iterative deepening dfs finds the path \n{}\n", 
        it_deep.search_tracked(game, G::root())
            .expect("iterative deepening dfs failed to find a solution"),
        it_deep.nodes_visited(),
        it_deep.path().iter().map(|s| format!("{}", s)).collect::<Vec<_>>().join("\n")
    );
}