#[derive(Clone, Copy, Debug)]
struct Element {
    value: u32,

    /// leading `[`s
    open: u32,
    /// trailing `]`s
    close: u32,
}

fn parse() -> Vec<Vec<Element>> {
    let input = include_str!("input");

    input
        .lines()
        .map(|line| {
            let mut snailfish = Vec::<Element>::new();

            let mut open = 0;

            for ch in line.chars() {
                match ch {
                    '[' => open += 1,
                    ']' => snailfish.last_mut().unwrap().close += 1,
                    ',' => {}
                    digit => {
                        snailfish.push(Element {
                            value: digit.to_digit(10).unwrap(),
                            open,
                            close: 0,
                        });
                        open = 0;
                    }
                }
            }

            snailfish
        })
        .collect()
}

fn explode(fish: &mut Vec<Element>) -> bool {
    let mut i = 0;
    let mut depth = 0;

    while i < fish.len() - 1 {
        let left = fish[i];

        depth += left.open;

        if depth > 4 {
            let right = fish[i + 1];

            if right.open == 0 {
                if let Some(before) = fish.get_mut(i.wrapping_sub(1)) {
                    before.value += left.value;
                }

                if let Some(after) = fish.get_mut(i + 2) {
                    after.value += right.value;
                }

                fish[i] = Element {
                    value: 0,
                    open: left.open - 1,
                    close: right.close - 1,
                };
                fish.remove(i + 1);

                return true;
            }
        }

        i += 1;
        depth -= left.close;
    }

    false
}

fn split(fish: &mut Vec<Element>) -> bool {
    fn div_ceil_2(lhs: u32) -> u32 {
        lhs / 2 + (lhs & 1)
    }

    if let Some(i) = fish.iter().position(|el| el.value >= 10) {
        let current = fish[i];

        fish[i] = Element {
            value: current.value / 2,
            open: current.open + 1,
            close: 0,
        };
        fish.insert(
            i + 1,
            Element {
                value: div_ceil_2(current.value),
                open: 0,
                close: current.close + 1,
            },
        );

        true
    } else {
        false
    }
}

fn reduce(fish: &mut Vec<Element>) {
    while explode(fish) || split(fish) {}
}

fn add(a: &mut Vec<Element>, b: &[Element]) {
    a.extend_from_slice(b);

    a[0].open += 1;
    a.last_mut().unwrap().close += 1;
}

fn magnitude(fish: &mut [Element]) -> u32 {
    match fish {
        [single] => return single.value,
        [start, .., end] => {
            start.open -= 1;
            end.close -= 1;
        }
        [] => unreachable!(),
    }

    let mut depth = 0;
    let mut i = 0;

    while i < fish.len() {
        let current = fish[i];

        depth += current.open;
        depth -= current.close;

        if depth == 0 {
            let (left, right) = fish.split_at_mut(i + 1);

            return 3 * magnitude(left) + 2 * magnitude(right);
        }

        i += 1;
    }

    unreachable!()
}

fn main() {
    let fishes = parse();

    let mut sum = fishes[0].clone();

    for fish in fishes.iter().skip(1) {
        add(&mut sum, fish);
        reduce(&mut sum);
    }

    println!("Part 1: {}", magnitude(&mut sum));

    let largest = (0..fishes.len())
        .flat_map(|a| (0..fishes.len()).map(move |b| (a, b)))
        .filter(|(a, b)| a != b)
        .map(|(a, b)| {
            let mut sum = fishes[a].clone();
            add(&mut sum, &fishes[b]);
            reduce(&mut sum);

            magnitude(&mut sum)
        })
        .max()
        .unwrap();

    println!("Part 2: {}", largest);
}
