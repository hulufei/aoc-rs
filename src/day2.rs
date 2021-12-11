use advent_of_code_traits::{days::Day2, ParseInput, Solution};

use crate::AdventOfCode2021;

pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl ParseInput<Day2> for AdventOfCode2021 {
    type Parsed = Vec<Instruction>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let action = iter.next().unwrap();
                let steps = iter.next().unwrap().parse().unwrap();
                match action {
                    "forward" => Instruction::Forward(steps),
                    "down" => Instruction::Down(steps),
                    "up" => Instruction::Up(steps),
                    _ => panic!("Unknown line {}", line),
                }
            })
            .collect()
    }
}

impl Solution<Day2> for AdventOfCode2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Instruction>) -> Self::Part1Output {
        let (x, y) = input
            .iter()
            .fold((0, 0), |(x, y), instruction| match instruction {
                Instruction::Down(steps) => (x, y + steps),
                Instruction::Up(steps) => (x, y - steps),
                Instruction::Forward(steps) => (x + steps, y),
            });
        x * y
    }

    fn part2(input: &Vec<Instruction>) -> Self::Part2Output {
        let (x, y, _) =
            input
                .iter()
                .fold((0, 0, 0), |(x, y, aim), instruction| match instruction {
                    Instruction::Down(steps) => (x, y, aim + steps),
                    Instruction::Up(steps) => (x, y, aim - steps),
                    Instruction::Forward(steps) => (x + steps, y + aim * steps, aim),
                });
        x * y
    }
}
