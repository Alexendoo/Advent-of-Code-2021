use rustc_hash::FxHashMap;
use std::cmp::Ordering::{Greater, Less};
use std::collections::hash_map::Entry;
use std::{fmt, mem};
use Frog::{A, B, C, D};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

const HALLWAY: [Pos; 7] = [
    Pos { row: 0, col: 0 },
    Pos { row: 0, col: 1 },
    Pos { row: 0, col: 3 },
    Pos { row: 0, col: 5 },
    Pos { row: 0, col: 7 },
    Pos { row: 0, col: 9 },
    Pos { row: 0, col: 10 },
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

    fn destinations(self, height: usize) -> impl Iterator<Item = Pos> + DoubleEndedIterator {
        let col = self.destination_col();

        (1..height).map(move |row| Pos { row, col })
    }

    fn is_destination(self, pos: Pos) -> bool {
        let col = self.destination_col();

        pos.col == col && pos.row > 0
    }
}

type Board<const H: usize> = [[Option<Frog>; 11]; H];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Burrow<const H: usize> {
    board: Board<H>,
    energy: usize,
}

impl<const H: usize> Burrow<H> {
    fn room_positions(self) -> impl Iterator<Item = Pos> + Clone {
        (1..H).flat_map(|row| [2, 4, 6, 8].map(|col| Pos { row, col }))
    }

    fn new(s: &str) -> Self {
        let mut burrow = Burrow {
            board: [[None; 11]; H],
            energy: 0,
        };

        let inputs = s.chars().filter_map(|ch| match ch {
            'A' => Some(A),
            'B' => Some(B),
            'C' => Some(C),
            'D' => Some(D),
            _ => None,
        });

        for (frog, pos) in inputs.zip(burrow.room_positions()) {
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

    fn deepest(self, frog: Frog) -> Option<Pos> {
        frog.destinations(H)
            .rfind(|&pos| self.get(pos) != Some(frog))
    }

    fn tick(mut self, source: Pos, target: Pos) -> Option<Self> {
        let frog = self.get(source)?;

        // Once an amphipod stops moving in the hallway, it will stay in that
        // spot until it can move into a room
        if source.row == 0 && target.row == 0 {
            return None;
        }

        if frog.is_destination(source) {
            match self.deepest(frog) {
                Some(pos) => {
                    if source.row > pos.row {
                        return None;
                    }
                }
                // All in final position
                None => return None,
            }
        }

        if target.row > 0 {
            // Amphipods will never move from the hallway into a room unless
            // that room is their destination room and...
            if !frog.is_destination(target) {
                return None;
            }

            for dest in frog.destinations(H) {
                if let Some(dest_frog) = self.get(dest) {
                    // ...that room contains no amphipods which do not also have
                    // that room as their own destination
                    if !dest_frog.is_destination(dest) {
                        return None;
                    }
                }
            }

            // Should move into the lower space if it's free
            if target != self.deepest(frog).unwrap() {
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
        for dest in self.room_positions() {
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

    fn solve(self, cost: &mut usize, seen: &mut FxHashMap<Board<H>, usize>) {
        if self.energy >= *cost {
            return;
        }

        match seen.entry(self.board) {
            Entry::Occupied(mut e) => {
                if self.energy >= *e.get() {
                    return;
                }

                e.insert(self.energy);
            }
            Entry::Vacant(e) => {
                e.insert(self.energy);
            }
        }

        if self.is_finished() {
            *cost = self.energy.min(*cost);
        }

        let targets = self.room_positions().chain(HALLWAY);

        for source in targets.clone() {
            for target in targets.clone() {
                if let Some(next) = self.tick(source, target) {
                    next.solve(cost, seen);
                }
            }
        }
    }
}

impl<const H: usize> fmt::Display for Burrow<H> {
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
    let input = include_str!("input");

    let burrow = Burrow::<3>::new(input);

    let mut min = usize::MAX;
    let mut seen = Default::default();

    burrow.solve(&mut min, &mut seen);
    println!("Part 1: {}", min);

    let extended = format!(
        "{}
            #D#C#B#A#
            #D#B#A#C#
        {}",
        &input[..41],
        &input[42..]
    );

    let mut min = usize::MAX;
    let mut seen = Default::default();

    let extended_burrow = Burrow::<5>::new(&extended);
    extended_burrow.solve(&mut min, &mut seen);

    println!("Part 2: {}", min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick() {
        let mut burrow = Burrow::<3>::new(
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
