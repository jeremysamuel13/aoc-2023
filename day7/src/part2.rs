use crate::utils::*;
use aoc_core::*;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Part2 {
    entries: Vec<Entry<false>>,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            entries: input.map(Entry::parse_string).collect(),
        })
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .entries
            .iter()
            .sorted_by(|l, r| l.hand.compare_hand(&r.hand))
            .enumerate()
            .map(|(idx, e)| e.bid * (idx + 1))
            .sum::<usize>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "5905");

        Ok(())
    }
}
