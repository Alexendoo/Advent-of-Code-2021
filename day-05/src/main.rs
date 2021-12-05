use std::cmp::max;
use std::collections::HashMap;

type Vent = (i32, i32);
type Vents = HashMap<Vent, i32>;

fn plot(vents: &mut Vents, (x1, y1): Vent, (x2, y2): Vent) {
    let dx = x2.cmp(&x1) as i32;
    let dy = y2.cmp(&y1) as i32;

    let count = max((x1 - x2).abs(), (y1 - y2).abs());
    for i in 0..=count {
        let x = x1 + i * dx;
        let y = y1 + i * dy;
        *vents.entry((x, y)).or_default() += 1;
    }
}

fn overlaps(vents: &Vents) -> usize {
    vents.values().filter(|&&c| c > 1).count()
}

fn is_diagonal((x1, y1): Vent, (x2, y2): Vent) -> bool {
    x1 != x2 && y1 != y2
}

fn main() {
    let input = include_str!("input");

    let mut numbers = input
        .split(|ch: char| !ch.is_ascii_digit())
        .flat_map(|digit| digit.parse::<i32>())
        .peekable();

    let mut lines = Vec::new();
    while numbers.peek().is_some() {
        let mut next = || numbers.next().unwrap();

        lines.push(((next(), next()), (next(), next())));
    }

    let mut vents: Vents = HashMap::new();
    for &(a, b) in &lines {
        if !is_diagonal(a, b) {
            plot(&mut vents, a, b);
        }
    }

    println!("Part 1: {}", overlaps(&vents));

    for &(a, b) in &lines {
        if is_diagonal(a, b) {
            plot(&mut vents, a, b);
        }
    }

    println!("Part 2: {}", overlaps(&vents));
}
