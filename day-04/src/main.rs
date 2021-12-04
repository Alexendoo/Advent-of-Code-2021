fn bingo(board: &[usize; 25], draws: &[usize]) -> bool {
    for col in 0..5 {
        for row in 0..5 {
            let number = board[row * 5 + col];
            if !draws.contains(&number) {
                break;
            }
            if row == 4 {
                return true;
            }
        }
    }

    for row in 0..5 {
        for col in 0..5 {
            let number = board[row * 5 + col];
            if !draws.contains(&number) {
                break;
            }
            if col == 4 {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = include_str!("input");

    let mut tokens = input.split_whitespace();

    let draws: Vec<usize> = tokens
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut numbers = tokens.map(|n| n.parse().unwrap());

    let mut boards = Vec::new();
    'outer: loop {
        let mut board = [0; 25];
        for i in 0..25 {
            match numbers.next() {
                Some(n) => board[i] = n,
                None => break 'outer,
            };
        }
        boards.push(board);
    }

    for len in 0.. {
        let draws = &draws[..len];
        if let Some(board) = boards.iter().find(|board| bingo(board, draws)) {
            let sum: usize = board.iter().filter(|n| !draws.contains(n)).sum();
            let last_called = draws.last().unwrap();

            println!("Part 1: {}", sum * last_called);
            break;
        }
    }
}
