use super::Args;
use regex::Regex;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
        _ => todo!(),
    }
}

fn read(filename: String) -> String {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    // eprintln!("{contents}");

    contents.trim().to_string()
}

fn part1(filename: String) -> usize {
    let memory = read(filename);
    let re = Regex::new(r"mul\((-?[0-9]{1,3}),(-?[0-9]{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(memory.as_ref())
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum();
    dbg!(result);
    result as usize
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 3, 1, 1, 161);
}
