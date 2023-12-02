#[macro_use]
pub mod macros;

use std::{
    error,
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Lines},
    iter::Flatten,
    path::PathBuf,
    time::{Duration, Instant},
};

use thiserror::Error;

pub struct AnswerInner<T> {
    pub answer: String,
    pub parsing: ParsedInput<T>,
    pub time: Duration,
}

pub struct Answer<A, B> {
    pub p1: AnswerInner<A>,
    pub p2: AnswerInner<B>,
}

impl<A, B> Answer<A, B> {
    pub fn print_answer(&self) {
        println!("Part 1:\n{}", self.p1.answer);
        println!("Part 2:\n{}", self.p2.answer);
    }
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
        let input2 = Input {
            path: inp2.canonicalize().expect("Invalid path"),
        };

        //Part 1
        let inp1 = self.parse_inp::<Self::Input1>(input1)?;
        let p1d = Instant::now();
        let ans1 = self.part_1(inp1.input.clone())?;
        let p1 = AnswerInner {
            answer: ans1.into(),
            parsing: inp1,
            time: p1d.elapsed(),
        };

        //Part 2
        let inp2 = self.parse_inp::<Self::Input2>(input2)?;
        let p2d = Instant::now();
        let ans2 = self.part_2(inp2.input.clone())?;
        let p2 = AnswerInner {
            answer: ans2.into(),
            parsing: inp2,
            time: p2d.elapsed(),
        };

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
