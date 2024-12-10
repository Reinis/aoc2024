use std::collections::HashMap;

use crate::Args;
use crate::DEBUG;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let board = &read(filename);
    match args.part {
        1 => part1(board),
        2 => part2(board),
        _ => todo!(),
    }
}

fn read(filename: String) -> Vec<Vec<char>> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(board: &[Vec<char>]) -> usize {
    let board1 = &mut board.to_owned();
    let position = find_guard(board);
    walk_free(position, board1);
    let count = board1
        .iter()
        .map(|row| row.iter().filter(|&x| *x == 'X').count())
        .sum();
    dbg!(count);
    count
}

fn walk_free(position: (usize, usize), board: &mut [Vec<char>]) {
    let mut position = position;
    while on_board(position, board) {
        position = advance(position, board);
        print_board(board);
    }
}

fn print_board(board: &[Vec<char>]) {
    if *DEBUG {
        let (x, y) = find_guard(board);
        eprintln!("({x},{y})");
        for row in board {
            eprintln!("{}", row.iter().collect::<String>())
        }
    }
}

fn advance(position: (usize, usize), board: &mut [Vec<char>]) -> (usize, usize) {
    let (x, y) = position;
    let len = board.len();
    let guard = board[x][y];
    let barrier_chars = ['#', 'O'];
    match guard {
        '^' => {
            if x == 0 {
                board[x][y] = 'X';
                position
            } else if barrier_chars.contains(&board[x - 1][y]) {
                board[x][y] = '>';
                position
            } else {
                board[x][y] = 'X';
                board[x - 1][y] = '^';
                (x - 1, y)
            }
        }
        '>' => {
            if y == len - 1 {
                board[x][y] = 'X';
                position
            } else if barrier_chars.contains(&board[x][y + 1]) {
                board[x][y] = 'v';
                position
            } else {
                board[x][y] = 'X';
                board[x][y + 1] = '>';
                (x, y + 1)
            }
        }
        'v' => {
            if x == len - 1 {
                board[x][y] = 'X';
                position
            } else if barrier_chars.contains(&board[x + 1][y]) {
                board[x][y] = '<';
                position
            } else {
                board[x][y] = 'X';
                board[x + 1][y] = 'v';
                (x + 1, y)
            }
        }
        '<' => {
            if y == 0 {
                board[x][y] = 'X';
                position
            } else if barrier_chars.contains(&board[x][y - 1]) {
                board[x][y] = '^';
                position
            } else {
                board[x][y] = 'X';
                board[x][y - 1] = '<';
                (x, y - 1)
            }
        }
        _ => todo!("Unexpected guard at ({x},{y}): {:?}", guard),
    }
}

fn on_board(position: (usize, usize), board: &[Vec<char>]) -> bool {
    let (x, y) = position;
    board[x][y] != 'X'
}

fn find_guard(board: &[Vec<char>]) -> (usize, usize) {
    let guard_chars = ['^', '>', 'v', '<'];

    for (x, row) in board.iter().enumerate() {
        if let Some(y) = row.iter().position(|x| guard_chars.contains(x)) {
            return (x, y);
        }
    }
    (0, 0)
}

fn part2(board: &[Vec<char>]) -> usize {
    let board0 = &mut board.to_owned();
    let position = find_guard(board0);
    walk_free(position, board0);
    let (x0, y0) = position;
    board0[x0][y0] = '^';
    let board1 = &mut board.to_owned();
    let mut count = 0;
    let mut i = 0;

    for (x, row) in board0.iter().enumerate() {
        for (y, &tile) in row.iter().enumerate() {
            if tile != 'X' {
                continue;
            }
            i += 1;
            eprint!("\r{i}");
            board1[x][y] = 'O';
            if is_loop(board1) {
                count += 1;
            }
            board1[x][y] = 'X';
            board1[x0][y0] = '^';
        }
    }
    dbg!(count);
    count
}

fn is_loop(board: &mut [Vec<char>]) -> bool {
    let mut position = find_guard(board);
    let (x, y) = position;
    let mut guard = board[x][y];
    let mut visited: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    visited.insert(position, vec![guard]);

    loop {
        position = advance(position, board);
        if !on_board(position, board) {
            return false;
        }
        let (x, y) = position;
        guard = board[x][y];
        if let Some(v) = visited.get_mut(&(x, y)) {
            if v.contains(&guard) {
                return true;
            }
            v.push(guard);
        } else {
            visited.insert(position, vec![guard]);
        }
        print_board(board);
    }
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 6, 1, 1, 41);
    test!(p2, 6, 2, 1, 6);

    bench!(b1e, 6, 1, Some(1));
    bench!(b1i, 6, 1, None);
    bench!(b2e, 6, 2, Some(1));
    bench!(b2i, 6, 2, None);
}
