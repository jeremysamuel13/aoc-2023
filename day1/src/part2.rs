use aoc_core::*;

#[derive(Clone)]
pub struct Part2 {
    lines: Vec<String>,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            lines: input.collect(),
        })
    }
}

impl Part2 {
    pub fn solve(&mut self) -> AOCResult<String> {
        const NUMS: [&'static str; 10] = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        Ok(self
            .lines
            .iter()
            .map(|line| {
                let mut line_iter = line.chars().enumerate().flat_map(|(line_idx, c)| {
                    c.to_digit(10).or_else(|| {
                        NUMS.iter().enumerate().find_map(|(num_idx, num)| {
                            line[line_idx..].starts_with(*num).then_some(num_idx as u32)
                        })
                    })
                });

                let first = line_iter.next().expect("Need at least one number");
                let last = line_iter.last().unwrap_or(first); // `line_iter.next_back()` is better but not implemented for this iter smh
                return first * 10 + last;
            })
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() -> AOCResult<()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, "281");

        Ok(())
    }
}
