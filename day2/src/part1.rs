use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part1 {
    pub games: Vec<Game>,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            games: input.map(Game::parse_str).collect(),
        })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        let id_sum: u32 = self
            .games
            .iter()
            .filter(|game| {
                game.subsets.iter().all(|subset| {
                    subset.red <= RED_COUNT
                        && subset.green <= GREEN_COUNT
                        && subset.blue <= BLUE_COUNT
                })
            })
            .map(|g| g.id)
            .sum();

        Ok(id_sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "8");

        Ok(())
    }
}
