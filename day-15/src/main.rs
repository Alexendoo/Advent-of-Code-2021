use petgraph::algo::dijkstra;
use petgraph::graph::{node_index, UnGraph};
use petgraph::visit::EdgeRef;

type Board = Vec<Vec<u32>>;

fn search(board: &Board) -> u32 {
    let width = board[0].len();
    let height = board.len();
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

    let start = node_index(0);
    let end = node_index(capacity - 1);

    let map = dijkstra(&graph, start, Some(end), |e| {
        *graph.node_weight(e.target()).unwrap()
    });

    map[&end]
}

fn risk(tile: &mut u32) {
    *tile = if *tile >= 9 { 1 } else { *tile + 1 }
}

fn main() {
    let input = include_str!("input");

    let board: Board = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let width = board[0].len();
    let height = board.len();

    println!("Part 1: {}", search(&board));

    let mut expanded = board.clone();

    for repeat in 1..=4 {
        let mid = repeat * width;
        let prev_start = mid - width;
        let current_end = mid + width;

        for row in &mut expanded {
            row.extend_from_within(prev_start..mid);
            for tile in &mut row[mid..current_end] {
                risk(tile);
            }
        }
    }

    for repeat in 1..=4 {
        let mid = repeat * height;
        let prev_start = mid - height;
        let current_end = mid + height;

        expanded.extend_from_within(prev_start..mid);
        for row in &mut expanded[mid..current_end] {
            for tile in row {
                risk(tile);
            }
        }
    }

    println!("Part 2: {}", search(&expanded));
}
