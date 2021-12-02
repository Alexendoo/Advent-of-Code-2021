fn main() {
    let input = include_str!("input");

    let (x, depth) = input
        .lines()
        .map(|line| {
            let (direction, by) = line.split_once(" ").unwrap();
            (direction, by.parse::<usize>().unwrap())
        })
        .fold((0, 0), |(x, depth), (direction, by)| match direction {
            "forward" => (x + by, depth),
            "down" => (x, depth + by),
            "up" => (x, depth - by),
            _ => unreachable!()
        });

    println!("Part 1: {}", x * depth);
}
