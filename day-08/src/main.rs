fn main() {
    let input = include_str!("input");

    let mut out = 0;
    for line in input.lines() {
        let note: Vec<&str> = line.split_ascii_whitespace().filter(|&s| s != "|").collect();
        let patterns = &note[..10];
        let outputs = &note[10..];

        for output in outputs {
            match output.len() {
                2 | 4 | 3 | 7 => out += 1,
                _ => {}
            }
        }
    }

    println!("Part 1: {}", out);
}
