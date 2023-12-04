#[macro_use]
pub mod macros;

use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Lines},
    iter::Flatten,
    path::PathBuf,
    time::{Duration, Instant},
};

pub struct AnswerInner<T> {
    pub answer: AOCResult<String>,
    pub parsing: ParsedInput<T>,
    pub time: Duration,
}

impl<T> AnswerInner<T> {
    pub fn print_answer<const PART: u8>(&self) {
        println!(
            "Part {PART}:\n{}",
            self.answer
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_else(|e| format!("Error running part 1: {e}"))
        );
    }
}

pub struct Answer<A, B> {
    pub p1: AnswerInner<A>,
    pub p2: AnswerInner<B>,
}

pub struct ParsedInput<T> {
    pub input: T,
    pub parse_time: Duration,
}

pub type AOCResult<T> = anyhow::Result<T>;

#[derive(Clone)]
pub struct Input {
    path: PathBuf,
}

impl Input {
    pub fn lines(&self) -> AOCResult<Flatten<Lines<BufReader<File>>>> {
        let file = File::open(self.path.clone()).unwrap();
        let reader = BufReader::new(file);

        Ok(reader.lines().flatten())
    }

    pub fn as_string(&self) -> AOCResult<String> {
        Ok(read_to_string(self.path.clone())?)
    }
}

pub trait Solution {
    type Input1: ParseInput + Clone;
    type Input2: ParseInput + Clone;

    const DAY: usize;

    fn part_1(&self, input: Self::Input1) -> AOCResult<String>;
    fn part_2(&self, input: Self::Input2) -> AOCResult<String>;

    fn parse_inp<T: ParseInput + Clone>(&self, input: Input) -> AOCResult<ParsedInput<T>> {
        let now = Instant::now();

        let input = T::parse_from(input.lines()?)?;

        let parse_time = now.elapsed();

        Ok(ParsedInput { input, parse_time })
    }

    fn solve(&self, inp1: PathBuf, inp2: PathBuf) -> AOCResult<Answer<Self::Input1, Self::Input2>> {
        //Read input
        let input1 = Input {
            path: inp1.canonicalize().expect("Invalid path"),
        };

        //Part 1
        let inp1 = self.parse_inp::<Self::Input1>(input1)?;
        let p1d = Instant::now();
        let ans1 = self.part_1(inp1.input.clone());
        let p1 = AnswerInner {
            answer: ans1,
            parsing: inp1,
            time: p1d.elapsed(),
        };

        p1.print_answer::<1>();

        let input2 = Input {
            path: inp2.canonicalize().expect("Invalid path"),
        };

        //Part 2
        let inp2 = self.parse_inp::<Self::Input2>(input2)?;
        let p2d = Instant::now();
        let ans2 = self.part_2(inp2.input.clone());
        let p2 = AnswerInner {
            answer: ans2,
            parsing: inp2,
            time: p2d.elapsed(),
        };

        p2.print_answer::<2>();

        Ok(Answer { p1, p2 })
    }
}

pub trait ParseInput
where
    Self: Sized,
{
    fn parse_from<T: Iterator<Item = String>>(input: T) -> AOCResult<Self>;

    fn parse_from_str<'a, T: Iterator<Item = &'a str>>(input: T) -> AOCResult<Self> {
        Self::parse_from(input.map(|v| v.to_owned()))
    }
}
