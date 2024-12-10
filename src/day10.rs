use std::collections::HashSet;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let board = &read(filename);
    match args.part {
        1 => part1(board),
        2 => part2(board),
        _ => todo!(),
    }
}

fn read(filename: String) -> Vec<Vec<usize>> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(board: &[Vec<usize>]) -> usize {
    part(board, true)
}

fn part2(board: &[Vec<usize>]) -> usize {
    part(board, false)
}

fn part(board: &[Vec<usize>], unique: bool) -> usize {
    let len = board.len();
    let count: usize = board
        .iter()
        .flatten()
        .enumerate()
        .filter(|&(_, &x)| x == 0)
        .map(|(i, _)| {
            let x = i / len;
            let y = i % len;
            score(x, y, board, unique)
        })
        .sum();
    dbg!(count);
    count
}

fn score(x: usize, y: usize, board: &[Vec<usize>], unique: bool) -> usize {
    ep!("check: ({x},{y}) -> {}", board[x][y]);
    if unique {
        find_peaks(board, x, y, 0)
            .iter()
            .collect::<HashSet<_>>()
            .len()
    } else {
        find_peaks(board, x, y, 0).len()
    }
}

fn find_peaks(board: &[Vec<usize>], x: usize, y: usize, step: usize) -> Vec<(usize, usize)> {
    let p = board[x][y];
    if p == 9 {
        return vec![(x, y)];
    }
    let len = board.len();
    let end = len - 1;
    let mut positions = Vec::new();
    if x > 0 {
        positions.push((x - 1, y));
    }
    if x < end {
        positions.push((x + 1, y));
    }
    if y > 0 {
        positions.push((x, y - 1));
    }
    if y < end {
        positions.push((x, y + 1));
    }
    let mut peaks = Vec::new();
    for (x, y) in positions.iter() {
        if board[*x][*y] != p + 1 {
            continue;
        }
        ep!("{}check: ({x},{y}) -> {}", " ".repeat(step), board[*x][*y]);
        peaks.extend(find_peaks(board, *x, *y, step + 1));
    }
    peaks
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 10, 1, 1, 36);
    test!(p2, 10, 2, 1, 81);

    bench!(b1e, 10, 1, Some(1));
    bench!(b1i, 10, 1, None);
    bench!(b2e, 10, 2, Some(1));
    bench!(b2i, 10, 2, None);
}
