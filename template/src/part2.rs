use aoc_core::*;

#[derive(Clone)]
pub struct Part2 {}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> Result<Self, AOCError> {
        todo!()
    }
}

impl Part2 {
    pub fn solve(&mut self) -> Result<String, AOCError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> Result<(), AOCError> {
        let input = "";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "");

        Ok(())
    }
}
