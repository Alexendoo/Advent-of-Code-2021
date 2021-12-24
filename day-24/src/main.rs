use itertools::Itertools;

//   inp w    | inp w    | inp w    | inp w     | inp w    |       | inp w
//   mul x 0  | mul x 0  | mul x 0  | mul x 0   | mul x 0  |       | mul x 0
//   add x z  | add x z  | add x z  | add x z   | add x z  |       | add x z
//   mod x 26 | mod x 26 | mod x 26 | mod x 26  | mod x 26 |       | mod x 26
// p div z 1  | div z 1  | div z 1  | div z 26  | div z 1  |       | div z 26
// q add x 10 | add x 14 | add x 14 | add x -13 | add x 10 |       | add x -9
//   eql x w  | eql x w  | eql x w  | eql x w   | eql x w  |       | eql x w
//   eql x 0  | eql x 0  | eql x 0  | eql x 0   | eql x 0  |       | eql x 0
//   mul y 0  | mul y 0  | mul y 0  | mul y 0   | mul y 0  |  ...  | mul y 0
//   add y 25 | add y 25 | add y 25 | add y 25  | add y 25 |       | add y 25
//   mul y x  | mul y x  | mul y x  | mul y x   | mul y x  |       | mul y x
//   add y 1  | add y 1  | add y 1  | add y 1   | add y 1  |       | add y 1
//   mul z y  | mul z y  | mul z y  | mul z y   | mul z y  |       | mul z y
//   mul y 0  | mul y 0  | mul y 0  | mul y 0   | mul y 0  |       | mul y 0
//   add y w  | add y w  | add y w  | add y w   | add y w  |       | add y w
// r add y 2  | add y 13 | add y 13 | add y 9   | add y 15 |       | add y 9
//   mul y x  | mul y x  | mul y x  | mul y x   | mul y x  |       | mul y x
//   add z y  | add z y  | add z y  | add z y   | add z y  |       | add z y

// d00,     p: 1,   q: 10,  r: 2
// d01,     p: 1,   q: 14,  r: 13
// d02,     p: 1,   q: 14,  r: 13
// d03,     p: 26,  q: -13, r: 9
// d04,     p: 1,   q: 10,  r: 15
// d05,     p: 26,  q: -13, r: 3
// d06,     p: 26,  q: -7,  r: 6
// d07,     p: 1,   q: 11,  r: 5
// d08,     p: 1,   q: 10,  r: 16
// d09,     p: 1,   q: 13,  r: 1
// d10,     p: 26,  q: -4,  r: 6
// d11,     p: 26,  q: -9,  r: 3
// d12,     p: 26,  q: -13, r: 7
// d13,     p: 26,  q: -9,  r: 9

// d02,     p: 1,   q: 14,  r: 13 | d03,     p: 26,  q: -13, r: 9
// d04,     p: 1,   q: 10,  r: 15 | d05,     p: 26,  q: -13, r: 3
// d01,     p: 1,   q: 14,  r: 13 | d06,     p: 26,  q: -7,  r: 6
// d09,     p: 1,   q: 13,  r: 1  | d10,     p: 26,  q: -4,  r: 6
// d08,     p: 1,   q: 10,  r: 16 | d11,     p: 26,  q: -9,  r: 3
// d07,     p: 1,   q: 11,  r: 5  | d12,     p: 26,  q: -13, r: 7
// d00,     p: 1,   q: 10,  r: 2  | d13,     p: 26,  q: -9,  r: 9

// d03 = d02 + 13 - 13  = d02
// d05 = d04 + 15 - 13  = d04 + 2
// d06 = d01 + 13 - 7   = d01 + 6
// d10 = d09 + 1  - 4   = d09 - 3
// d11 = d08 + 16 - 9   = d08 + 7
// d12 = d07 + 5  - 13  = d07 - 8
// d13 = d00 + 2  - 9   = d00 - 7

#[derive(Clone, Copy, Debug)]
struct Block {
    i: usize,
    p: i32,
    q: i32,
    r: i32,
}

fn main() {
    let blocks = include_str!("input")
        .split("inp w")
        .filter(|&block| block != "")
        .enumerate()
        .map(|(i, block)| {
            let mut lines = block.lines();
            let mut val = |n| lines.nth(n).unwrap()[6..].parse().unwrap();

            Block {
                i,
                p: val(4),
                q: val(0),
                r: val(9),
            }
        })
        .collect_vec();

    let mut hi = [0; 14];
    let mut lo = [0; 14];

    let mut stack = Vec::new();
    for (i, block) in blocks.iter().enumerate() {
        if block.p == 1 {
            stack.push(block);
        } else {
            let popped = stack.pop().unwrap();
            let diff = block.q + popped.r;

            hi[block.i] = 9 + diff.min(0);
            hi[popped.i] = 9 - diff.max(0);

            lo[block.i] = 1 + diff.max(0);
            lo[popped.i] = 1 - diff.min(0);
        }
    }

    println!("{}", hi.map(|digit| digit.to_string()).join(""));
    println!("{}", lo.map(|digit| digit.to_string()).join(""));
}
