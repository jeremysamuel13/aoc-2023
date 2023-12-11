use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part2 {}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        todo!()
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "");

        Ok(())
    }
}
