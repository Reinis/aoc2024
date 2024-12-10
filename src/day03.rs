use regex::Regex;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let memory = &read(filename);
    match args.part {
        1 => part1(memory),
        2 => part2(memory),
        _ => todo!(),
    }
}

fn read(filename: String) -> String {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents.trim().to_string()
}

fn part1(memory: &String) -> usize {
    let re = Regex::new(r"mul\((-?[0-9]{1,3}),(-?[0-9]{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(memory.as_ref())
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum();
    dbg!(result);
    result as usize
}

fn part2(memory: &String) -> usize {
    let re =
        Regex::new(r"(?:mul\((-?[0-9]{1,3}),(-?[0-9]{1,3})\))|(?:do\(\))|(?:don't\(\))").unwrap();
    let result = re
        .captures_iter(memory.as_ref())
        .fold((0i32, true), |sum, c| {
            let m = c.get(0).unwrap().as_str();
            ep!("{m}");
            if m == "do()" {
                return (sum.0, true);
            }
            if m == "don't()" {
                return (sum.0, false);
            }
            if !sum.1 {
                return sum;
            }
            let a: i32 = c.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = c.get(2).unwrap().as_str().parse().unwrap();
            (sum.0 + a * b, true)
        })
        .0;
    dbg!(result);
    result as usize
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 3, 1, 1, 161);
    test!(p2, 3, 2, 2, 48);

    bench!(b1e, 3, 1, Some(1));
    bench!(b1i, 3, 1, None);
    bench!(b2e, 3, 2, Some(1));
    bench!(b2i, 3, 2, None);
}
