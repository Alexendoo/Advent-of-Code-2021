fn parse(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ch => {
                if stack.pop() != Some(ch) {
                    return Some(ch);
                }
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("input");

    let score: usize = input
        .lines()
        .filter_map(parse)
        .map(|ch| match ch {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum();

    println!("Part 1: {}", score);
}
