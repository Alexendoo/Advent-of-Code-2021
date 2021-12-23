use std::cmp::Ordering::{Greater, Less};
use std::collections::HashSet;
use std::{fmt, mem};
use Frog::{A, B, C, D};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

const TARGETS: [Pos; 15] = [
    Pos { row: 0, col: 0 },
    Pos { row: 0, col: 1 },
    Pos { row: 0, col: 3 },
    Pos { row: 0, col: 5 },
    Pos { row: 0, col: 7 },
    Pos { row: 0, col: 9 },
    Pos { row: 0, col: 10 },
    Pos { row: 1, col: 2 },
    Pos { row: 1, col: 4 },
    Pos { row: 1, col: 6 },
    Pos { row: 1, col: 8 },
    Pos { row: 2, col: 2 },
    Pos { row: 2, col: 4 },
    Pos { row: 2, col: 6 },
    Pos { row: 2, col: 8 },
];

const DESTINATIONS: [Pos; 8] = [
    Pos { row: 1, col: 2 },
    Pos { row: 1, col: 4 },
    Pos { row: 1, col: 6 },
    Pos { row: 1, col: 8 },
    Pos { row: 2, col: 2 },
    Pos { row: 2, col: 4 },
    Pos { row: 2, col: 6 },
    Pos { row: 2, col: 8 },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Frog {
    A,
    B,
    C,
    D,
}

impl Frog {
    fn cost(self) -> usize {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn destination_col(self) -> usize {
        match self {
            A => 2,
            B => 4,
            C => 6,
            D => 8,
        }
    }

    fn destination(self) -> [Pos; 2] {
        let col = self.destination_col();

        [Pos { row: 1, col }, Pos { row: 2, col }]
    }

    fn is_destination(self, pos: Pos) -> bool {
        let col = self.destination_col();

        pos.col == col && (pos.row == 1 || pos.row == 2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct Burrow {
    board: [[Option<Frog>; 11]; 3],
    energy: usize,
}

impl Burrow {
    fn new(s: &str) -> Burrow {
        let mut burrow = Burrow::default();

        let inputs = s.chars().filter_map(|ch| match ch {
            'A' => Some(A),
            'B' => Some(B),
            'C' => Some(C),
            'D' => Some(D),
            _ => None,
        });

        for (frog, &pos) in inputs.zip(&TARGETS[7..]) {
            burrow.board[pos.row][pos.col] = Some(frog);
        }

        burrow
    }

    fn get(self, pos: Pos) -> Option<Frog> {
        self.board[pos.row][pos.col]
    }

    fn occupied(self, pos: Pos) -> bool {
        self.get(pos).is_some()
    }

    fn tick(mut self, source: Pos, target: Pos) -> Option<Burrow> {
        let frog = self.get(source)?;

        // Once an amphipod stops moving in the hallway, it will stay in that
        // spot until it can move into a room
        if source.row == 0 && target.row == 0 {
            return None;
        }

        if target.row > 0 {
            // Amphipods will never move from the hallway into a room unless
            // that room is their destination room and...
            if !frog.is_destination(target) {
                return None;
            }

            for dest in frog.destination() {
                if let Some(dest_frog) = self.get(dest) {
                    // ...that room contains no amphipods which do not also have
                    // that room as their own destination
                    if !dest_frog.is_destination(dest) {
                        return None;
                    }
                }
            }

            // Should move into the lower space if it's free
            if target.row == 1
                && !self.occupied(Pos {
                    row: 2,
                    col: target.col,
                })
            {
                return None;
            }
        }

        if frog.is_destination(source) {
            // In end spot
            if source.row == 2 {
                return None;
            }

            // Both in final position
            if frog.destination().map(|pos| self.get(pos)) == [Some(frog), Some(frog)] {
                return None;
            }
        }

        let mut current = source;

        macro_rules! walk {
            ($rowcol:ident, $val:expr) => {{
                while let diff @ (Greater | Less) = current.$rowcol.cmp(&$val) {
                    current.$rowcol = current.$rowcol.wrapping_sub(diff as usize);

                    if self.occupied(current) {
                        return None;
                    }

                    self.energy += frog.cost();
                }
            }};
        }

        walk!(row, 0);
        walk!(col, target.col);
        walk!(row, target.row);

        self.board[target.row][target.col] = mem::take(&mut self.board[source.row][source.col]);

        Some(self)
    }

    fn is_finished(self) -> bool {
        for dest in DESTINATIONS {
            match self.get(dest) {
                Some(frog) => {
                    if !frog.is_destination(dest) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }

    fn solve(self, cost: &mut usize, seen: &mut HashSet<Burrow>) {
        if self.energy >= *cost {
            return;
        }

        if !seen.insert(self) {
            return;
        }

        if self.is_finished() {
            println!("{}", cost);
            *cost = self.energy.min(*cost);
        }

        for source in TARGETS {
            for target in TARGETS {
                if let Some(next) = self.tick(source, target) {
                    next.solve(cost, seen);
                }
            }
        }
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board {
            for tile in row {
                if let Some(frog) = tile {
                    write!(f, "{:?}", frog)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let burrow = Burrow::new(include_str!("input"));

    let mut min = usize::MAX;
    let mut seen = HashSet::new();

    burrow.solve(&mut min, &mut seen);
    println!("Part 1: {}", min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick() {
        let mut burrow = Burrow::new(
            "
            #############
            #...........#
            ###B#C#B#D###
              #A#D#C#A#
              #########",
        );

        let mut next = |sr, sc, dr, dc| {
            burrow = burrow
                .tick(Pos { row: sr, col: sc }, Pos { row: dr, col: dc })
                .unwrap();
            println!("{}", burrow);
        };

        next(1, 6, 0, 3);
        next(1, 4, 1, 6);
        next(2, 4, 0, 5);
        next(0, 3, 2, 4);
        next(1, 2, 1, 4);
        next(1, 8, 0, 7);
        next(2, 8, 0, 9);
        next(0, 7, 2, 8);
        next(0, 5, 1, 8);
        next(0, 9, 1, 2);

        assert_eq!(burrow.energy, 12521);
    }
}
