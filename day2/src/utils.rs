use std::{ops::Add, str::FromStr};
use strum::EnumString;

#[derive(Clone, Debug)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<Rgb>,
}

impl Game {
    pub fn parse_str(s: String) -> Self {
        let s = s.trim();
        let (game_prefix, subsets) = s.split_once(':').expect("No colon?");
        let id = game_prefix["Game ".len()..]
            .parse::<u32>()
            .expect("Failed to parse id");

        let subsets = subsets.trim().split(';').map(Rgb::parse_str).collect();

        Game { id, subsets }
    }

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

#[derive(Clone, Debug, Default)]
pub struct Rgb {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Add<&Rgb> for Rgb {
    type Output = Rgb;

    fn add(self, rhs: &Rgb) -> Self::Output {
        Self::Output {
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            red: self.red + rhs.red,
        }
    }
}

impl Rgb {
    fn parse_str(s: &str) -> Rgb {
        s.split(',')
            .map(|ss| ss.trim().split_once(' ').expect("Failed to split"))
            .map(|(count, color)| {
                (
                    count.parse::<u32>().expect("Failed to parse"),
                    Color::from_str(color).expect("Failed to parse"),
                )
            })
            .fold(Rgb::default(), |mut acc, (count, color)| {
                match color {
                    Color::Blue => acc.blue += count,
                    Color::Red => acc.red += count,
                    Color::Green => acc.green += count,
                };
                acc
            })
    }

    pub fn power(&self) -> u128 {
        self.red as u128 * self.green as u128 * self.blue as u128
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Red,
    Blue,
    Green,
}

pub const RED_COUNT: u32 = 12;
pub const GREEN_COUNT: u32 = 13;
pub const BLUE_COUNT: u32 = 14;
