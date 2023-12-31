use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part1 {
    tiles: Tiles,
}

impl ParseInput for Part1 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            tiles: Tiles::parse(input),
        })
    }
}

impl Solvable for Part1 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self.tiles.get_max_dist().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &'static str, expected: &'static str) -> AOCResult<()> {
        let mut parsed = Part1::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, expected);

        Ok(())
    }

    #[test]
    fn test_1() -> AOCResult<()> {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";

        test(input, "4")?;

        Ok(())
    }

    #[test]
    fn test_2() -> AOCResult<()> {
        let input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

        test(input, "8")?;

        Ok(())
    }
}
