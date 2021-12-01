use itertools::Itertools;

fn count_increased(numbers: impl Iterator<Item = usize>) -> usize {
    numbers.tuple_windows().filter(|(a, b)| a < b).count()
}

fn main() {
    let input = include_str!("input");

    let numbers: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part 1: {}", count_increased(numbers.iter().copied()));

    let windows = numbers
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c);

    println!("Part 2: {}", count_increased(windows));
}
