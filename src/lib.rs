#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use graph::HeuristicGraph;

pub mod astar;
pub mod bestfirst;
pub mod bfs;
pub mod dfs;
pub mod dijkstra;
mod examplegraph;
pub mod graph;
pub mod graphsearcher;
mod heapelement;
pub mod perft;

pub fn gamut<G: graph::Graph>(game: &G) {
    use crate::graphsearcher::GraphSearcher;
    let mut breadthfirst = bfs::BFS::new();
    let mut depthfirst = dfs::DFS::new();
    let mut it_deep = dfs::IterDeepening::new();
    println!(
        "bfs finds the solution {} \n bfs expands {} nodes. \n bfs finds the path \n{}\n the largest frontier maintained was {} nodes.\n", 
        breadthfirst.search_tracked(game, game.root())
            .map_or_else(|| "[NO SOLUTION]".to_string(), |s| format!("{s}")),
        breadthfirst.nodes_visited(),
        breadthfirst.path().map_or_else(|| "no path".to_string(), |p| p.iter().map(|s| format!("{s}")).collect::<Vec<_>>().join("\n")),
        breadthfirst.max_frontier()
    );
    println!(
        "dfs finds the solution {} \n dfs expands {} nodes. \n dfs finds the path \n{}\n",
        depthfirst
            .search_tracked(game, game.root())
            .map_or_else(|| "[NO SOLUTION]".to_string(), |s| format!("{s}")),
        depthfirst.nodes_visited(),
        depthfirst.path().map_or_else(
            || "no path".to_string(),
            |p| p
                .iter()
                .map(|s| format!("{s}"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    );
    println!(
        "iterative deepening dfs finds the solution {} \n iterative deepening dfs expands {} nodes. \n iterative deepening dfs finds the path \n{}\n", 
        it_deep.search_tracked(game, game.root())
            .map_or_else(|| "[NO SOLUTION]".to_string(), |s| format!("{s}")),
        it_deep.nodes_visited(),
        it_deep.path().map_or_else(|| "no path".to_string(), |p| p.iter().map(|s| format!("{s}")).collect::<Vec<_>>().join("\n")),
    );
}

pub fn complex_gamut<G: graph::WeightedGraph + HeuristicGraph>(game: &G) {
    use crate::graphsearcher::GraphSearcher;
    let mut bestfirst = bestfirst::BestFirstSearch::new();
    println!(
        "best first search finds the solution {} \n best first search expands {} nodes. \n best first search finds the path \n{}\n", 
        bestfirst.search_tracked(game, game.root())
            .map_or_else(|| "[NO SOLUTION]".to_string(), |s| format!("{s}")),
        bestfirst.nodes_visited(),
        bestfirst.path().map_or_else(|| "no path".to_string(), |p| p.iter().map(|s| format!("{s}")).collect::<Vec<_>>().join("\n")),
    );
}
