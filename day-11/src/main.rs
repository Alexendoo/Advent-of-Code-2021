use std::collections::HashSet;

type Grid = Vec<Vec<u8>>;

fn print(grid: &Grid) {
    for row in grid {
        for &energy in row {
            if energy > 9 {
                print!("+");
            } else {
                print!("{}", energy)
            }
        }
        println!();
    }
}

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

fn flashes(grid: &mut Grid) -> usize {
    const FLASH: u8 = 128;

    let mut flashes = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let energy = &mut grid[y][x];
            if *energy > 9 && *energy < FLASH {
                *energy = FLASH;
                flashes += 1;

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

    let mut flashed = 0;
    loop {
        let n = flashes(grid);
        if n == 0 {
            break;
        }
        flashed += n;
    }

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

    let total_flashes: usize = (0..100).map(|_| step(&mut grid)).sum();

    print(&grid);

    println!("Part 1: {}", total_flashes);
}
