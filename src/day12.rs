use std::collections::HashMap;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let board = &read(filename);
    match args.part {
        1 => part1(board),
        2 => part2(board),
        3 => part3(board),
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

fn part3(board: &[Vec<char>]) -> usize {
    let cost = plots3(board)
        .iter()
        .map(|(size, perimeter)| size * perimeter)
        .sum();
    dbg!(cost);
    cost
}

fn part2(board: &[Vec<char>]) -> usize {
    let cost = plots2(board).iter().map(|(size, sides)| size * sides).sum();
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

fn plots3(board: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut partitions = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited.contains_key(&(i, j)) {
                continue;
            }
            ep!("start ({i},{j}): {}", board[i][j]);
            let value = partition3(board, &mut visited, i, j);
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

fn partition3(
    board: &[Vec<char>],
    visited: &mut HashMap<(usize, usize), usize>,
    i: usize,
    j: usize,
) -> Vec<(usize, usize)> {
    if visited.contains_key(&(i, j)) {
        return vec![];
    }
    // visited.insert((i, j), 0);
    let len = board.len();
    let end = len - 1;
    let tile0 = board[i][j];
    let mut plot = vec![];
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        if visited.contains_key(&(x, y)) {
            continue;
        }
        visited.insert((x, y), 0);

        if board[x][y] != tile0 {
            continue;
        }
        plot.push((x, y));
        if x > 0 {
            if board[x - 1][y] == tile0 {
                if !visited.contains_key(&(x - 1, y)) {
                    stack.push((x - 1, y));
                }
            } else {
                ep!("border ({},{}): {}", x - 1, y, board[x - 1][y]);
                *visited.entry((x, y)).or_insert(0) += 1;
            }
        } else {
            *visited.entry((x, y)).or_insert(0) += 1;
            ep!("border ({},{})", -1, y);
        }
        ep!("{plot:?}");
        if y > 0 {
            if board[x][y - 1] == tile0 {
                if !visited.contains_key(&(x, y - 1)) {
                    stack.push((x, y - 1));
                }
            } else {
                ep!("border ({},{}): {}", x, y - 1, board[x][y - 1]);
                *visited.entry((x, y)).or_insert(0) += 1;
            }
        } else {
            *visited.entry((x, y)).or_insert(0) += 1;
            ep!("border ({},{})", x, -1);
        }
        ep!("{plot:?}");
        if x < end {
            if board[x + 1][y] == tile0 {
                if !visited.contains_key(&(x + 1, y)) {
                    stack.push((x + 1, y));
                }
            } else {
                ep!("border ({},{}): {}", x + 1, y, board[x + 1][y]);
                *visited.entry((x, y)).or_insert(0) += 1;
            }
        } else {
            *visited.entry((x, y)).or_insert(0) += 1;
            ep!("border ({},{})", len, y);
        }
        ep!("{plot:?}");
        if y < end {
            if board[x][y + 1] == tile0 {
                if !visited.contains_key(&(x, y + 1)) {
                    stack.push((x, y + 1));
                }
            } else {
                ep!("border ({},{}): {}", x, y + 1, board[x][y + 1]);
                *visited.entry((x, y)).or_insert(0) += 1;
            }
        } else {
            *visited.entry((x, y)).or_insert(0) += 1;
            ep!("border ({},{})", x, len);
        }
        ep!("{plot:?}");
    }
    plot
}

fn plots2(board: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut partitions = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited.contains_key(&(i, j)) {
                continue;
            }
            ep!("start ({i},{j}): {}", board[i][j]);
            let value = partition2(board, &mut visited, i, j, 0);
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

fn partition2(
    board: &[Vec<char>],
    visited: &mut HashMap<(usize, usize), usize>,
    i: usize,
    j: usize,
    depth: usize,
) -> Vec<(usize, usize)> {
    if visited.contains_key(&(i, j)) {
        return vec![];
    }
    visited.insert((i, j), 0);
    let len = board.len();
    let end = len - 1;
    let tile0 = board[i][j];
    let mut plot = vec![(i, j)];
    ep!("{}:{plot:?}", " ".repeat(depth));
    if i > 0 {
        if board[i - 1][j] == tile0 {
            plot.extend(partition2(board, visited, i - 1, j, depth + 1));
        } else if j > 0 {
            if board[i][j - 1] != tile0 {
                ep!(
                    "{}corner ({},{}): {}",
                    " ".repeat(depth),
                    i - 1,
                    j - 1,
                    board[i - 1][j - 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            } else if board[i - 1][j - 1] == tile0 {
                ep!(
                    "{}inner corner ({},{}): {}",
                    " ".repeat(depth),
                    i - 1,
                    j - 1,
                    board[i - 1][j - 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            }
        } else if j == 0 {
            ep!("{}corner ({},{})", " ".repeat(depth), i - 1, -1);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else if j > 0 {
        if board[i][j - 1] != tile0 {
            ep!("{}corner ({},{})", " ".repeat(depth), -1, j - 1);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else {
        ep!("{}corner ({},{})", " ".repeat(depth), -1, -1);
        *visited.entry((i, j)).or_insert(0) += 1;
    }
    ep!("{}{plot:?}", " ".repeat(depth));
    ep!("{}-{visited:?}", " ".repeat(depth));
    if j > 0 {
        if board[i][j - 1] == tile0 {
            plot.extend(partition2(board, visited, i, j - 1, depth + 1));
        } else if i < end {
            if board[i + 1][j] != tile0 {
                ep!(
                    "{}corner ({},{}): {}",
                    " ".repeat(depth),
                    i + 1,
                    j - 1,
                    board[i + 1][j - 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            } else if board[i + 1][j - 1] == tile0 {
                ep!(
                    "{}inner corner ({},{}): {}",
                    " ".repeat(depth),
                    i + 1,
                    j - 1,
                    board[i + 1][j - 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            }
        } else if i == end {
            ep!("{}corner ({},{})", " ".repeat(depth), len, j - 1);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else if i < end {
        if board[i + 1][j] != tile0 {
            ep!("{}corner ({},{})", " ".repeat(depth), i + 1, -1);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else {
        ep!("{}corner ({},{})", " ".repeat(depth), len, -1);
        *visited.entry((i, j)).or_insert(0) += 1;
    }
    ep!("{}{plot:?}", " ".repeat(depth));
    ep!("{}-{visited:?}", " ".repeat(depth));
    if i < end {
        if board[i + 1][j] == tile0 {
            plot.extend(partition2(board, visited, i + 1, j, depth + 1));
        } else if j < end {
            if board[i][j + 1] != tile0 {
                ep!(
                    "{}corner ({},{}): {}",
                    " ".repeat(depth),
                    i + 1,
                    j + 1,
                    board[i + 1][j + 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            } else if board[i + 1][j + 1] == tile0 {
                ep!(
                    "{}inner corner ({},{}): {}",
                    " ".repeat(depth),
                    i + 1,
                    j + 1,
                    board[i + 1][j + 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            }
        } else if j == end {
            ep!("{}corner ({},{})", " ".repeat(depth), i + 1, len,);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else if j < end {
        if board[i][j + 1] != tile0 {
            ep!("{}corner ({},{})", " ".repeat(depth), len, j + 1);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else {
        ep!("{}corner ({},{})", " ".repeat(depth), len, len);
        *visited.entry((i, j)).or_insert(0) += 1;
    }
    ep!("{}{plot:?}", " ".repeat(depth));
    ep!("{}-{visited:?}", " ".repeat(depth));
    if j < end {
        if board[i][j + 1] == tile0 {
            plot.extend(partition2(board, visited, i, j + 1, depth + 1));
        } else if i > 0 {
            if board[i - 1][j] != tile0 {
                ep!(
                    "{}corner ({},{}): {}",
                    " ".repeat(depth),
                    i - 1,
                    j + 1,
                    board[i - 1][j + 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            } else if board[i - 1][j + 1] == tile0 {
                ep!(
                    "{}inner corner ({},{}): {}",
                    " ".repeat(depth),
                    i - 1,
                    j + 1,
                    board[i - 1][j + 1]
                );
                *visited.entry((i, j)).or_insert(0) += 1;
            }
        } else if i == 0 {
            ep!("{}corner ({},{})", " ".repeat(depth), -1, j + 1);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else if i > 0 {
        if board[i - 1][j] != tile0 {
            ep!("{}corner ({},{})", " ".repeat(depth), i - 1, len);
            *visited.entry((i, j)).or_insert(0) += 1;
        }
    } else {
        ep!("{}corner ({},{})", " ".repeat(depth), -1, len);
        *visited.entry((i, j)).or_insert(0) += 1;
    }
    ep!("{}{plot:?}", " ".repeat(depth));
    ep!("{}-{visited:?}", " ".repeat(depth));

    plot
}

#[cfg(test)]
mod tests {
    use crate::bench0;
    use crate::test;

    test!(p1e1, 12, 1, 1, 140);
    test!(p1e2, 12, 1, 2, 772);
    test!(p1e3, 12, 1, 3, 1930);
    test!(p3e1, 12, 3, 1, 140);
    test!(p3e2, 12, 3, 2, 772);
    test!(p3e3, 12, 3, 3, 1930);
    test!(p2e1, 12, 2, 1, 80);
    test!(p2e2, 12, 2, 2, 436);
    test!(p2e3, 12, 2, 3, 1206);
    test!(p2e4, 12, 2, 4, 236);
    test!(p2e5, 12, 2, 5, 368);

    bench0!(b1e1, part1, 12, 1, 1);
    bench0!(b1e2, part1, 12, 1, 2);
    bench0!(b1e3, part1, 12, 1, 3);
    bench0!(b1i, part1, 12, 1, 0);
    bench0!(b3e1, part3, 12, 3, 1);
    bench0!(b3e2, part3, 12, 3, 2);
    bench0!(b3e3, part3, 12, 3, 3);
    bench0!(b3i, part3, 12, 3, 0);
    bench0!(b2e1, part2, 12, 2, 1);
    bench0!(b2e2, part2, 12, 2, 2);
    bench0!(b2e3, part2, 12, 2, 3);
    bench0!(b2e4, part2, 12, 2, 4);
    bench0!(b2e5, part2, 12, 2, 5);
    // bench0!(b2i, part2, 12, 2, 0);
}
