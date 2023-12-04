use std::collections::{hash_map::RandomState, hash_set::Intersection, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScratchCard {
    pub id: usize,
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl ScratchCard {
    pub fn parse_from_str(inp: &str) -> Self {
        let (card_identifier, rest) = inp.split_once(':').expect("Failed to split at ':'");

        let id = card_identifier
            .split_once(' ')
            .expect("Failed to split on whitespace")
            .1
            .trim()
            .parse()
            .expect("Failed to parse card id");

        let (winning, reg) = rest.split_once('|').expect("Failed to split at '|'");

        Self {
            id,
            numbers: Self::parse_numbers(reg),
            winning_numbers: Self::parse_numbers(winning),
        }
    }

    pub fn parse_numbers(nums: &str) -> HashSet<u32> {
        nums.split_whitespace()
            .map(|num| {
                num.parse::<u32>()
                    .unwrap_or_else(|_| panic!("Failed to parse {num} into a number"))
            })
            .collect()
    }

    pub fn calculate_winning_numbers(&self) -> Intersection<'_, u32, RandomState> {
        self.numbers.intersection(&self.winning_numbers)
    }
}
