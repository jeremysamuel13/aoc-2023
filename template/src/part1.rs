use aoc_core::*;

#[derive(Clone)]
pub struct Part1 {}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> Result<Self, AOCError> {
        todo!()
    }
}

impl Part1 {
    pub fn solve(&mut self) -> Result<String, AOCError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<(), AOCError> {
        let input = "";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "");

        Ok(())
    }
}
