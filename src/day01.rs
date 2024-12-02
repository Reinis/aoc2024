use super::Args;
use std::iter::zip;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
        2 => part2(filename),
        _ => todo!(),
    }
}

fn extract(filename: String) -> (Vec<i64>, Vec<i64>) {
    let contents =
        std::fs::read_to_string(dbg!(filename)).expect("should have been able to read the file");
    // println!("{contents}");
    let pairs: Vec<Vec<&str>> = contents
        .trim()
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect();
    // dbg!(&pairs);
    let mut first: Vec<i64> = Vec::new();
    let mut second: Vec<i64> = Vec::new();
    for pair in pairs {
        first.push(pair[0].parse().unwrap());
        second.push(pair[1].parse().unwrap());
    }
    // dbg!(&first);
    // dbg!(&second);
    (first, second)
}

fn part1(filename: String) -> usize {
    let (mut first, mut second) = extract(filename);
    first.sort();
    second.sort();
    // dbg!(&first);
    // dbg!(&second);
    let terms: Vec<i64> = zip(first, second).map(|(a, b)| (a - b).abs()).collect();
    // dbg!(&terms);
    let result: i64 = terms.iter().sum();
    dbg!(result);
    result as usize
}

fn part2(filename: String) -> usize {
    let (first, second) = extract(filename);
    let terms: Vec<usize> = first
        .iter()
        .map(|x| *x as usize * second.iter().filter(|y| *y == x).count())
        .collect();
    // dbg!(&terms);
    let result: usize = terms.iter().sum();
    dbg!(result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1e1() {
        let args = Args {
            day: 1,
            part: 1,
            example: Some(1),
        };
        let result = run(args);
        assert_eq!(result, 11);
    }

    #[test]
    fn p2e1() {
        let args = Args {
            day: 1,
            part: 2,
            example: Some(1),
        };
        let result = run(args);
        assert_eq!(result, 31);
    }
}
