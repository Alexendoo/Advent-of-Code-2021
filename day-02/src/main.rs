fn main() {
    let input = include_str!("input");

    let commands: Vec<(&str, usize)> = input
        .lines()
        .map(|line| {
            let (direction, by) = line.split_once(" ").unwrap();
            (direction, by.parse::<usize>().unwrap())
        })
        .collect();

    let (x, depth) = commands
        .iter()
        .fold((0, 0), |(x, depth), &(direction, by)| match direction {
            "forward" => (x + by, depth),
            "down" => (x, depth + by),
            "up" => (x, depth - by),
            _ => unreachable!(),
        });

    println!("Part 1: {}", x * depth);

    let (x, depth, _) =
        commands.iter().fold(
            (0, 0, 0),
            |(x, depth, aim), &(direction, by)| match direction {
                "forward" => (x + by, depth + aim * by, aim),
                "down" => (x, depth, aim + by),
                "up" => (x, depth, aim - by),
                _ => unreachable!(),
            },
        );

    println!("Part 2: {}", x * depth);
}
