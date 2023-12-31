use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part1 {
    report: Report,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            report: Report::from_iter(input),
        })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self.report.solve().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "114");

        Ok(())
    }
}
