use std::collections::HashMap;

use advent_of_code_traits::{days::Day5, ParseInput, Solution};

use crate::AdventOfCode2021;

type Line = ((u32, u32), (u32, u32));

impl ParseInput<Day5> for AdventOfCode2021 {
    type Parsed = Vec<Line>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let parse_point = |s: &str| -> (u32, u32) {
                    let mut points = s.split(',').map(|n| n.parse().unwrap());
                    (points.next().unwrap(), points.next().unwrap())
                };
                let mut parts = line.split(" -> ").map(parse_point);
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect()
    }
}

impl Solution<Day5> for AdventOfCode2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Line>) -> Self::Part1Output {
        let mut cover_points = HashMap::new();
        for line in input {
            let ((x1, y1), (x2, y2)) = line;
            if x1 == x2 {
                let range = if y1 < y2 { *y1..=*y2 } else { *y2..=*y1 };
                for y in range {
                    let count = cover_points.entry((*x1, y)).or_insert(0);
                    *count += 1;
                }
            } else if y1 == y2 {
                let range = if x1 < x2 { *x1..=*x2 } else { *x2..=*x1 };
                for x in range {
                    let count = cover_points.entry((x, *y1)).or_insert(0);
                    *count += 1;
                }
            }
        }
        cover_points.iter().filter(|(_, &count)| count >= 2).count()
    }

    fn part2(input: &Vec<Line>) -> Self::Part2Output {
        let mut cover_points = HashMap::new();
        for line in input {
            let ((x1, y1), (x2, y2)) = line;
            if x1 == x2 {
                let range = if y1 < y2 { *y1..=*y2 } else { *y2..=*y1 };
                for y in range {
                    let count = cover_points.entry((*x1, y)).or_insert(0);
                    *count += 1;
                }
            } else if y1 == y2 {
                let range = if x1 < x2 { *x1..=*x2 } else { *x2..=*x1 };
                for x in range {
                    let count = cover_points.entry((x, *y1)).or_insert(0);
                    *count += 1;
                }
            } else {
                let rangex: Vec<_> = if x1 < x2 {
                    (*x1..=*x2).collect()
                } else {
                    (*x2..=*x1).rev().collect()
                };
                let rangey: Vec<_> = if y1 < y2 {
                    (*y1..=*y2).collect()
                } else {
                    (*y2..=*y1).rev().collect()
                };
                for point in rangex.into_iter().zip(rangey.into_iter()) {
                    let count = cover_points.entry(point).or_insert(0);
                    *count += 1;
                }
            }
        }
        cover_points.iter().filter(|(_, &count)| count >= 2).count()
    }
}
