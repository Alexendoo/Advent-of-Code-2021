type Timers = [usize; 9];
fn tick(timers: &mut Timers) {
    let zeros = timers[0];
    timers.copy_within(1.., 0);
    timers[6] += zeros;
    timers[8] = zeros;
}

fn main() {
    let input = include_str!("input");

    let numbers: Vec<usize> = input.trim().split(',').map(|n| n.parse().unwrap()).collect();

    let mut timers: Timers = Default::default();
    for timer in numbers {
        timers[timer] += 1;
    }

    for _ in 0..80 {
        tick(&mut timers);
    }
    println!("Part 1: {}", timers.iter().sum::<usize>());
}
