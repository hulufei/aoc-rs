use advent_of_code_traits::{days::Day1, ParseInput, Solution};

use crate::AdventOfCode2021;

impl ParseInput<Day1> for AdventOfCode2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().filter_map(|l| l.parse().ok()).collect()
    }
}

impl Solution<Day1> for AdventOfCode2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u32>) -> Self::Part1Output {
        input.windows(2).filter(|w| w[0] < w[1]).count()
    }

    fn part2(input: &Vec<u32>) -> Self::Part2Output {
        input
            .windows(3)
            .map(|w| w.into_iter().sum())
            .collect::<Vec<u32>>()
            .as_slice()
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count()
    }
}
