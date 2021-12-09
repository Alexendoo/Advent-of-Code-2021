fn main() {
    let input = include_str!("input");

    let numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

    let width = numbers[0].len();
    let height = numbers.len();

    let get = |row, col| {
        numbers
            .get(row)
            .and_then(|v: &Vec<_>| v.get(col).copied())
            .unwrap_or(u32::MAX)
    };

    let risk: u32 = (0..width)
        .flat_map(|col| (0..height).map(move |row| (row, col)))
        .filter_map(|(row, col)| {
            let n = numbers[row][col];

            let surrounds = [
                get(row + 1, col),
                get(row, col + 1),
                get(row.wrapping_sub(1), col),
                get(row, col.wrapping_sub(1)),
            ];

            if surrounds.iter().all(|&x| n < x) {
                Some(n + 1)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", risk);
}
