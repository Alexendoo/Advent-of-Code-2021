use rustc_hash::FxHashSet;
use std::{array, mem};

type Pixels = FxHashSet<(i32, i32)>;

fn square((row, col): (i32, i32)) -> array::IntoIter<(i32, i32), 9> {
    [
        (row + 1, col + 1),
        (row + 1, col),
        (row + 1, col - 1),
        (row, col + 1),
        (row, col),
        (row, col - 1),
        (row - 1, col + 1),
        (row - 1, col),
        (row - 1, col - 1),
    ]
    .into_iter()
}

struct Image<'a> {
    alg: &'a [bool],
    mask: bool,
    pixels: Pixels,
}

impl<'a> Image<'a> {
    fn set(&mut self, pixel: (i32, i32), val: bool) {
        if val ^ self.mask {
            self.pixels.insert(pixel);
        }
    }

    fn has(&self, pixel: (i32, i32)) -> bool {
        self.pixels.contains(&pixel) ^ self.mask
    }

    fn enhance(&mut self) {
        let candidates: Pixels = self.pixels.iter().copied().flat_map(square).collect();
        let prev = Image {
            alg: self.alg,
            mask: self.mask,
            pixels: mem::take(&mut self.pixels),
        };

        self.mask ^= self.alg[0];

        for &candidate in &candidates {
            let mut num = 0;
            for (i, pixel) in square(candidate).enumerate() {
                let bit = prev.has(pixel) as usize;

                num |= bit << i;
            }

            self.set(candidate, self.alg[num]);
        }
    }
}

fn parse() -> (Pixels, Vec<bool>) {
    let input = include_str!("input");

    let (alg, img) = input.split_once("\n\n").unwrap();

    let enhancement = alg
        .chars()
        .filter_map(|ch| match ch {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .collect();

    let image = img
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, ch)| match ch {
                    '#' => Some((row as i32, col as i32)),
                    _ => None,
                })
        })
        .collect();

    (image, enhancement)
}

fn main() {
    let (img, alg) = parse();

    let mut image = Image {
        alg: &alg,
        mask: false,
        pixels: img,
    };

    image.enhance();
    image.enhance();

    println!("Part 1: {}", image.pixels.len());

    for _ in 2..50 {
        image.enhance();
    }

    println!("Part 2: {}", image.pixels.len());
}
