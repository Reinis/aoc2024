use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let disk_map = &read(filename);
    match args.part {
        1 => part1(disk_map),
        2 => part2(disk_map),
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
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn part1(disk_map: &[usize]) -> usize {
    let mut disk_ids = expand(disk_map);
    compact(&mut disk_ids);
    let checksum = disk_ids
        .iter()
        .enumerate()
        .map(|(i, label)| {
            if label == "." {
                0
            } else {
                i * label.parse::<usize>().unwrap()
            }
        })
        .sum();
    dbg!(checksum);
    checksum
}

fn compact(disk_ids: &mut [String]) {
    let len = disk_ids.len();
    let mut end = len - 1;

    for i in 0..len {
        if i >= end - 1 {
            break;
        }
        if disk_ids[i] != "." {
            continue;
        }
        for j in (i + 1..len).rev() {
            if disk_ids[j] == "." {
                continue;
            }
            disk_ids[i] = disk_ids[j].clone();
            disk_ids[j] = ".".to_string();
            end -= 1;
            break;
        }
        ep!("{}", disk_ids.concat());
    }
}

fn part2(disk_map: &[usize]) -> usize {
    let mut disk_ids = blocks(disk_map);
    compact2(&mut disk_ids);
    let checksum = disk_ids
        .iter()
        .fold((0, 0), |(acc, last), block| {
            let next = if block.0 {
                block.2 * (last..last + block.1).sum::<usize>()
            } else {
                0
            };
            (acc + next, last + block.1)
        })
        .0;
    dbg!(checksum);
    checksum
}

fn blocks(disk_map: &[usize]) -> Vec<(bool, usize, usize)> {
    let mut id = 0;
    disk_map
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 0 {
                let block = (true, x, id);
                id += 1;
                block
            } else {
                (false, x, 0)
            }
        })
        .collect()
}

fn compact2(blocks: &mut Vec<(bool, usize, usize)>) {
    let len = blocks.len();
    ep!("{blocks:?} {len}");
    print_mem(blocks);

    for i in (0..len).rev() {
        if !blocks[i].0 {
            continue;
        }
        let size = blocks[i].1;
        for j in 0..len - 1 {
            if j >= i {
                break;
            }
            if blocks[j].0 {
                continue;
            }
            if blocks[j].1 < size {
                continue;
            }
            if blocks[j].1 == size {
                blocks[j] = blocks[i];
                blocks[i] = (false, size, 0);
                break;
            }
            let d = blocks[j].1 - size;
            blocks[j] = blocks[i];
            blocks[i] = (false, size, 0);
            if i + 1 < blocks.len() && !blocks[i + 1].0 {
                blocks[i].1 += blocks.remove(i + 1).1;
            }
            if !blocks[i - 1].0 {
                blocks[i - 1].1 += blocks.remove(i).1;
            }
            blocks.insert(j + 1, (false, d, 0));
            break;
        }
        print_mem(blocks);
    }
    ep!("{blocks:?} {}", blocks.len());
}

fn print_mem(blocks: &[(bool, usize, usize)]) {
    if !*DEBUG {
        return;
    }
    for (is_file, size, id) in blocks {
        let char = if *is_file { &format!("{id}") } else { "." };
        eprint!("{}", char.repeat(*size))
    }
    eprintln!();
}

fn expand(disk_map: &[usize]) -> Vec<String> {
    let mut result = Vec::new();
    let mut id = 0;

    for (i, &block_count) in disk_map.iter().enumerate() {
        let label = if i % 2 != 0 {
            ".".to_string()
        } else {
            let label = id.to_string();
            id += 1;
            label
        };

        for _ in 0..block_count {
            result.push(label.clone());
        }
    }
    ep!("{}", result.concat());
    result
}

#[cfg(test)]
mod tests {
    use crate::bench;
    use crate::test;

    test!(p1, 9, 1, 1, 1928);
    test!(p2, 9, 2, 1, 2858);

    bench!(b1e, 9, 1, Some(1));
    bench!(b2e, 9, 2, Some(1));
}
