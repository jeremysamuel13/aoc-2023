
use aoc_core::*;

#[derive(Clone)]
pub struct Part1 {
    lines: Vec<String>,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            lines: input.collect(),
        })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self
            .lines
            .iter()
            .map(|line| {
                let mut line_iter = line.chars().flat_map(|c| c.to_digit(10));

                let first = line_iter.next().expect("Need at least one number");
                let last = line_iter.next_back().unwrap_or(first); // `line_iter.next_back()` is better but not implemented for this iter smh
                first * 10 + last
            })
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> AOCResult<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "142");

        Ok(())
    }
}
