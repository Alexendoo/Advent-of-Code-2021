use rustc_hash::FxHashMap;
use std::mem;
use std::time::Instant;

type Pair = (char, char);
type Pairs = FxHashMap<Pair, i64>;
type Rules = FxHashMap<Pair, char>;

fn replace(pairs: &mut Pairs, rules: &Rules) {
    let mut replacements = pairs.clone();
    let mut change = |pair, by| *replacements.entry(pair).or_insert(0) += by;

    for (&pair, &count) in pairs.iter() {
        if let Some(&insertion) = rules.get(&pair) {
            change(pair, -count);
            change((pair.0, insertion), count);
            change((insertion, pair.1), count);
        }
    }

    mem::swap(pairs, &mut replacements);
}

fn result(pairs: &Pairs) -> i64 {
    fn div_ceil_2(lhs: i64) -> i64 {
        let d = lhs / 2;
        if lhs % 2 != 0 {
            d + 1
        } else {
            d
        }
    }

    let mut counts = FxHashMap::<char, i64>::default();
    for (&(l, r), &count) in pairs {
        *counts.entry(l).or_default() += count;
        *counts.entry(r).or_default() += count;
    }

    let (_, &max) = counts.iter().max_by_key(|&(_, &v)| v).unwrap();
    let (_, &min) = counts.iter().min_by_key(|&(_, &v)| v).unwrap();

    div_ceil_2(max) - div_ceil_2(min)
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input");

    let (template, rules) = input.split_once("\n\n").unwrap();

    let mut pairs = Pairs::default();
    template
        .chars()
        .zip(template.chars().skip(1))
        .for_each(|pair| {
            *pairs.entry(pair).or_insert(0) += 1;
        });

    let rules: Rules = rules
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" -> ").unwrap();
            let mut l_chars = l.chars();

            (
                (l_chars.next().unwrap(), l_chars.next().unwrap()),
                r.chars().next().unwrap(),
            )
        })
        .collect();

    for _ in 1..=10 {
        replace(&mut pairs, &rules);
    }
    let part_1 = result(&pairs);

    for _ in 11..=40 {
        replace(&mut pairs, &rules);
    }
    let part_2 = result(&pairs);
    let elapsed = Instant::now() - start;

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    println!("Elapsed {:?}", elapsed);
}
