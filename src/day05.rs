use super::Args;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
        _ => todo!(),
    }
}

const DEBUG: bool = false;

fn read(filename: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if DEBUG {
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
    if DEBUG {
        eprintln!("{page}")
    }
}

fn debug_update(update: &Vec<usize>) {
    if DEBUG {
        eprintln!("{update:?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 5, 1, 1, 143);
}
