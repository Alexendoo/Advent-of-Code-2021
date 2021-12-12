#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Node {
    Start,
    End,
    Small(&'static str),
    Big(&'static str),
}
use Node::*;
type Edge = (Node, Node);

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
}

impl<'a> Walker<'a> {
    fn new(edges: &'a [Edge]) -> Self {
        Self {
            stack: vec![Start],
            edges,
            finishes: 0,
        }
    }

    fn neighbours(&self, current: Node) -> Vec<Node> {
        fn neigh(next: Node, stack: &[Node]) -> Option<Node> {
            match next {
                Small(_) if stack.contains(&next) => None,
                Start => None,
                _ => Some(next),
            }
        }

        self.edges
            .iter()
            .filter_map(|&(left, right)| {
                if left == current {
                    neigh(right, &self.stack)
                } else if right == current {
                    neigh(left, &self.stack)
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
                self.finishes += 1;
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

    let mut walker = Walker::new(&edges);
    walker.walk();

    println!("Part 1: {}", walker.finishes);
}
