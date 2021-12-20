use std::collections::{BTreeMap, BTreeSet};
use std::mem;

type Pixels = BTreeSet<(isize, isize)>;

struct Image<'a> {
    alg: &'a [bool],
    mask: bool,
    pixels: Pixels,
}

impl<'a> Image<'a> {
    fn set(&mut self, pixel: (isize, isize), val: bool) {
        if val ^ self.mask {
            self.pixels.insert(pixel);
        }
    }

    fn has(&self, pixel: (isize, isize)) -> bool {
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

        for &(row, col) in &candidates {
            let window = square((row, col));

            let mut num = 0;
            for (i, &pixel) in window.iter().rev().enumerate() {
                let bit = prev.has(pixel) as usize;

                num |= bit << i;
            }

            self.set((row, col), self.alg[num]);
        }
    }

    fn print(&self, from: isize, to: isize) {
        for row in from..to {
            for col in from..to {
                if self.has((row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
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
                    '#' => Some((row as isize, col as isize)),
                    _ => None,
                })
        })
        .collect();

    (image, enhancement)
}

fn print(img: &Pixels, from: isize, to: isize) {
    for row in from..to {
        for col in from..to {
            if img.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn square((row, col): (isize, isize)) -> [(isize, isize); 9] {
    [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
}

fn enhance(img: &mut Pixels, alg: &[bool]) {
    let candidates = img.iter().copied().flat_map(square).collect::<Pixels>();
    let copy = img.clone();

    for &(row, col) in &candidates {
        let window = square((row, col));

        let mut num = 0;
        for (i, pixel) in window.iter().rev().enumerate() {
            let bit = copy.contains(pixel) as usize;

            num |= bit << i;
        }

        if alg[num] {
            img.insert((row, col));
        } else {
            img.remove(&(row, col));
        }
    }
}

fn main() {
    let (mut img, alg) = parse();

    let mut image = Image {
        alg: &alg,
        mask: false,
        pixels: img,
    };

    image.enhance();
    image.enhance();

    println!("Part 1: {}", image.pixels.len());
}
