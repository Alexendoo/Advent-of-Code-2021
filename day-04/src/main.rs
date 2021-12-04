type Board = [usize; 25];

fn bingo(board: &Board, draws: &[usize]) -> bool {
    let test = |f: &dyn Fn(usize, usize) -> usize| {
        (0..5).any(|i| {
            (0..5).all(|j| {
                draws.contains(&board[f(i, j)])
            })
        })
    };

    test(&|i, j| i * 5 + j) || test(&|i, j| j * 5 + i)
}

fn winner<'a>(boards: &'a [Board], draws: &[usize], negate: bool) -> Option<&'a Board> {
    boards.iter().find(|board| bingo(board, draws) ^ negate)
}

fn score(board: &Board, draws: &[usize]) -> usize {
    let sum: usize = board.iter().filter(|n| !draws.contains(n)).sum();
    let last_called = draws.last().unwrap();

    sum * last_called
}

fn main() {
    let input = include_str!("input");

    let mut tokens = input.split_ascii_whitespace();

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
        for tile in &mut board {
            match numbers.next() {
                Some(n) => *tile = n,
                None => break 'outer,
            };
        }
        boards.push(board);
    }

    for len in 0.. {
        if let Some(board) = winner(&boards, &draws[..len], false) {
            println!("Part 1: {}", score(board, &draws[..len]));
            break;
        }
    }

    for len in (0..draws.len()).rev() {
        if let Some(board) = winner(&boards, &draws[..len], true) {
            println!("Part 2: {}", score(board, &draws[..=len]));
            break;
        }
    }
}
