use std::collections::HashMap;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let board = &read(filename);
    match args.part {
        1 => part1(board),
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
    let cost = plots(board)
        .iter()
        .map(|(size, perimeter)| size * perimeter)
        .sum();
    dbg!(cost);
    cost
}

fn plots(board: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut partitions = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited.contains_key(&(i, j)) {
                continue;
            }
            ep!("start ({i},{j}): {}", board[i][j]);
            let value = partition(board, &mut visited, i, j);
            ep!("{value:?}");
            partitions.push(value);
        }
    }
    partitions
        .iter()
        .map(|partition| {
            (
                partition.len(),
                partition.iter().fold(0, |acc, tile| acc + visited[tile]),
            )
        })
        .collect()
}

fn partition(
    board: &[Vec<char>],
    visited: &mut HashMap<(usize, usize), usize>,
    i: usize,
    j: usize,
) -> Vec<(usize, usize)> {
    if visited.contains_key(&(i, j)) {
        return vec![];
    }
    visited.insert((i, j), 0);
    let len = board.len();
    let end = len - 1;
    let tile0 = board[i][j];
    let mut plot = vec![(i, j)];
    if i > 0 {
        let tile = board[i - 1][j];
        if tile != tile0 {
            ep!("border ({},{}): {}", i - 1, j, tile);
            *visited.entry((i, j)).or_insert(0) += 1;
        } else {
            plot.extend(partition(board, visited, i - 1, j));
        }
    } else {
        *visited.entry((i, j)).or_insert(0) += 1;
        ep!("border ({},{})", -1, j);
    }
    ep!("{plot:?}");
    if i < end {
        let tile = board[i + 1][j];
        if tile != tile0 {
            ep!("border ({},{}): {}", i + 1, j, tile);
            *visited.entry((i, j)).or_insert(0) += 1;
        } else {
            plot.extend(partition(board, visited, i + 1, j));
        }
    } else {
        *visited.entry((i, j)).or_insert(0) += 1;
        ep!("border ({},{})", len, j);
    }
    ep!("{plot:?}");
    if j > 0 {
        let tile = board[i][j - 1];
        if tile != tile0 {
            ep!("border ({},{}): {}", i, j - 1, tile);
            *visited.entry((i, j)).or_insert(0) += 1;
        } else {
            plot.extend(partition(board, visited, i, j - 1));
        }
    } else {
        *visited.entry((i, j)).or_insert(0) += 1;
        ep!("border ({},{})", i, -1);
    }
    ep!("{plot:?}");
    if j < end {
        let tile = board[i][j + 1];
        if tile != tile0 {
            ep!("border ({},{}): {}", i, j + 1, tile);
            *visited.entry((i, j)).or_insert(0) += 1;
        } else {
            plot.extend(partition(board, visited, i, j + 1));
        }
    } else {
        *visited.entry((i, j)).or_insert(0) += 1;
        ep!("border ({},{})", i, len);
    }
    ep!("{plot:?}");

    plot
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1e1, 12, 1, 1, 140);
    test!(p1e2, 12, 1, 2, 772);
    test!(p1e3, 12, 1, 3, 1930);
}
