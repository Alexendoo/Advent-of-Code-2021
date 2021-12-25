type Board = Vec<Vec<char>>;

fn print(board: &Board) {
    for row in board {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn step(board: &mut Board) -> bool {
    let width = board[0].len();
    let height = board.len();

    let prev = board.clone();
    for row in 0..height {
        for col in 0..width {
            if prev[row][col] == '>' && prev[row][(col + 1) % width] == '.' {
                board[row][col] = '.';
                board[row][(col + 1) % width] = '>';
            }
        }
    }

    let moved_east = board.clone();
    for row in 0..height {
        for col in 0..width {
            if moved_east[row][col] == 'v' && moved_east[(row + 1) % height][col] == '.' {
                board[row][col] = '.';
                board[(row + 1) % height][col] =  'v';
            }
        }
    }

    prev == *board
}

fn main() {
    let input = include_str!("input");

    let mut board: Board = input.lines().map(|line| line.chars().collect()).collect();

    let mut steps = 1;
    while !step(&mut board) {
        steps += 1;
    }

    println!("🎄 {}", steps)
}
