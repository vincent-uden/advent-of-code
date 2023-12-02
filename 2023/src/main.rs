mod day01;

use std::fs;

use clap::Parser;


#[derive(Parser)]
#[command(
    author = "Vincent Udén",
    about = "Solutions to Advent of Code"
)]
struct Cli {
    #[clap(short, long)]
    day: i32,

    #[clap(short, long)]
    part: i32,
}

pub(crate) struct Solution<'a> {
    part1: &'a dyn Fn(&str) -> (),
    part2: &'a dyn Fn(&str) -> (),
}

fn main() {
    let days: Vec<Solution> = vec![
        Solution { part1: &day01::part1, part2: &day01::part2 }
    ];

    let args = Cli::parse();

    if args.part == 1 {
        (days[args.day as usize - 1].part1)(fs::read_to_string(format!("./inputs/day{}.txt", args.day)).unwrap().as_str());
    }
    if args.part == 2 {
        (days[args.day as usize - 1].part2)(fs::read_to_string(format!("./inputs/day{}.txt", args.day)).unwrap().as_str());
    }
}
