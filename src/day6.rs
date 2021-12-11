use advent_of_code_traits::{days::Day6, ParseInput, Solution};

use crate::AdventOfCode2021;

impl ParseInput<Day6> for AdventOfCode2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.split(',').filter_map(|n| n.parse().ok()).collect()
    }
}

impl Solution<Day6> for AdventOfCode2021 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &Vec<u32>) -> Self::Part1Output {
        let mut state: Vec<u32> = input.clone();
        for _ in 0..80 {
            let mut new_fishes = vec![];
            state = state
                .iter()
                .map(|v| {
                    if *v == 0 {
                        new_fishes.push(8);
                        6
                    } else {
                        v - 1
                    }
                })
                .collect();
            state.extend(new_fishes);
        }
        state.len()
    }

    fn part2(input: &Vec<u32>) -> Self::Part2Output {
        0
    }
}

#[test]
fn test_part1() {
    let input = "3,4,3,1,2";
    let input = <AdventOfCode2021 as ParseInput<Day6>>::parse_input(input);
    assert_eq!(<AdventOfCode2021 as Solution<Day6>>::part1(&input), 5934);
}
