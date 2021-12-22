use euclid::default::{Box3D, Point3D};
use itertools::{iproduct, zip, Itertools};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
struct Step {
    on: bool,
    cube: Box3D<i64>,
}

fn solve(steps: &[Step]) -> i64 {
    let mut stops = <[BTreeSet<i64>; 3]>::default();
    let mut add_stops = |point: Point3D<i64>| {
        zip(&mut stops, point.to_array()).for_each(|(stops, v)| {
            stops.insert(v);
        });
    };

    for step in steps {
        add_stops(step.cube.min);
        add_stops(step.cube.max);
    }

    let [xs, ys, zs] = [0, 1, 2].map(|i| stops[i].iter().copied().tuple_windows());

    iproduct!(xs, ys, zs)
        .map(|((x0, x1), (y0, y1), (z0, z1))| {
            let cuboid = Box3D::new(Point3D::new(x0, y0, z0), Point3D::new(x1, y1, z1));

            if let Some(Step { on: true, .. }) =
                steps.iter().rfind(|step| step.cube.intersects(&cuboid))
            {
                cuboid.volume()
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("input");

    let steps: Vec<Step> = input
        .lines()
        .map(|line| {
            let (state, rest) = line.split_once(' ').unwrap();
            let mut ranges = rest.split(',').map(|range| {
                let (start, end) = range[2..].split_once("..").unwrap();

                (
                    start.parse::<i64>().unwrap(),
                    1 + end.parse::<i64>().unwrap(),
                )
            });
            let mut next = || ranges.next().unwrap();

            let (xmin, xmax) = next();
            let (ymin, ymax) = next();
            let (zmin, zmax) = next();

            Step {
                on: state == "on",
                cube: Box3D::new(
                    Point3D::new(xmin, ymin, zmin),
                    Point3D::new(xmax, ymax, zmax),
                ),
            }
        })
        .collect();

    let small_steps = steps
        .iter()
        .copied()
        .filter(|Step { cube, .. }| {
            cube.min.to_array().iter().all(|&min| min >= -50)
                && cube.max.to_array().iter().all(|&max| max <= 51)
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", solve(&small_steps));
    println!("Part 2: {}", solve(&steps));
}
