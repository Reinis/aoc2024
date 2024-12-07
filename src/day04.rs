use crate::Args;
use crate::DEBUG;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
        2 => part2(filename),
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
        .map(|x| x.chars().collect())
        .collect()
}

fn part1(filename: String) -> usize {
    let word: Vec<char> = "XMAS".chars().collect();
    let board = read(filename);
    let len = board.len();
    assert!(len == board[0].len());
    // dbg!(&map, &map[0], &map[0][0]);
    let mut count = board
        .iter()
        .map(count_words(&word))
        // .inspect(|x| eprintln!("{x}"))
        .sum();
    count += transpose(&board)
        .iter()
        .map(count_words(&word))
        .sum::<usize>();
    count += diag(&board, map_diag1)
        .iter()
        .map(count_words(&word))
        .sum::<usize>();
    count += diag(&board, map_diag2)
        .iter()
        .map(count_words(&word))
        .sum::<usize>();
    dbg!(count);
    count
}

fn map_diag1(pair: (i16, i16), _len: i16) -> (usize, usize) {
    (pair.0 as usize, pair.1 as usize)
}

fn map_diag2(pair: (i16, i16), len: i16) -> (usize, usize) {
    (pair.0 as usize, (len - pair.1 - 1) as usize)
}

fn diag(
    board: &[Vec<char>],
    map_diag: impl Fn((i16, i16), i16) -> (usize, usize),
) -> Vec<Vec<char>> {
    let len = board.len() as i16;
    (4 - len..len - 3)
        .fold((Vec::new(), false), |(mut acc, mut swap), diag: i16| {
            let line: Vec<char> = (diag.abs()..len)
                .zip(0..len)
                .map(|(row, col)| if swap { (col, row) } else { (row, col) })
                .map(|x| map_diag(x, len))
                .inspect(|(row, col)| {
                    if *DEBUG {
                        eprintln!("({row},{col})")
                    }
                })
                .map(|(row, col)| board[row][col])
                .collect();
            if *DEBUG {
                eprintln!("{}", line.iter().collect::<String>());
            }
            acc.push(line);
            if diag == 0 {
                swap = true;
            }
            (acc, swap)
        })
        .0
}

fn transpose(board: &[Vec<char>]) -> Vec<Vec<char>> {
    let len = board.len();
    (0..len)
        .map(|col| (0..len).map(|row| board[row][col]).collect())
        .collect()
}

fn count_words(word: &[char]) -> impl FnMut(&Vec<char>) -> usize {
    |line| {
        line.iter()
            .fold((0, 0, 3), |(mut count, mut i, mut j), &letter| {
                // dbg!(letter, i, word[i] == letter);
                if word[i] == letter {
                    if i == 3 {
                        count += 1;
                        i = 0;
                    } else {
                        i += 1;
                    }
                } else {
                    i = 0;
                    if word[i] == letter {
                        i += 1;
                    }
                }
                if word[j] == letter {
                    if j == 0 {
                        count += 1;
                        j = 3;
                    } else {
                        j -= 1;
                    }
                } else {
                    j = 3;
                    if word[j] == letter {
                        j -= 1;
                    }
                }
                (count, i, j)
            })
            .0
    }
}

fn part2(filename: String) -> usize {
    let board = read(filename);
    let len = board.len();
    let count = (1..len - 1)
        .map(|row| {
            (1..len - 1)
                .map(|col| {
                    if board[row][col] != 'A' {
                        return 0;
                    }
                    let a = board[row - 1][col - 1];
                    let b = board[row + 1][col + 1];
                    let c = board[row - 1][col + 1];
                    let d = board[row + 1][col - 1];
                    let list = [a, b, c, d];
                    if list.iter().filter(|x| **x == 'M').count() != 2
                        || list.iter().filter(|x| **x == 'S').count() != 2
                    {
                        return 0;
                    }
                    if a == b || c == d {
                        return 0;
                    }
                    if *DEBUG {
                        eprintln!("({row},{col})");
                    }
                    1
                })
                .sum::<usize>()
        })
        .sum();
    dbg!(count);
    count
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 4, 1, 1, 18);
    test!(p2, 4, 2, 1, 9);
}
