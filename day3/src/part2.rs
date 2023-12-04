use aoc_core::*;
use itertools::Itertools;

use crate::{
    parsing::{Element, Sequence},
    utils::adjacent,
};

#[derive(Clone)]
pub struct Part2 {
    sequences: Vec<Sequence>,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            sequences: input
                .enumerate()
                .map(|(row, line)| Sequence::from_str_and_row(&line, row))
                .collect(),
        })
    }
}

impl Part2 {
    pub fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .sequences
            .iter()
            .enumerate()
            .flat_map(|(x, seq)| {
                seq.iter()
                    .enumerate()
                    .filter_map(move |(y, e)| e.is_symbol().then_some((x, y, e)))
            })
            .filter_map(|(x, y, _symbol)| {
                let mut len = 0;
                let product = adjacent((x, y))
                    .map(|(nx, ny)| &self.sequences[nx][ny])
                    .unique()
                    .filter_map(Element::try_number)
                    .inspect(|_| len += 1)
                    .product::<usize>();
                (len == 2).then_some(product)
            })
            .sum::<usize>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
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

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "467835");

        Ok(())
    }
}
