use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part2 {
    map: Map,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            map: Map::parse_from_iter(input),
        })
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .map
            .graph
            .ghost_start()
            .cloned()
            .map(|node| self.map.steps(node))
            .lcm()
            .expect("Failed to get LCM")
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "6");

        Ok(())
    }
}
