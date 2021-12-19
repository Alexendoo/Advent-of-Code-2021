use nalgebra::{Rotation3, Vector3};
use rustc_hash::FxHashSet;

type Beacon = Vector3<i32>;

#[derive(Debug, Clone)]
struct Scan {
    number: usize,
    beacons: Vec<Beacon>,
}

impl Scan {
    fn rotate(&self, rotation: &Rotation3<i32>) -> Scan {
        Scan {
            number: self.number,
            beacons: self
                .beacons
                .iter()
                .map(|beacon| rotation * beacon)
                .collect(),
        }
    }

    fn translate_beacons(&self, by: Vector3<i32>) -> impl Iterator<Item = Beacon> + '_ {
        self.beacons.iter().map(move |beacon| beacon + by)
    }
}

fn parse() -> Vec<Scan> {
    let input = include_str!("input");

    input
        .split("\n\n")
        .map(|section| {
            let mut lines = section.lines();
            let number: usize = lines.next()?.split(' ').nth(2)?.parse().ok()?;

            let beacons: Vec<Beacon> = lines
                .map(|line| Beacon::from_iterator(line.split(',').map(|n| n.parse().unwrap())))
                .collect();

            Some(Scan { number, beacons })
        })
        .collect::<Option<Vec<_>>>()
        .unwrap()
}

fn transforms() -> Vec<Rotation3<i32>> {
    let x = Vector3::x_axis();
    let y = Vector3::y_axis();
    let z = Vector3::z_axis();

    let directions = [x, y, z, -x, -y, -z];

    let mut out = Vec::with_capacity(24);
    for dir in &directions {
        for up in &directions {
            if up.abs() != dir.abs() {
                out.push(Rotation3::from_matrix_unchecked(
                    Rotation3::<f64>::face_towards(dir, up)
                        .matrix()
                        .map(|e| e as i32),
                ));
            }
        }
    }

    out
}

fn main() {
    let scans = parse();

    let mut points = FxHashSet::from_iter(scans[0].beacons.iter().copied());
    let mut scanners = vec![Vector3::zeros()];
    let mut pending = Vec::from_iter(scans.iter().skip(1));
    let transforms = transforms();

    while !pending.is_empty() {
        'pending: for (target_idx, target) in pending.iter().enumerate() {
            for transform in &transforms {
                let rotated = target.rotate(transform);

                for rotated_beacon in &rotated.beacons {
                    for source_beacon in &points {
                        let diff = source_beacon - rotated_beacon;

                        let count = rotated
                            .translate_beacons(diff)
                            .filter(|beacon| points.contains(beacon))
                            .count();

                        if count >= 12 {
                            pending.swap_remove(target_idx);
                            points.extend(rotated.translate_beacons(diff));
                            scanners.push(diff);

                            break 'pending;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", points.len());

    let mut largest = 0;
    for x in &scanners {
        for y in &scanners {
            let distance = x.iter().zip(y).map(|(l, r)| (l - r).abs()).sum();

            largest = largest.max(distance);
        }
    }

    println!("Part 2: {}", largest);
}
