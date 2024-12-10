use crate::Args;
use crate::DEBUG;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let reports = &read(filename);
    match args.part {
        1 => part1(reports),
        2 => part2(reports),
        _ => todo!(),
    }
}

fn read(filename: String) -> Vec<Vec<i8>> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .lines()
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .collect()
}

fn part1(reports: &[Vec<i8>]) -> usize {
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

fn part2(reports: &[Vec<i8>]) -> usize {
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
    use crate::bench;
    use crate::test;

    test!(p1, 2, 1, 1, 2);
    test!(p2, 2, 2, 1, 4);

    bench!(b1e, 2, 1, Some(1));
    bench!(b1i, 2, 1, None);
    bench!(b2e, 2, 2, Some(1));
    bench!(b2i, 2, 2, None);
}
