use std::cmp::Ordering;

fn ones(numbers: &[usize], pos: usize) -> Ordering {
    let count = numbers
        .iter()
        .filter(|&number| (number >> pos) & 1 == 1)
        .count();

    (count * 2).cmp(&numbers.len())
}

fn rating(mut numbers: Vec<usize>, width: usize, compare: fn(Ordering) -> bool) -> usize {
    for pos in (0..width).rev() {
        let keep = compare(ones(&numbers, pos)) as usize;
        numbers.retain(|&number| (number >> pos) & 1 == keep);

        if numbers.len() == 1 {
            break;
        }
    }

    numbers[0]
}

fn main() {
    let input = include_str!("input");

    let width = input.lines().next().unwrap().len();

    let numbers: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let mut gamma = 0;
    for pos in 0..width {
        if ones(&numbers, pos).is_gt() {
            gamma |= 1 << pos;
        }
    }
    let epsilon = !gamma & !(!0 << width);

    println!("Part 1: {}", gamma * epsilon);

    let oxygen = rating(numbers.clone(), width, Ordering::is_ge);
    let co2 = rating(numbers, width, Ordering::is_lt);

    println!("Part 2: {}", oxygen * co2);
}
