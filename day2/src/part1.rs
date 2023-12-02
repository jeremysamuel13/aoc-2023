use aoc_core::*;
use std::{ops::Add, str::FromStr};
use strum::EnumString;

#[derive(Clone, Debug)]
pub struct Part1 {
    pub games: Vec<Game>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<RGB>,
}

#[derive(Clone, Debug, Default)]
pub struct RGB {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Add<&RGB> for RGB {
    type Output = RGB;

    fn add(self, rhs: &RGB) -> Self::Output {
        Self::Output {
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            red: self.red + rhs.red,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Red,
    Blue,
    Green,
}

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            games: input.map(Self::parse_game).collect(),
        })
    }
}

impl Part1 {
    fn parse_game(s: String) -> Game {
        let s = s.trim();
        let (game_prefix, subsets) = s.split_once(":").expect("No colon?");
        let id = game_prefix["Game ".len()..]
            .parse::<u32>()
            .expect("Failed to parse id");

        let subsets = subsets.trim().split(";").map(Self::parse_subset).collect();

        Game { id, subsets }
    }

    fn parse_subset(s: &str) -> RGB {
        s.split(",")
            .map(|ss| ss.trim().split_once(" ").expect("Failed to split"))
            .map(|(count, color)| {
                (
                    count.parse::<u32>().expect("Failed to parse"),
                    Color::from_str(color).expect("Failed to parse"),
                )
            })
            .fold(RGB::default(), |mut acc, (count, color)| {
                match color {
                    Color::Blue => acc.blue += count,
                    Color::Red => acc.red += count,
                    Color::Green => acc.green += count,
                };
                acc
            })
    }

    pub fn solve(&mut self) -> AOCResult<String> {
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
