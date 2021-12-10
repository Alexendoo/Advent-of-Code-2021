enum Parsed {
    Error(char),
    Incomplete(Vec<char>),
}
use Parsed::{Error, Incomplete};

fn parse(line: &str) -> Parsed {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ch => {
                if stack.pop() != Some(ch) {
                    return Error(ch);
                }
            }
        }
    }

    Incomplete(stack)
}

fn main() {
    let input = include_str!("input");

    let results: Vec<Parsed> = input.lines().map(parse).collect();

    let error_score: usize = results
        .iter()
        .map(|parsed| match parsed {
            Error(')') => 3,
            Error(']') => 57,
            Error('}') => 1197,
            Error('>') => 25137,
            _ => 0,
        })
        .sum();

    println!("Part 1: {}", error_score);

    let mut scores: Vec<usize> = results
        .into_iter()
        .filter_map(|parsed| match parsed {
            Incomplete(stack) => Some(stack),
            Error(_) => None,
        })
        .map(|stack| {
            stack.into_iter().rev().fold(0, |acc, ch| {
                acc * 5
                    + match ch {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();
    scores.sort_unstable();

    println!("Part 2: {}", scores[scores.len() / 2]);
}
