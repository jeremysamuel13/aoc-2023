mod part1;
mod part2;
use aoc_core::*;
use part1::*;
use part2::*;


#[derive(Clone)]
pub struct Day{{day}};

impl Solution for Day{{day}} {
    type Input1 = Part1;
    type Input2 = Part2;
    
    const DAY: usize = {{day}};
    
    fn part_1(&self, mut input: Self::Input1) -> Result<String, AOCError> {
        input.solve()
    }

    fn part_2(&self, mut input: Self::Input2) -> Result<String, AOCError> {
        input.solve()
    }
}

