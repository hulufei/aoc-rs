use advent_of_code_traits::days::{Day1, Day2, Day3, Day4, Day5, Day6};
use advent_of_code_traits::Solution;
use aoc_rs::AdventOfCode2021;
use std::fs::read_to_string;

macro_rules! run {
    ($aoc: ty, $day: ty, $input: expr) => {
        <$aoc as Solution<$day>>::run($input)
    };
}

fn main() {
    let mut year = None;
    let mut day = None;
    let mut args = std::env::args();
    while let Some(arg) = args.next().as_deref() {
        match arg {
            "-y" => year = args.next(),
            "-d" => day = args.next(),
            _ => (),
        }
    }
    let year = year.as_deref().unwrap_or("2021");
    let day = day
        .as_deref()
        .expect("Require -d to specify which day to run");
    let input_file = format!("input/{}/day{}.txt", year, day);
    let input = read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));

    match (year, day) {
        ("2021", "1") => run!(AdventOfCode2021, Day1, &input),
        ("2021", "2") => run!(AdventOfCode2021, Day2, &input),
        ("2021", "3") => run!(AdventOfCode2021, Day3, &input),
        ("2021", "4") => run!(AdventOfCode2021, Day4, &input),
        ("2021", "5") => run!(AdventOfCode2021, Day5, &input),
        ("2021", "6") => run!(AdventOfCode2021, Day6, &input),
        _ => println!("Unknown year {}, day {}", year, day),
    }
}
