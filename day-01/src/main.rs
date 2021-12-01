use itertools::Itertools;
use std::io::BufRead;
use std::iter::once;

fn main() {
    let input = include_bytes!("input");

    let numbers = input
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap());

    let p1 = once(usize::MAX)
        .chain(numbers)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("{}", p1);
}
