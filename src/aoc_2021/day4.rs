use advent_of_code_traits::{days::Day4, ParseInput, Solution};

use crate::AdventOfCode2021;

#[derive(Debug, Clone)]
pub struct GridBoard {
    grid: [[(i32, bool); 5]; 5],
}

impl GridBoard {
    fn new(grid: [[i32; 5]; 5]) -> Self {
        Self {
            grid: grid.map(|row| row.map(|n| (n, false))),
        }
    }

    fn is_winning(&self) -> bool {
        let any_row_matched = self
            .grid
            .iter()
            .any(|row| row.iter().all(|(_, marked)| *marked));
        let any_col_matched = (0..5).any(|x| (0..5).all(|y| self.grid[y][x].1));
        any_row_matched || any_col_matched
    }

    fn mark_number(&mut self, n: i32) {
        for row in self.grid.iter_mut() {
            for (v, marked) in row {
                if *v == n {
                    *marked = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.grid
            .map(|row| {
                row.iter()
                    .filter_map(|(n, marked)| if *marked { None } else { Some(n) })
                    .sum()
            })
            .iter()
            .sum()
    }
}

fn parse_line(s: &str) -> [i32; 5] {
    let mut line = [0; 5];
    let numbers: Vec<i32> = s
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    line.copy_from_slice(&numbers);
    line
}

impl ParseInput<Day4> for AdventOfCode2021 {
    type Parsed = (Vec<i32>, Vec<GridBoard>);

    fn parse_input(input: &str) -> Self::Parsed {
        let mut lines = input.lines();
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        let mut boards = vec![];
        while let Some(_) = lines.next() {
            let mut grid = [[0; 5]; 5];
            for i in 0..5 {
                grid[i] = parse_line(lines.next().unwrap());
            }
            let board = GridBoard::new(grid);
            boards.push(board);
        }

        (numbers, boards)
    }
}

impl Solution<Day4> for AdventOfCode2021 {
    type Part1Output = i32;
    type Part2Output = i32;

    fn part1(input: &(Vec<i32>, Vec<GridBoard>)) -> Self::Part1Output {
        let (numbers, boards) = input;
        let mut boards = boards.clone();
        for n in numbers {
            for board in boards.iter_mut() {
                board.mark_number(*n);
                if board.is_winning() {
                    return board.sum_unmarked() * n;
                }
            }
        }
        unreachable!("No answer for part1")
    }

    fn part2(input: &(Vec<i32>, Vec<GridBoard>)) -> Self::Part2Output {
        let (numbers, boards) = input;
        let mut boards = boards.clone();
        let mut winning_boards = Vec::new();
        for n in numbers {
            for (i, board) in boards.iter_mut().enumerate() {
                if let Some(_) = winning_boards.iter().find(|(index, _)| *index == i) {
                    continue;
                }
                board.mark_number(*n);
                if board.is_winning() {
                    winning_boards.push((i, n));
                }
            }
        }
        match winning_boards.pop() {
            Some((i, n)) => boards[i].sum_unmarked() * n,
            None => unreachable!("No answer for part2"),
        }
    }
}
