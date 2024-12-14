use std::str::FromStr;

use crate::Args;
use crate::DEBUG;

pub(crate) fn run(args: Args) -> usize {
    let filename = args.filename();
    let robots = &read(filename);
    match args.part {
        1 => part1(robots),
        _ => todo!(),
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct Robot(Position, Velocity);

#[derive(Debug, PartialEq, Eq)]
struct ParseRobotError;

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s
            .trim()
            .strip_prefix("p=")
            .and_then(|s| s.split_once(" v="))
            .ok_or(ParseRobotError)?;

        let (px, py) = pos
            .trim()
            .split_once(",")
            .map(|(x, y)| {
                (
                    x.to_string().parse().unwrap(),
                    y.to_string().parse().unwrap(),
                )
            })
            .ok_or(ParseRobotError)?;
        let (vx, vy) = vel
            .trim()
            .split_once(",")
            .map(|(x, y)| {
                (
                    x.to_string().parse().unwrap(),
                    y.to_string().parse().unwrap(),
                )
            })
            .ok_or(ParseRobotError)?;

        Ok(Robot(Position { x: px, y: py }, Velocity { x: vx, y: vy }))
    }
}

fn read(filename: String) -> Vec<Robot> {
    let contents =
        std::fs::read_to_string(filename).expect("should have been able to read the file");
    if *DEBUG {
        eprintln!("{contents}");
    }

    contents
        .trim()
        .lines()
        .map(|line| Robot::from_str(line).unwrap())
        .collect()
}

fn part1(robots: &[Robot]) -> usize {
    let robots = &mut robots.to_owned();
    let dx = if robots.len() <= 12 { 11 } else { 101 };
    let dy = if robots.len() <= 12 { 7 } else { 103 };
    let t = 100;

    print_robots(robots, dx, dy);
    evolve(robots, dx, dy, t);
    print_robots(robots, dx, dy);

    let safety_factor = quadrant_populations(robots, dx, dy).iter().product();
    dbg!(safety_factor);
    safety_factor
}

fn quadrant_populations(robots: &[Robot], dx: i64, dy: i64) -> [usize; 4] {
    let hx = (dx - 1) / 2;
    let hy = (dy - 1) / 2;
    let q1 = robots
        .iter()
        .filter(|&r| r.0.x >= 0 && r.0.x < hx && r.0.y >= 0 && r.0.y < hy)
        .count();
    let q2 = robots
        .iter()
        .filter(|&r| r.0.x > hx && r.0.y >= 0 && r.0.y < hy)
        .count();
    let q3 = robots
        .iter()
        .filter(|&r| r.0.x >= 0 && r.0.x < hx && r.0.y > hy)
        .count();
    let q4 = robots.iter().filter(|&r| r.0.x > hx && r.0.y > hy).count();
    [q1, q2, q3, q4]
}

fn evolve(robots: &mut Vec<Robot>, dx: i64, dy: i64, time: i64) {
    for robot in robots {
        robot.0.x += time * robot.1.x;
        robot.0.y += time * robot.1.y;
        robot.0.x = robot.0.x.rem_euclid(dx);
        robot.0.y = robot.0.y.rem_euclid(dy);
    }
}

fn print_robots(robots: &Vec<Robot>, dx: i64, dy: i64) {
    if !*DEBUG {
        return;
    }
    let mut board: Vec<Vec<usize>> = (0..dy).map(|_| (0..dx).map(|_| 0).collect()).collect();

    for robot in robots {
        board[robot.0.y as usize][robot.0.x as usize] += 1;
    }
    for row in board {
        for tile in row {
            eprint!(
                "{}",
                if tile == 0 {
                    ".".to_string()
                } else {
                    tile.to_string()
                }
            )
        }
        eprintln!();
    }
    eprintln!("---");
}

#[cfg(test)]
mod tests {
    use crate::test;

    test!(p1, 14, 1, 1, 12);
}
