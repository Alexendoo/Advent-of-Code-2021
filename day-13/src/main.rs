fn print_row(row: &[bool]) {
    for &t in row {
        if t {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

fn print_board(board: &[Vec<bool>]) {
    for row in board {
        print_row(row);
    }
}

fn fold(board: &mut Vec<Vec<bool>>, line: &str) {
    let (axis, at) = line.rsplit_once('=').unwrap();
    let at: usize = at.parse().unwrap();

    if axis == "fold along y" {
        let (top_rows, bottom_rows) = board.split_at_mut(at);

        for (top_row, bottom_row) in top_rows.iter_mut().rev().zip(&bottom_rows[1..]) {
            for (top, bottom) in top_row.iter_mut().zip(bottom_row) {
                *top |= bottom;
            }
        }

        board.truncate(at);
    } else {
        for row in board {
            let (left, right) = row.split_at_mut(at);

            for (left_dot, right_dot) in left.iter_mut().rev().zip(&right[1..]) {
                *left_dot |= right_dot;
            }

            row.truncate(at);
        }
    }
}

fn main() {
    let (coords, folds) = include_str!("input").split_once("\n\n").unwrap();

    let mut width = 0;
    let mut height = 0;

    let coords: Vec<(usize, usize)> = coords
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            width = width.max(x + 1);
            height = height.max(y + 1);

            (x, y)
        })
        .collect();

    let mut board = vec![vec![false; width]; height];

    for (x, y) in coords {
        board[y][x] = true;
    }

    let mut fold_lines = folds.lines();

    fold(&mut board, fold_lines.next().unwrap());

    println!(
        "Part 1: {}",
        board.iter().flatten().filter(|&&dot| dot).count()
    );

    for line in fold_lines {
        fold(&mut board, line);
    }

    println!("Part 2:");
    print_board(&board);
}
