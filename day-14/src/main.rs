use std::time::Instant;

type Pair = usize;
type Pairs = [Pair; 1024];

fn pair_from_letters(a: u8, b: u8) -> Pair {
    fn letter(byte: u8) -> usize {
        (byte - b'A') as usize
    }

    letter(a) + (letter(b) << 5)
}

#[derive(Debug)]
struct Rule {
    replaces: Pair,
    with: (Pair, Pair),
    times: usize,
}

impl Rule {
    fn new(line: &str) -> Self {
        if let &[l, r, .., insert] = line.as_bytes() {
            Self {
                replaces: pair_from_letters(l, r),
                with: (pair_from_letters(l, insert), pair_from_letters(insert, r)),
                times: 0,
            }
        } else {
            unreachable!()
        }
    }
}

fn replace(pairs: &mut Pairs, rules: &mut [Rule]) {
    for rule in rules.iter_mut() {
        rule.times = pairs[rule.replaces];
    }

    for &Rule {
        replaces,
        with,
        times,
    } in rules.iter()
    {
        pairs[replaces] -= times;
        pairs[with.0] += times;
        pairs[with.1] += times;
    }
}

fn result(pairs: &Pairs) -> usize {
    fn div_ceil_2(lhs: usize) -> usize {
        lhs / 2 + (lhs & 1)
    }

    let mut counts = [0; 32];

    for (i, &count) in pairs.iter().enumerate() {
        counts[i & 0b11111] += count;
        counts[i >> 5] += count;
    }

    let iter = || counts.iter().copied();
    let max = iter().max().unwrap();
    let min = iter().filter(|&count| count > 0).min().unwrap();

    div_ceil_2(max) - div_ceil_2(min)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input");

    let (template, rules) = input.split_once("\n\n").unwrap();

    let mut pairs = [0; 1024];
    for window in template.as_bytes().windows(2) {
        pairs[pair_from_letters(window[0], window[1])] += 1;
    }

    let mut rules: Vec<Rule> = rules.lines().map(Rule::new).collect();

    for _ in 1..=10 {
        replace(&mut pairs, &mut rules);
    }
    let part_1 = result(&pairs);

    for _ in 11..=40 {
        replace(&mut pairs, &mut rules);
    }
    let part_2 = result(&pairs);
    let elapsed = Instant::now() - start;

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Elapsed {:?}", elapsed);
}
