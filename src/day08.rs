use std::collections::HashMap;
use std::collections::HashSet;

use crate::Args;
use crate::DEBUG;
use crate::ep;

type Point = (i64, i64);
type Pair = (Point, Point);

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
    let len = board.len();
    let count = antinodes(board, |pair| nodes(pair, len)).len();
    dbg!(count);
    count
}

fn part2(board: &[Vec<char>]) -> usize {
    let len = board.len();
    let count = antinodes(board, |pair| nodes2(pair, len)).len();
    dbg!(count);
    count
}

fn antinodes(board: &[Vec<char>], get_nodes: impl Fn(&Pair) -> Vec<Point>) -> HashSet<Point> {
    let antenas = antenas(board);
    let mut antinodes = HashSet::new();

    for (kind, coords) in antenas.iter() {
        let mut pairs = Vec::new();
        for &c1 in coords.iter() {
            for &c2 in coords.iter() {
                if c1 == c2 {
                    continue;
                }
                let pair = mk_pair(c1, c2);
                if pairs.contains(&pair) {
                    continue;
                }
                pairs.push(pair);
                antinodes.extend(get_nodes(&pair));
            }
        }
        ep!("{kind}: {pairs:?}");
    }
    print_board(board, &antinodes);
    antinodes
}

fn print_board(board: &[Vec<char>], antinodes: &HashSet<Point>) {
    if !*DEBUG {
        return;
    }
    let board1 = &mut board.to_owned();
    for (x, y) in antinodes.iter() {
        board1[*x as usize][*y as usize] = '#';
    }
    for row in board1.iter() {
        eprintln!("{}", row.iter().collect::<String>());
    }
}

fn on_board(point: Point, len: usize) -> bool {
    let (x, y) = point;
    x >= 0 && x < len as i64 && y >= 0 && y < len as i64
}

fn mk_pair(c1: Point, c2: Point) -> Pair {
    if c1 < c2 { (c1, c2) } else { (c2, c1) }
}

fn nodes(pair: &Pair, len: usize) -> Vec<Point> {
    let ((x1, y1), (x2, y2)) = pair;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let a1 = (x1 - dx, y1 - dy);
    let a2 = (x2 + dx, y2 + dy);
    let mut nodes = vec![a1, a2];
    nodes.retain(|&node| on_board(node, len));
    for node in nodes.iter() {
        ep!("o -> ({},{})", node.0, node.1);
    }
    nodes
}

fn nodes2(pair: &Pair, len: usize) -> Vec<Point> {
    let ((x1, y1), (x2, y2)) = pair;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut n = 0;
    let mut a1 = (x1 - dx * n, y1 - dy * n);
    let mut a2 = (x2 + dx * n, y2 + dy * n);
    let mut nodes = vec![];

    while on_board(a1, len) {
        nodes.push(a1);
        ep!("o -> ({},{})", a1.0, a1.1);
        n += 1;
        a1 = (x1 - dx * n, y1 - dy * n);
    }
    n = 0;

    while on_board(a2, len) {
        nodes.push(a2);
        ep!("o -> ({},{})", a2.0, a2.1);
        n += 1;
        a2 = (x2 + dx * n, y2 + dy * n);
    }

    nodes
}

fn antenas(board: &[Vec<char>]) -> HashMap<char, Vec<Point>> {
    let mut chars: HashMap<char, Vec<Point>> = HashMap::new();

    for (i, row) in board.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == '.' {
                continue;
            }
            if let Some(x) = chars.get_mut(tile) {
                x.push((i as i64, j as i64));
            } else {
                chars.insert(*tile, vec![(i as i64, j as i64)]);
            }
        }
    }
    chars.retain(|_k, v| v.len() > 1);
    for (k, v) in chars.iter() {
        ep!("{k}: {v:?}");
    }
    chars
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 8, 1, 1, 14);
    test!(p2, 8, 2, 1, 34);

    bench!(b1e, 8, 1, Some(1));
    bench!(b1i, 8, 1, None);
    bench!(b2e, 8, 2, Some(1));
    bench!(b2i, 8, 2, None);
}
