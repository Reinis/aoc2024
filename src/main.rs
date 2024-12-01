use clap::Parser;

mod day01;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    example: Option<u8>,
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

impl Args {
    fn example_filename(self: &Args) -> String {
        let Some(day) = self.day else { todo!() };
        let Some(number) = self.example else { todo!() };
        format!("data/{day:02}/example{number}")
    }

    fn input_filename(self: &Args) -> String {
        let Some(day) = self.day else { todo!() };
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
    let Some(day) = args.day else { todo!() };

    match day {
        1 => day01::run(args),
        _ => todo!(),
    };
}
