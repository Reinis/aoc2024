use std::collections::HashMap;
use std::mem;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let stones = &read(filename);
    match args.part {
        1 => part1(stones),
        2 => part2(stones),
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
    part(stones, 25)
}

fn part2(stones: &[usize]) -> usize {
    part(stones, 75)
}

fn part(stones: &[usize], steps: usize) -> usize {
    let mut stones: HashMap<usize, usize> = stones.iter().map(|&stone| (stone, 1)).collect();
    let mut stones2: HashMap<usize, usize> = HashMap::new();
    for step in 0..steps {
        for (stone, count) in stones.iter() {
            if *stone == 0 {
                *stones2.entry(1).or_insert(0) += count;
                continue;
            }
            let n = stone.ilog10() + 1;
            if n % 2 == 0 {
                let factor = 10usize.pow(n / 2);
                *stones2.entry(stone / factor).or_insert(0) += count;
                *stones2.entry(stone % factor).or_insert(0) += count;
                continue;
            }
            *stones2.entry(stone * 2024).or_insert(0) += count;
        }
        mem::swap(&mut stones, &mut stones2);
        stones2.clear();
        ep!("{step:02}: {}", stones.values().sum::<usize>());
        // ep!("{stones:#?}");
    }
    let count = stones.values().sum();
    dbg!(count);
    count
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 11, 1, 1, 55312);

    bench!(b1e, 11, 1, Some(1));
    bench!(b1i, 11, 1, None);
    bench!(b2e, 11, 2, Some(1));
    bench!(b2i, 11, 2, None);
}
