use crate::utils::*;
use aoc_core::*;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Part1 {
    races: Vec<Race>,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        let [time, distance] = &input
            .map(|line| {
                line.split_whitespace()
                    .flat_map(str::parse)
                    .collect::<Vec<_>>()
            })
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()[..]
        else {
            panic!("Expected only two lines in input")
        };

        Ok(Self {
            races: time.iter().zip(distance.iter()).map(Race::from).collect(),
        })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .races
            .iter()
            .map(|race| race.possible_wins())
            .product::<isize>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "288");

        Ok(())
    }
}
