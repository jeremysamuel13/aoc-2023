use crate::utils::*;
use aoc_core::*;

#[derive(Clone)]
pub struct Part1 {}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        todo!()
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "");

        Ok(())
    }
}
