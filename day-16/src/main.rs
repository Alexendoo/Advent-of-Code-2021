use std::cmp::Ordering;
use PacketKind::{Operator, Value};

fn advance<'a>(cursor: &mut &'a str, bits: usize) -> &'a str {
    let (l, rest) = cursor.split_at(bits);
    *cursor = rest;
    l
}

fn number(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

#[derive(Debug)]
enum PacketKind {
    Value(usize),
    Operator(usize, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    kind: PacketKind,
}

fn parse(cursor: &mut &str) -> Packet {
    let version = number(advance(cursor, 3));
    let type_id = number(advance(cursor, 3));

    match type_id {
        4 => {
            let mut value = String::new();

            loop {
                let continuation = advance(cursor, 1);
                value.push_str(advance(cursor, 4));

                if continuation == "0" {
                    break;
                }
            }

            Packet {
                version,
                kind: Value(number(&value)),
            }
        }
        op => {
            let length_type = advance(cursor, 1);
            let mut operands = Vec::new();

            if length_type == "0" {
                let bit_length = number(advance(cursor, 15));

                let mut sub_cursor = advance(cursor, bit_length);

                while !sub_cursor.is_empty() {
                    operands.push(parse(&mut sub_cursor));
                }
            } else {
                let num_packets = number(advance(cursor, 11));

                for _ in 0..num_packets {
                    operands.push(parse(cursor));
                }
            }

            Packet {
                version,
                kind: Operator(op, operands),
            }
        }
    }
}

fn version_sum(packet: &Packet) -> usize {
    packet.version
        + match &packet.kind {
            Operator(_, ops) => ops.iter().map(version_sum).sum(),
            Value(_) => 0,
        }
}

fn operate(packet: &Packet) -> usize {
    fn iter(ops: &[Packet]) -> impl Iterator<Item = usize> + '_ {
        ops.iter().map(operate)
    }
    fn compare(ops: &[Packet], ordering: Ordering) -> usize {
        if Ord::cmp(&operate(&ops[0]), &operate(&ops[1])) == ordering {
            1
        } else {
            0
        }
    }

    match &packet.kind {
        Value(value) => *value,
        Operator(0, ops) => iter(ops).sum(),
        Operator(1, ops) => iter(ops).product(),
        Operator(2, ops) => iter(ops).min().unwrap(),
        Operator(3, ops) => iter(ops).max().unwrap(),
        Operator(5, ops) => compare(ops, Ordering::Greater),
        Operator(6, ops) => compare(ops, Ordering::Less),
        Operator(7, ops) => compare(ops, Ordering::Equal),
        Operator(..) => unreachable!(),
    }
}

fn main() {
    let input = include_str!("input");

    let packet: String = input
        .trim()
        .chars()
        .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
        .collect();

    let packets = parse(&mut &packet[..]);

    println!("Part 1: {}", version_sum(&packets));
    println!("Part 2: {}", operate(&packets));
}
