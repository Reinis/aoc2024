use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let stones = &read(filename);
    match args.part {
        1 => part1(stones),
        _ => todo!(),
    }
}

fn read(filename: String) -> Vec<usize> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect()
}

fn part1(stones: &[usize]) -> usize {
    let stones = &mut stones.to_owned();
    for step in 0..25 {
        let len = stones.len();
        let mut add = Vec::new();
        for k in 0..len {
            if stones[k] == 0 {
                stones[k] = 1;
                continue;
            }
            let stone = stones[k];
            let n = stone.ilog10() + 1;
            if n % 2 == 0 {
                let factor = 10usize.pow(n / 2);
                stones[k] /= factor;
                add.push((k, stone % factor));
                continue;
            }
            stones[k] = stone * 2024;
        }
        for (i, (k, stone)) in add.iter().enumerate() {
            stones.insert(k + i + 1, *stone);
        }
        ep!("{step:02}: {}", stones.len());
        // ep!("{stones:?}");
    }
    let count = stones.len();
    dbg!(count);
    count
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 11, 1, 1, 55312);
}
