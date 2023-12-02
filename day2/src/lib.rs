mod part1;
mod part2;
use aoc_core::*;
use part1::*;
use part2::*;

#[derive(Clone)]
pub struct Day2;

impl Solution for Day2 {
    type Input1 = Part1;
    type Input2 = Part2;

    const DAY: usize = 2;

    fn part_1(&self, mut input: Self::Input1) -> AOCResult<String> {
        input.solve()
    }

    fn part_2(&self, mut input: Self::Input2) -> AOCResult<String> {
        input.solve()
    }
}
