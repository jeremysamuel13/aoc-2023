use derive_more::Deref;
use itertools::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Number {
        number: usize,
        row: usize,
        count: usize,
    },
    Symbol(char),
    Empty,
}

impl Element {
    pub fn take_single(
        iter: &mut (impl Iterator<Item = char> + PeekingNext),
        row: usize,
        count: usize,
    ) -> Option<Either<std::iter::Take<std::iter::Repeat<Element>>, std::iter::Once<Element>>> {
        let s = iter
            .peeking_take_while(char::is_ascii_digit)
            .collect::<String>();

        s.parse::<usize>()
            .ok()
            .map(|n| {
                Either::Left(
                    std::iter::repeat(Element::Number {
                        number: n,
                        row,
                        count,
                    })
                    .take(s.len()),
                )
            })
            .or_else(|| {
                iter.next().map(|v| {
                    Either::Right(std::iter::once(match v {
                        '.' => Element::Empty,
                        c => Element::Symbol(c),
                    }))
                })
            })
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Element::Symbol(_))
    }

    pub fn try_number(&self) -> Option<usize> {
        match self {
            Element::Number { number, .. } => Some(*number),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deref)]
pub struct Sequence(Vec<Element>);

impl Sequence {
    pub fn from_str_and_row(s: &str, row: usize) -> Self {
        let mut count = 0;
        Sequence(
            s.trim()
                .chars()
                .batching(|it| {
                    count += 1;
                    Element::take_single(it, row, count)
                })
                .flatten()
                .collect(),
        )
    }
}
