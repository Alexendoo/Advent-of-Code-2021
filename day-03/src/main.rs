fn main() {
    let input = include_str!("input");

    let width = input.lines().next().unwrap().len();

    let numbers: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    let mut gamma = 0;
    for pos in 0..width {
        let ones = numbers
            .iter()
            .filter(|&number| number & (1 << pos) > 0)
            .count();

        if ones > numbers.len() / 2 {
            gamma |= 1 << pos
        }
    }
    let epsilon = !gamma & !(!0 << width);

    println!("Part 1: {}", gamma * epsilon)
}
