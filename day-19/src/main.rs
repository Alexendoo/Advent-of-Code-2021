use nalgebra::{Rotation3, Vector3};
use std::collections::HashSet;

type Beacon = Vector3<i32>;

#[derive(Debug, Clone)]
struct Scan {
    number: usize,
    pos: Vector3<i32>,
    beacons: Vec<Beacon>,
}

impl Scan {
    fn rotate(&self, rotation: &Rotation3<i32>) -> Scan {
        Scan {
            number: self.number,
            pos: self.pos,
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

            Some(Scan {
                number,
                pos: Vector3::zeros(),
                beacons,
            })
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

    let mut matched = vec![scans[0].clone()];
    let mut pending = Vec::from_iter(scans.iter().skip(1));
    let transforms = transforms();

    while !pending.is_empty() {
        'source: for source in &matched {
            for (target_idx, target) in pending.iter().enumerate() {
                for transform in &transforms {
                    let mut rotated = target.rotate(transform);

                    for rotated_beacon in &rotated.beacons {
                        for source_beacon in &source.beacons {
                            let diff = source_beacon - rotated_beacon;

                            let count = rotated
                                .translate_beacons(diff)
                                .filter(|beacon| source.beacons.contains(beacon))
                                .count();

                            if count >= 12 {
                                pending.swap_remove(target_idx);
                                rotated.beacons = rotated.translate_beacons(diff).collect();
                                rotated.pos = diff;
                                matched.push(rotated);

                                println!("{:>2}/{}", matched.len(), scans.len());

                                break 'source;
                            }
                        }
                    }
                }
            }
        }
    }

    let all_beacons = matched
        .iter()
        .flat_map(|scan| &scan.beacons)
        .collect::<HashSet<_>>();

    println!("Part 1: {}", all_beacons.len());

    let mut largest = 0;
    for x in &matched {
        for y in &matched {
            let distance = x
                .pos
                .iter()
                .zip(y.pos.iter())
                .map(|(l, r)| (l - r).abs())
                .sum();

            largest = largest.max(distance);
        }
    }

    println!("Part 2: {}", largest);
}
