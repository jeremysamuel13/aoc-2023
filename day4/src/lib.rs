mod part1;
mod part2;
mod utils;
use aoc_core::*;
use part1::*;
use part2::*;

#[derive(Debug, Clone)]
pub struct Day4;

impl Solution for Day4 {
    type Input1 = Part1;
    type Input2 = Part2;

    const DAY: usize = 4;

    fn part_1(&self, mut input: Self::Input1) -> AOCResult<String> {
        input.solve()
    }

    fn part_2(&self, mut input: Self::Input2) -> AOCResult<String> {
        input.solve()
    }
}
