use super::Args;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
        2 => part2(filename),
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
    for w in report.windows(2) {
        let d = w[1] - w[0];
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

fn part2(filename: String) -> usize {
    let reports = read(filename);
    let safe = reports.iter().filter(|x| is_tolerable(x)).count();
    dbg!(safe);
    safe
}

fn is_tolerable(report: &[i8]) -> bool {
    if is_safe(report) {
        return true;
    }
    let v = Vec::from(report);
    for i in 0..report.len() {
        let mut w = v.clone();
        w.remove(i);
        if is_safe(w.as_ref()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 2, 1, 1, 2);
    test!(p2, 2, 2, 1, 4);
}
