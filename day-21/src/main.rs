use std::mem::swap;

fn wrap(val: usize, at: usize) -> usize {
    ((val - 1) % at) + 1
}

#[derive(Debug)]
struct Die {
    current: usize,
    rolled: usize,
}

impl Die {
    fn roll(&mut self) {
        self.current = wrap(self.current + 1, 100);
        self.rolled += 1;
    }
}

#[derive(Debug)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Self { pos, score: 0 }
    }

    fn turn(&mut self, die: &mut Die) {
        for _ in 0..3 {
            self.pos = wrap(self.pos + die.current, 10);
            die.roll();
        }
        self.score += self.pos;
    }
}

fn main() {
    let mut player1 = Player::new(3);
    let mut player2 = Player::new(10);

    let mut die = Die {
        current: 1,
        rolled: 0,
    };

    let mut current = &mut player1;
    let mut next = &mut player2;
    loop {
        current.turn(&mut die);

        if current.score >= 1000 {
            break;
        }

        swap(&mut current, &mut next);
    }

    println!("Part 1: {}",  next.score * die.rolled);
}
