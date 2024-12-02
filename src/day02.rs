use super::Args;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
        _ => todo!(),
    }
}

fn read(filename: String) -> Vec<Vec<i8>> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    // eprintln!("{contents}");

    contents
        .lines()
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .collect()
}

fn part1(filename: String) -> usize {
    let reports = read(filename);
    let safe = reports.iter().filter(|x| is_safe(x)).count();
    dbg!(safe);
    safe
}

fn is_safe(report: &[i8]) -> bool {
    let mut diff = 0;
    for (a, b) in report[..report.len() - 1].iter().zip(report[1..].iter()) {
        let d = b - a;
        if d.abs() < 1 || d.abs() > 3 {
            return false;
        }
        if diff == 0 {
            diff = d;
            continue;
        }
        if diff > 0 && d < 0 || diff < 0 && d > 0 {
            return false;
        }
    }
    true
}
