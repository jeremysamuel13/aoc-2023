use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part1 {
    map: Map,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            map: Map::parse_from_iter(input),
        })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self.map.steps("AAA".to_string()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        dbg!(&parsed);
        assert_eq!(parsed.solve()?, "6");

        Ok(())
    }

    #[test]
    fn second_test_part_1() -> AOCResult<()> {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        dbg!(&parsed);
        assert_eq!(parsed.solve()?, "2");

        Ok(())
    }
}
