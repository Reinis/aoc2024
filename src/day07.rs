use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let equations = &read(filename);
    match args.part {
        1 => part1(equations),
        2 => part2(equations),
        _ => todo!(),
    }
}

fn read(filename: String) -> Vec<(usize, Vec<usize>)> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            (
                parts[0].parse().unwrap(),
                parts[1].split(" ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part1(equations: &[(usize, Vec<usize>)]) -> usize {
    let op_count = 2;
    part(equations, op_count)
}

fn part2(equations: &[(usize, Vec<usize>)]) -> usize {
    let op_count = 3;
    part(equations, op_count)
}

fn part(equations: &[(usize, Vec<usize>)], op_count: usize) -> usize {
    let result = equations
        .iter()
        .filter(|&equation| possibly_valid(equation, op_count))
        .map(|(x, _)| x)
        .sum();
    dbg!(result);
    result
}

fn possibly_valid(equation: &(usize, Vec<usize>), op_count: usize) -> bool {
    let (value, args) = equation;
    solve(*value, args[0], 1, args, 0, op_count)
}

fn solve(value: usize, head: usize, k: usize, args: &[usize], opi: usize, op_count: usize) -> bool {
    if opi == op_count {
        ep!("Teriminating op {opi}: {value}: {args:?}");
        return false;
    }
    if args.len() <= k {
        ep!(
            "Teriminating value op {opi}: {value}: {args:?} {}",
            if value == head { "O" } else { "X" }
        );
        return value == head;
    }
    if value < head {
        return false;
    }
    for i in 0..op_count {
        let first = op(head, args[k], i);
        ep!(
            "Reducing {value}: {head} {:?} -> {first} {:?}",
            &args[k..],
            &args[k + 1..],
        );

        if solve(value, first, k + 1, args, i, op_count) {
            return true;
        }
    }
    false
}

fn op(a: usize, b: usize, opi: usize) -> usize {
    match opi {
        0 => a + b,
        1 => a * b,
        2 => {
            let n = b.checked_ilog10().unwrap_or(0) + 1;
            a * 10usize.pow(n) + b
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 7, 1, 1, 3749);
    test!(p2, 7, 2, 1, 11387);

    bench!(b1e, 7, 1, Some(1));
    bench!(b1i, 7, 1, None);
    bench!(b2e, 7, 2, Some(1));
    bench!(b2i, 7, 2, None);
}
