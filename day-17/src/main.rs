use std::cmp::max;
use std::ops::Range;

#[derive(Debug)]
struct Target {
    x: Range<isize>,
    y: Range<isize>,
}

fn parse() -> Option<Target> {
    fn parse_range(range: &str) -> Option<Range<isize>> {
        let (start, end) = range.split_once("..")?;
        Some(start.parse().ok()?..(1 + end.parse::<isize>().ok()?))
    }

    let input = include_str!("input");

    let (l, r) = input
        .trim()
        .strip_prefix("target area: x=")?
        .split_once(", y=")?;

    Some(Target {
        x: parse_range(l)?,
        y: parse_range(r)?,
    })
}

fn inverse_sum(x: isize) -> f64 {
    (-1. + (1. + 8. * x as f64).sqrt()) / 2.
}

fn reaches(target: &Target, mut vx: isize, mut vy: isize) -> (bool, isize) {
    let mut x = 0;
    let mut y = 0;

    let mut highest_y = 0;

    loop {
        x += vx;
        y += vy;

        highest_y = max(highest_y, y);

        vx = max(vx - 1, 0);
        vy -= 1;

        if x > target.x.end || y < target.y.start {
            return (false, 0);
        }

        if target.x.contains(&x) && target.y.contains(&y) {
            return (true, highest_y);
        }
    }
}

fn main() {
    let target = parse().unwrap();

    let max_vx = target.x.end;
    let min_vx = inverse_sum(target.x.start).ceil() as isize;

    let max_vy = -target.y.start;
    let min_vy = target.y.start;

    let mut highest_y = 0;
    let mut valid = 0;

    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            let (reached, high) = reaches(&target, vx, vy);
            if reached {
                highest_y = max(high, highest_y);
                valid += 1;
            }
        }
    }

    println!("Part 1: {}", highest_y);
    println!("Part 2: {}", valid);
}
