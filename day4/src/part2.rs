use crate::utils::*;
use aoc_core::*;

#[derive(Clone)]
pub struct Part2 {
    scratchcards: Vec<ScratchCard>,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            scratchcards: input
                .map(|line| ScratchCard::parse_from_str(line.trim()))
                .collect(),
        })
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        let mut counts = vec![1; self.scratchcards.len()];

        for sc in self.scratchcards.iter() {
            for i in 0..sc.calculate_winning_numbers().count() {
                counts[sc.id + i] += counts[sc.id - 1];
            }
        }

        Ok(counts.iter().sum::<usize>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "30");

        Ok(())
    }
}
