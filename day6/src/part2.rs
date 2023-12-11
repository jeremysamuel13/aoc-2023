use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part2 {
    race: Race,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        let [time, distance] = input
            .flat_map(|line| {
                line.replace(" ", "")
                    .split_once(":")
                    .expect("Failed to split at color")
                    .1
                    .parse::<isize>()
            })
            .collect::<Vec<_>>()[..]
        else {
            panic!("Only expected two numbers from input")
        };

        Ok(Self {
            race: Race::from((&time, &distance)),
        })
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self.race.possible_wins().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "71503");

        Ok(())
    }
}
