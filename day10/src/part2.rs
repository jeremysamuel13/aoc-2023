use crate::utils::*;
use aoc_core::*;

#[derive(Clone, Debug)]
pub struct Part2 {
    tiles: Tiles,
}

impl ParseInput for Part2 {
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self> {
        Ok(Self {
            tiles: Tiles::parse(input),
        })
    }
}

impl Solvable for Part2 {
    fn solve(&mut self) -> AOCResult<String> {
        Ok(self.tiles.get_area().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &'static str, expected: &'static str) -> AOCResult<()> {
        let mut parsed = Part2::parse_from_str(input.lines())?;
        assert_eq!(parsed.solve()?, expected);

        Ok(())
    }

    #[test]
    fn test_1() -> AOCResult<()> {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

        test(input, "4")?;

        Ok(())
    }

    #[test]
    fn test_2() -> AOCResult<()> {
        let input = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";

        test(input, "8")?;

        Ok(())
    }

    #[test]
    fn test_3() -> AOCResult<()> {
        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

        test(input, "10")?;

        Ok(())
    }
}
