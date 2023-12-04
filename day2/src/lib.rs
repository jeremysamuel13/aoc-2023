mod part1;
mod part2;
mod utils;
use aoc_core::*;
use part1::*;
use part2::*;

#[derive(Debug, Clone)]
pub struct Day2;

impl Solution for Day2 {
    type Input1 = Part1;
    type Input2 = Part2;

    const DAY: usize = 2;
}
