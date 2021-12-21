use derive_more::{Add, Sum};
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Mul;
use Turn::{Player1, Player2};

fn wrap(val: u64, at: u64) -> u64 {
    ((val - 1) % at) + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    fn new(pos: u64) -> Self {
        Self { pos, score: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Player1,
    Player2,
}

impl Turn {
    fn next(self) -> Self {
        match self {
            Player1 => Player2,
            Player2 => Player1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Game {
    p1: Player,
    p2: Player,

    turn: Turn,
}

impl Game {
    fn advance(mut self, roll: u64) -> Self {
        self.turn = self.turn.next();
        let player = match self.turn {
            Player1 => &mut self.p1,
            Player2 => &mut self.p2,
        };

        player.pos = wrap(player.pos + roll, 10);
        player.score += player.pos;

        self
    }
}

#[derive(Debug, Clone, Copy, Default, Add, Sum)]
struct Wins {
    p1: u64,
    p2: u64,
}

impl Mul<Wins> for u64 {
    type Output = Wins;

    fn mul(self, wins: Wins) -> Wins {
        Wins {
            p1: wins.p1 * self,
            p2: wins.p2 * self,
        }
    }
}

fn deterministic(mut game: Game) -> u64 {
    let mut die = 1;
    let mut rolls = 0;

    loop {
        let mut roll = 0;
        for _ in 0..3 {
            rolls += 1;
            roll += die;
            die = wrap(die + 1, 100);
        }
        game = game.advance(roll);

        let (score, loser) = match game.turn {
            Player1 => (game.p1.score, game.p2),
            Player2 => (game.p2.score, game.p1),
        };

        if score >= 1000 {
            return loser.score * rolls;
        }
    }
}

const BRANCHES: [(u64, u64); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn dirac(initial: Game) -> u64 {
    fn recurse(game: Game, memo: &mut HashMap<Game, Wins>) -> Wins {
        if let Some(&wins) = memo.get(&game) {
            return wins;
        }

        let wins = match game.turn {
            Player1 if game.p1.score >= 21 => Wins { p1: 1, p2: 0 },
            Player2 if game.p2.score >= 21 => Wins { p1: 0, p2: 1 },
            _ => BRANCHES
                .into_iter()
                .map(|(roll, count)| count * recurse(game.advance(roll), memo))
                .sum(),
        };

        memo.insert(game, wins);

        wins
    }

    let mut memo = HashMap::default();
    let wins = recurse(initial, &mut memo);

    max(wins.p1, wins.p2)
}

fn main() {
    let input = Game {
        p1: Player::new(3),
        p2: Player::new(10),
        turn: Player2,
    };

    println!("Part 1: {}", deterministic(input));
    println!("Part 2: {}", dirac(input));
}
