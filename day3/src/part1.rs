use crate::parsing::{Element, Sequence};
use crate::utils::adjacent;
use aoc_core::*;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Part1 {
    sequences: Vec<Sequence>,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            sequences: input
                .enumerate()
                .map(|(row, line)| Sequence::from_str_and_row(&line, row))
                .collect(),
        })
    }
}

impl Part1 {
    pub fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .sequences
            .iter()
            .enumerate()
            .flat_map(|(x, s)| {
                s.iter()
                    .enumerate()
                    .filter(|(_, e)| e.is_symbol())
                    .flat_map(move |(y, _)| adjacent((x, y)))
                    .filter_map(|(x, y)| match self.sequences[x][y] {
                        Element::Number { number, row, count } => Some((number, row, count)),
                        _ => None,
                    })
            })
            .unique()
            .map(|(number, _, _)| number)
            .sum::<usize>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "4361");

        Ok(())
    }
}
