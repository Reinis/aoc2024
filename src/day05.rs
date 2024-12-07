use std::cmp::Ordering;

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

fn read(filename: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    let blocks: Vec<Vec<&str>> = contents
        .trim()
        .split("\n\n")
        .map(|x| x.lines().collect())
        .collect();
    let rules = blocks[0]
        .iter()
        .map(|&x| {
            let pair = x
                .split("|")
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (pair[0], pair[1])
        })
        .collect();
    let updates = blocks[1]
        .iter()
        .map(|x| x.split(",").map(|y| y.parse::<usize>().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn part1(filename: String) -> usize {
    let (rules, updates) = read(filename);
    let count = updates
        .iter()
        .filter(|update| is_ordered(update, &rules))
        .inspect(|x| debug_update(x))
        .map(|update| get_mid(update))
        .inspect(debug_page)
        .sum();
    dbg!(count);
    count
}

fn get_mid(update: &[usize]) -> usize {
    update[update.len().div_ceil(2) - 1]
}

fn is_ordered(update: &[usize], rules: &Vec<(usize, usize)>) -> bool {
    for (x, y) in rules {
        if !update.contains(x) || !update.contains(y) {
            continue;
        }
        if update.iter().position(|&n| n == *x) > update.iter().position(|&n| n == *y) {
            return false;
        }
    }
    true
}

fn debug_page(page: &usize) {
    if *DEBUG {
        eprintln!("{page}")
    }
}

fn debug_update(update: &Vec<usize>) {
    if *DEBUG {
        eprintln!("{update:?}")
    }
}

fn part2(filename: String) -> usize {
    let (rules, updates) = read(filename);
    let count = updates
        .iter()
        .filter(|update| !is_ordered(update, &rules))
        .inspect(|x| debug_update(x))
        .map(|update| order(update, &rules))
        .inspect(debug_update)
        .map(|update| get_mid(&update))
        .inspect(debug_page)
        .sum();
    dbg!(count);
    count
}

fn order(update: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let mut result = update.to_vec();
    result.sort_by(|&x, &y| {
        if rules.contains(&(x, y)) {
            return Ordering::Less;
        }
        if rules.contains(&(y, x)) {
            return Ordering::Greater;
        }
        Ordering::Equal
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 5, 1, 1, 143);
    test!(p2, 5, 2, 1, 123);
}
