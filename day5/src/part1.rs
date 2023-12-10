use crate::utils::*;
use aoc_core::*;

#[derive(Clone)]
pub struct Part1 {
    seeds: Vec<usize>,
    maps: Vec<SeedMap>,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(mut input: T) -> AOCResult<Self> {
        let seeds: Vec<usize> = input
            .next()
            .expect("Failed to get seeds")
            .trim()
            .split_once(':')
            .expect("Failed to split at `:`")
            .1
            .split_whitespace()
            .flat_map(str::parse)
            .collect();

        let maps = input
            .skip(1)
            .collect::<Vec<_>>()
            .split(|s| s.trim() == "")
            .map(SeedMap::parse_map)
            .collect::<Vec<_>>();

        Ok(Self { seeds, maps })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |acc, map| map.translate(acc)))
            .min()
            .expect("No location found")
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
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

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "35");

        Ok(())
    }
}
