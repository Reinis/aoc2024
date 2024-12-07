use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,
    #[arg(short, long)]
    example: Option<u8>,
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

impl Args {
    fn example_filename(self: &Args) -> String {
        let day = self.day;
        let Some(number) = self.example else { todo!() };
        format!("data/{day:02}/example{number}")
    }

    fn input_filename(self: &Args) -> String {
        let day = self.day;
        format!("data/{day:02}/input")
    }

    fn filename(self: &Args) -> String {
        if self.example.is_some() {
            self.example_filename()
        } else {
            self.input_filename()
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day01::run(args),
        2 => day02::run(args),
        3 => day03::run(args),
        4 => day04::run(args),
        5 => day05::run(args),
        6 => day06::run(args),
        7 => day07::run(args),
        _ => todo!(),
    };
}

#[macro_export]
macro_rules! test {
    ($func:ident, $day:expr, $part:expr, $example:expr, $expected:expr) => {
        #[test]
        fn $func() {
            let args = $crate::Args {
                day: $day,
                part: $part,
                example: Some($example),
            };
            assert_eq!(super::run(args), $expected);
        }
    };
}
