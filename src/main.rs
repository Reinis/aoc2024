use std::iter::zip;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    example: Option<u8>,
    #[arg(short, long)]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let Some(day) = args.day else { todo!() };

    match day {
        1 => run(day, args),
        _ => todo!(),
    }
}

fn run(day: u8, args: Args) {
    match day {
        1 => day01(args),
        _ => todo!(),
    }
}

fn day01(args: Args) {
    if let Some(number) = args.example {
        if let Some(part) = args.part {
            example(1, part, number);
        } else {
            example(1, 1, number);
            example(1, 2, number);
        }
    } else if let Some(part) = args.part {
        input(1, part)
    } else {
        input(1, 1);
        input(1, 2);
    }
}

fn example(day: u8, part: u8, number: u8) {
    let filename = format!("data/{day:02}/example{number}");
    if part == 1 {
        part1(filename);
    } else {
        part2(filename);
    }
}

fn input(day: u8, part: u8) {
    let filename = format!("data/{day:02}/input");
    if part == 1 {
        part1(filename);
    } else {
        part2(filename);
    }
}

fn extract(filename: String) -> (Vec<i64>, Vec<i64>) {
    let contents =
        std::fs::read_to_string(dbg!(filename)).expect("should have been able to read the file");
    // println!("{contents}");
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    // dbg!(&lines);
    let pairs: Vec<Vec<&str>> = lines
        .iter()
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

fn part1(filename: String) {
    let (mut first, mut second) = extract(filename);
    first.sort();
    second.sort();
    // dbg!(&first);
    // dbg!(&second);
    let terms: Vec<i64> = zip(first, second).map(|(a, b)| (a - b).abs()).collect();
    // dbg!(&terms);
    let result: i64 = terms.iter().sum();
    dbg!(result);
}

fn part2(filename: String) {
    let (first, second) = extract(filename);
    let terms: Vec<usize> = first
        .iter()
        .map(|x| *x as usize * second.iter().filter(|y| *y == x).count())
        .collect();
    // dbg!(&terms);
    let result: usize = terms.iter().sum();
    dbg!(result);
}
