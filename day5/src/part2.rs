use std::ops::Range;

use crate::utils::*;

use aoc_core::*;
use itertools::Itertools;

#[derive(Clone)]
pub struct Part2 {
    seed_ranges: Vec<Range<usize>>,
    maps: Vec<SeedMap>,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(mut input: T) -> AOCResult<Self> {
        let seed_ranges = input
            .next()
            .expect("Failed to get seeds")
            .trim()
            .split_once(':')
            .expect("Failed to split at `:`")
            .1
            .split_whitespace()
            .flat_map(str::parse)
            .tuples()
            .map(|(start, len)| start..(start + len))
            .collect();

        // dbg!(&seed_ranges);
        let maps = input
            .skip(1)
            .collect::<Vec<_>>()
            .split(|s| s.trim() == "")
            .map(SeedMap::parse_map)
            .collect::<Vec<_>>();

        Ok(Self { seed_ranges, maps })
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .maps
            .iter()
            .fold(self.seed_ranges.clone(), |acc, map| {
                acc.into_iter()
                    .flat_map(|range| map.translate_range(range))
                    .collect()
            })
            .into_iter()
            .flatten()
            .min()
            .expect("No answer found???")
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "46");

        Ok(())
    }
}
