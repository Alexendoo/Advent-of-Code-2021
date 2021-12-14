use std::collections::HashMap;

type Pair = (char, char);
// type Pairs = HashMap<Pair, i32>;
type Rules = HashMap<Pair, char>;

// fn replace(pairs: &mut Pairs, rules: &Rules) {
//     let mut replacements = Pairs::default();

//     for (&pair, &count) in pairs.iter() {
//         if let Some(&insertion) = rules.get(&pair) {
//             replacements.insert(pair, -count);
//             replacements.insert((pair.0, insertion), count);
//             replacements.insert((insertion, pair.1), count);
//         }
//     }

//     for (pair, count) in replacements {
//         *pairs.entry(pair).or_insert(0) += count;
//     }
// }


fn replace(polymer: &mut String, rules: &Rules) {
    let mut i = 0;
    while i < polymer.len() - 1 {
        let mut chars = polymer[i..].chars();
        let pair = (chars.next().unwrap(), chars.next().unwrap());

        if let Some(&insertion) = rules.get(&pair) {
            polymer.insert(i + 1, insertion);

            i += 2;
        } else {
            i += 1;
        }
    }
}

fn main() {
    let input = include_str!("input");

    let (template, rules) = input.split_once("\n\n").unwrap();

    // let mut pairs = Pairs::default();
    // template
    //     .chars()
    //     .zip(template.chars().skip(1))
    //     .for_each(|pair| {
    //         *pairs.entry(pair).or_insert(0) += 1;
    //     });

    let mut polymer = template.to_string();
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

    for _ in 0..10 {
        replace(&mut polymer, &rules);
        println!("{}", polymer.len());
    }

    let mut counts = HashMap::<char, usize>::new();
    for element in polymer.chars() {
        *counts.entry(element).or_insert(0) += 1;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("Part 1: {}", max - min);

    // println!("{:?}", pairs);

    // for step in 1..10 {
    //     replace(&mut pairs, &rules);
    //     let len = pairs.values().copied().sum::<i32>() + 1;
    //     // println!("\t{:?}", pairs);
    //     println!("step {}\tlen {}", step, len);
    // }
}
