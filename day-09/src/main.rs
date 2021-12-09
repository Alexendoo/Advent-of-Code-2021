use std::cmp::Reverse;

type Map = Vec<Vec<u32>>;

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
    .into_iter()
}

fn get(map: &Map, row: usize, col: usize) -> u32 {
    map.get(row)
        .and_then(|v: &Vec<_>| v.get(col).copied())
        .unwrap_or(9)
}

fn search(map: &mut Map, row: usize, col: usize) -> usize {
    let height = map[row][col];
    map[row][col] = 9;

    1 + neighbours(row, col)
        .map(|(nrow, ncol)| {
            let neighbour = get(map, nrow, ncol);

            if neighbour < 9 && neighbour > height {
                search(map, nrow, ncol)
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("input");

    let mut map: Map = input
        .lines()
        .map(|line| line.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    let low_points: Vec<(usize, usize)> = (0..width)
        .flat_map(|col| (0..height).map(move |row| (row, col)))
        .filter(|&(row, col)| {
            let height = map[row][col];

            neighbours(row, col).all(|(nrow, ncol)| height < get(&map, nrow, ncol))
        })
        .collect();

    let risk: u32 = low_points.iter().map(|&(row, col)| map[row][col] + 1).sum();

    println!("Part 1: {}", risk);

    let mut sizes = low_points
        .iter()
        .map(|&(row, col)| search(&mut map, row, col))
        .collect::<Vec<_>>();

    sizes.sort_unstable_by_key(|&n| Reverse(n));

    println!("Part 2: {}", sizes.into_iter().take(3).product::<usize>());
}
