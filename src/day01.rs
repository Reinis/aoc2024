use std::iter::zip;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let location_lists = &read(filename);
    match args.part {
        1 => part1(location_lists),
        2 => part2(location_lists),
        _ => todo!(),
    }
}

fn read(filename: String) -> (Vec<i64>, Vec<i64>) {
    let contents =
        std::fs::read_to_string(dbg!(filename)).expect("should have been able to read the file");
    if *DEBUG {
        println!("{contents}");
    }
    let pairs: Vec<Vec<&str>> = contents
        .trim()
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let mut first: Vec<i64> = Vec::new();
    let mut second: Vec<i64> = Vec::new();
    for pair in pairs {
        first.push(pair[0].parse().unwrap());
        second.push(pair[1].parse().unwrap());
    }
    (first, second)
}

fn part1(location_lists: &(Vec<i64>, Vec<i64>)) -> usize {
    let (mut first, mut second) = location_lists.to_owned();
    first.sort();
    second.sort();
    ep!("{first:?}");
    ep!("{second:?}");
    let terms: Vec<i64> = zip(first, second).map(|(a, b)| (a - b).abs()).collect();
    ep!("{terms:?}");
    let result: i64 = terms.iter().sum();
    dbg!(result);
    result as usize
}

fn part2(location_lists: &(Vec<i64>, Vec<i64>)) -> usize {
    let (first, second) = location_lists;
    let terms: Vec<usize> = first
        .iter()
        .map(|x| *x as usize * second.iter().filter(|y| *y == x).count())
        .collect();
    ep!("{terms:?}");
    let result: usize = terms.iter().sum();
    dbg!(result);
    result
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 1, 1, 1, 11);
    test!(p2, 1, 2, 1, 31);

    bench!(b1e, 1, 1, Some(1));
    bench!(b1i, 1, 1, None);
    bench!(b2e, 1, 2, Some(1));
    bench!(b2i, 1, 2, None);
}
