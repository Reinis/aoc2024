use std::str::FromStr;

use crate::Args;
use crate::DEBUG;
use crate::ep;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let machines = &read(filename);
    match args.part {
        1 => part1(machines),
        _ => todo!(),
    }
}

#[derive(Debug, PartialEq)]
struct Button {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseButtonError;

impl FromStr for Button {
    type Err = ParseButtonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s[12..].split_once(", Y+").ok_or(ParseButtonError)?;
        ep!("x: {x_str} y: {y_str}");

        let x = x_str.parse::<f64>().map_err(|_| ParseButtonError)?;
        let y = y_str.parse::<f64>().map_err(|_| ParseButtonError)?;

        Ok(Button { x, y })
    }
}

#[derive(Debug, PartialEq)]
struct Prize {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePrizeError;

impl FromStr for Prize {
    type Err = ParsePrizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s[9..].split_once(", Y=").ok_or(ParsePrizeError)?;
        ep!("x: {x_str} y: {y_str}");

        let x = x_str.parse::<f64>().map_err(|_| ParsePrizeError)?;
        let y = y_str.parse::<f64>().map_err(|_| ParsePrizeError)?;

        Ok(Prize { x, y })
    }
}

fn read(filename: String) -> Vec<(Button, Button, Prize)> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .trim()
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.lines().collect();
            let a = Button::from_str(lines[0]).unwrap();
            let b = Button::from_str(lines[1]).unwrap();
            let prize = Prize::from_str(lines[2]).unwrap();
            (a, b, prize)
        })
        .collect()
}

fn part1(machines: &[(Button, Button, Prize)]) -> usize {
    let cost = machines
        .iter()
        .filter_map(|(a, b, p)| solve(a, b, p))
        .map(|(cost_a, cost_b)| cost_a * 3 + cost_b)
        .sum();
    dbg!(cost);
    cost
}

fn solve(a: &Button, b: &Button, p: &Prize) -> Option<(usize, usize)> {
    // ax1 + bx2 = x3
    // ay1 + by2 = y3
    //
    // a = (y3-by2)/y1
    //
    // (y3-by2)x1/y1 + bx2 = x3
    // y3x1/y1 - by2x1/y1 + bx2 = x3
    // bx2 - by2x1/y1 = x3 - y3x1/y1
    // b(x2-y2x1/y1) = x3 - y3x1/y1
    // b = (x3 - y3x1/y1)/(x2-y2x1/y1)
    // b = (x3y1 - y3x1)/(x2y1-y2x1)
    let b_n = (p.x * a.y - p.y * a.x) / (b.x * a.y - b.y * a.x);
    let a_n = (p.y - b_n * b.y) / a.y;
    ep!("{a_n}");
    ep!("{b_n}");

    if a_n.fract() != 0.0 || b_n.fract() != 0.0 {
        None
    } else {
        Some((a_n as usize, b_n as usize))
    }
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 13, 1, 1, 480);
}
