use super::Args;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename, 2),
        2 => part1(filename, 3),
        _ => todo!(),
    }
}

const DEBUG: bool = false;

fn read(filename: String) -> Vec<(usize, Vec<usize>)> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if DEBUG {
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

fn part1(filename: String, op_count: usize) -> usize {
    let equations = read(filename);
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
    solve(*value, args, 0, op_count)
}

fn solve(value: usize, args: &[usize], opi: usize, op_count: usize) -> bool {
    if opi == op_count {
        dbg_print(format!("Teriminating op {opi}: {value}: {args:?}"));
        return false;
    }
    if args.len() == 1 {
        dbg_print(format!(
            "Teriminating value op {opi}: {value}: {args:?} {}",
            if value == args[0] { "O" } else { "X" }
        ));
        return value == args[0];
    }
    for i in 0..op_count {
        let first = op(args[0], args[1], i);
        let args1 = &mut vec![first];
        args1.extend(&args[2..]);
        dbg_print(format!("Reducing {value}: {args:?} -> {args1:?}"));

        if solve(value, args1, i, op_count) {
            return true;
        }
    }
    false
}

fn dbg_print(message: String) {
    if DEBUG {
        eprintln!("{message}")
    }
}

fn op(a: usize, b: usize, opi: usize) -> usize {
    match opi {
        0 => a + b,
        1 => a * b,
        2 => format!("{a}{b}").parse().unwrap(),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 7, 1, 1, 3749);
    test!(p2, 7, 2, 1, 11387);
}
