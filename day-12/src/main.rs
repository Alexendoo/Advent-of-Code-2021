use std::collections::HashSet;
use std::fmt;
use Node::*;

type Edge = (Node, Node);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Small(&'static str),
    Big(&'static str),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Start => write!(f, "start"),
            End => write!(f, "end"),
            Big(s) | Small(s) => write!(f, "{}", s),
        }
    }
}

impl Node {
    fn new(s: &'static str) -> Node {
        match s {
            "start" => Start,
            "end" => End,
            _ if s.chars().all(|ch| ch.is_ascii_lowercase()) => Small(s),
            _ => Big(s),
        }
    }
}

struct Walker<'a> {
    stack: Vec<Node>,
    edges: &'a [Edge],
    finishes: usize,
    twice: Option<Node>,
}

impl<'a> Walker<'a> {
    fn new(edges: &'a [Edge], twice: Option<Node>) -> Self {
        Self {
            stack: vec![Start],
            edges,
            finishes: 0,
            twice,
        }
    }

    fn seen_twice(&self, next: Node) -> bool {
        self.stack.iter().filter(|&&node| node == next).count() > 1
    }

    fn neigh(&self, next: Node) -> Option<Node> {
        match next {
            Small(_) => {
                if Some(next) == self.twice {
                    if self.seen_twice(next) {
                        None
                    } else {
                        Some(next)
                    }
                } else if self.stack.contains(&next) {
                    None
                } else {
                    Some(next)
                }
            }
            Start => None,
            _ => Some(next),
        }
    }

    fn neighbours(&self, current: Node) -> Vec<Node> {
        self.edges
            .iter()
            .filter_map(|&(left, right)| {
                if left == current {
                    self.neigh(right)
                } else if right == current {
                    self.neigh(left)
                } else {
                    None
                }
            })
            .collect()
    }

    fn walk(&mut self) {
        let current = *self.stack.last().unwrap();

        for neighbour in self.neighbours(current) {
            self.stack.push(neighbour);

            if neighbour == End {
                if let Some(twice_node) = self.twice {
                    if self.seen_twice(twice_node) {
                        self.finishes += 1;
                    }
                } else {
                    self.finishes += 1;
                }
            } else {
                self.walk();
            }

            self.stack.pop();
        }
    }
}

fn main() {
    let input = include_str!("input");

    let edges: Vec<Edge> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();

            (Node::new(left), Node::new(right))
        })
        .collect();

    let mut walker = Walker::new(&edges, None);
    walker.walk();

    let no_repeats = walker.finishes;
    println!("Part 1: {}", no_repeats);

    let small_nodes: HashSet<Node> = edges
        .iter()
        .flat_map(|&(left, right)| [left, right])
        .filter(|node| matches!(node, Small(_)))
        .collect();

    let repeats: usize = small_nodes
        .into_iter()
        .map(|small| {
            let mut walker = Walker::new(&edges, Some(small));
            walker.walk();
            walker.finishes
        })
        .sum();

    println!("Part 2: {}", no_repeats + repeats);
}
