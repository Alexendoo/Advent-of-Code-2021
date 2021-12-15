use petgraph::algo::dijkstra;
use petgraph::graph::{node_index, UnGraph};
use petgraph::visit::EdgeRef;

fn main() {
    let input = include_str!("input");

    let board: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let height = board.len();
    let width = board[0].len();
    let capacity = width * height;

    let mut graph = UnGraph::<u32, ()>::with_capacity(capacity, capacity * 2 - width - height);

    for row in 0..height {
        for col in 0..width {
            let to = graph.add_node(board[row][col]);

            if col > 0 {
                let from = node_index(to.index() - 1);
                graph.add_edge(from, to, ());
            }

            if row > 0 {
                let from = node_index(to.index() - width);
                graph.add_edge(from, to, ());
            }
        }
    }

    let end = node_index(capacity - 1);

    let map = dijkstra(
        &graph,
        node_index(0),
        Some(end),
        |e| *graph.node_weight(e.source()).unwrap()
    );

    println!("Part 1: {}", map[&end])
}
