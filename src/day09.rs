use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    match args.part {
        1 => part1(filename),
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

fn part1(filename: String) -> usize {
    let disk_map = read(filename);
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

fn expand(disk_map: Vec<usize>) -> Vec<String> {
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
    use crate::test;

    test!(p1, 9, 1, 1, 1928);
}
