fn main() {
    let input = include_str!("input");

    let mut crabs: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    crabs.sort();
    let mode = crabs[crabs.len() / 2];

    println!("Part 1: {}", crabs.iter().map(|x| (x - mode).abs()).sum::<i32>());
}
