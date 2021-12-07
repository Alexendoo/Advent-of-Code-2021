fn main() {
    let input = include_str!("input");

    let mut crabs: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    crabs.sort();
    let mode = crabs[crabs.len() / 2];

    println!(
        "Part 1: {}",
        crabs.iter().map(|x| (x - mode).abs()).sum::<i32>()
    );

    let mean = crabs.iter().sum::<i32>() / crabs.len() as i32;

    let consumption = |x: &i32| {
        let diff = (x - mean).abs();
        (diff * (diff + 1)) / 2
    };

    println!("Part 2: {}", crabs.iter().map(consumption).sum::<i32>());
}
