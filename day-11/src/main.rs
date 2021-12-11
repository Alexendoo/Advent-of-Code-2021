use std::iter;

type Grid = Vec<Vec<u8>>;

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x, y.wrapping_sub(1)),
        (x + 1, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x.wrapping_sub(1), y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
}

fn flashes(grid: &mut Grid) -> Option<usize> {
    const FLASH: u8 = 128;

    let mut flashes = None;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let energy = &mut grid[y][x];
            if *energy > 9 && *energy < FLASH {
                *energy = FLASH;
                *flashes.get_or_insert(0) += 1;

                for (nx, ny) in neighbours(x, y) {
                    if let Some(row) = grid.get_mut(ny) {
                        if let Some(energy) = row.get_mut(nx) {
                            *energy += 1;
                        }
                    }
                }
            }
        }
    }

    flashes
}

fn step(grid: &mut Grid) -> usize {
    for row in grid.iter_mut() {
        for energy in row.iter_mut() {
            *energy += 1;
        }
    }

    let flashed: usize = iter::from_fn(|| flashes(grid)).sum();

    for row in grid.iter_mut() {
        for energy in row.iter_mut() {
            if *energy > 9 {
                *energy = 0;
            }
        }
    }

    flashed
}

fn main() {
    let input = include_str!("input");

    let mut grid: Grid = input
        .lines()
        .map(|line| {
            line.bytes()
                .filter(u8::is_ascii_digit)
                .map(|digit| digit - b'0')
                .collect()
        })
        .collect();

    let total_flashes: usize = {
        let mut grid = grid.clone();
        (0..100).map(|_| step(&mut grid)).sum()
    };
    println!("Part 1: {}", total_flashes);

    let target = grid.len() * grid[0].len();
    let mut steps = 1;
    while step(&mut grid) != target {
        steps += 1;
    }
    println!("Part 2: {}", steps)
}
