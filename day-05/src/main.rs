use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    let mut numbers = input
        .split(|ch: char| !ch.is_ascii_digit())
        .flat_map(|digit| digit.parse::<usize>())
        .peekable();

    let mut lines = Vec::new();
    while numbers.peek().is_some() {
        let mut next = || numbers.next().unwrap();

        lines.push(((next(), next()), (next(), next())));
    }

    let mut vents: HashMap<(usize, usize), usize> = HashMap::new();
    for &((x1, y1), (x2, y2)) in &lines {
        println!("{},{} -> {},{}", x1, y1, x2, y2);

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                *vents.entry((x1, y)).or_default() += 1;
            }
        }

        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                *vents.entry((x, y1)).or_default() += 1;
            }
        }
    }

    let overlaps = vents.values().filter(|&&c| c > 1).count();

    println!("Part 1: {}", overlaps);
}
