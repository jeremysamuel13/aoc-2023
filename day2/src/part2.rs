use aoc_core::*;

use crate::part1::{Game, Part1, Rgb};

#[derive(Clone)]
pub struct Part2 {
    games: Vec<Game>,
}

impl Game {
    pub fn get_minimum_rgb(&self) -> Rgb {
        Rgb {
            red: self
                .subsets
                .iter()
                .max_by_key(|v| v.red)
                .expect("No subsets?")
                .red,
            green: self
                .subsets
                .iter()
                .max_by_key(|v| v.green)
                .expect("No subsets?")
                .green,
            blue: self
                .subsets
                .iter()
                .max_by_key(|v| v.blue)
                .expect("No subsets?")
                .blue,
        }
    }
}

impl Rgb {
    pub fn power(&self) -> u128 {
        self.red as u128 * self.green as u128 * self.blue as u128
    }
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            games: Part1::parse_from(input)?.games,
        })
    }
}

impl Part2 {
    pub fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .games
            .iter()
            .map(|g| g.get_minimum_rgb().power())
            .sum::<u128>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "2286");

        Ok(())
    }
}
