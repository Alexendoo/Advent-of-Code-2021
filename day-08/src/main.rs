use std::char;
use std::collections::BTreeMap;
use std::io::Write;

fn solve(patterns: &[&str]) -> [char; 7] {
    let mut occurences = BTreeMap::<char, usize>::new();
    for letter in patterns.iter().flat_map(|s| s.chars()) {
        *occurences.entry(letter).or_default() += 1;
    }
    let occurs = |n| {
        *occurences
            .iter()
            .find(move |&(_, &count)| count == n)
            .unwrap()
            .0
    };
    let diff = |pattern_len, nots: &[char]| {
        let pattern = patterns
            .iter()
            .copied()
            .find(|pattern| pattern.len() == pattern_len)
            .unwrap();

        pattern.chars().find(|ch| !nots.contains(ch)).unwrap()
    };

    let e = occurs(4);
    let b = occurs(6);
    let f = occurs(9);

    // digit 1
    let c = diff(2, &[f]);
    // digit 4
    let d = diff(4, &[b, c, f]);
    // digit 7
    let a = diff(3, &[c, f]);
    // digit 8
    let g = diff(7, &[a, b, c, d, e, f]);

    [a, b, c, d, e, f, g]
}

fn digit(solution: [char; 7], output: &str) -> usize {
    let segments = solution.map(|ch| output.contains(ch));
    let has = |ch| segments[ch as usize - 'a' as usize];
    let horiz = |mut out: &mut &mut [u8], ch| {
        write!(out, " {0}{0}{0}{0} ", if has(ch) { ch } else { '.' }).unwrap()
    };
    let vert = |mut out: &mut &mut [u8], ch1, ch2| {
        for _ in 0..2 {
            write!(
                out,
                " {}    {}",
                if has(ch1) { ch1 } else { '.' },
                if has(ch2) { ch2 } else { '.' }
            )
            .unwrap();
        }
    };

    let mut display = [0_u8; 46];
    let mut w = &mut display[..];
    horiz(&mut w, 'a');
    vert(&mut w, 'b', 'c');
    horiz(&mut w, 'd');
    vert(&mut w, 'e', 'f');
    horiz(&mut w, 'g');

    let digit = match &display {
        b" aaaa  \
          b    c \
          b    c \
           ....  \
          e    f \
          e    f \
           gggg " => 0,

        b" ....  \
          .    c \
          .    c \
           ....  \
          .    f \
          .    f \
           .... " => 1,

        b" aaaa  \
          .    c \
          .    c \
           dddd  \
          e    . \
          e    . \
           gggg " => 2,

        b" aaaa  \
          .    c \
          .    c \
           dddd  \
          .    f \
          .    f \
           gggg " => 3,

        b" ....  \
          b    c \
          b    c \
           dddd  \
          .    f \
          .    f \
           .... " => 4,

        b" aaaa  \
          b    . \
          b    . \
           dddd  \
          .    f \
          .    f \
           gggg " => 5,

        b" aaaa  \
          b    . \
          b    . \
           dddd  \
          e    f \
          e    f \
           gggg " => 6,

        b" aaaa  \
          .    c \
          .    c \
           ....  \
          .    f \
          .    f \
           .... " => 7,

        b" aaaa  \
          b    c \
          b    c \
           dddd  \
          e    f \
          e    f \
           gggg " => 8,

        b" aaaa  \
          b    c \
          b    c \
           dddd  \
          .    f \
          .    f \
           gggg " => 9,

        other => unreachable!(),
    };

    digit
}

fn main() {
    let input = include_str!("input");

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in input.lines() {
        let note: Vec<&str> = line
            .split_ascii_whitespace()
            .filter(|&s| s != "|")
            .collect();
        let patterns = &note[..10];
        let outputs = &note[10..];

        for output in outputs {
            match output.len() {
                2 | 4 | 3 | 7 => part_1 += 1,
                _ => {}
            }
        }

        let solution = solve(patterns);

        let count: usize = outputs
            .iter()
            .rev()
            .enumerate()
            .map(|(i, output)| digit(solution, output) * 10_usize.pow(i as u32))
            .sum();
        part_2 += count;
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
