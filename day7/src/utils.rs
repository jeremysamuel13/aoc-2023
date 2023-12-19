use std::{
    cmp::{Ordering, Reverse},
    fmt::Debug,
    iter::zip,
};

use arrayvec::ArrayVec;

#[derive(Clone, Debug)]
pub struct Entry<const PART1: bool> {
    pub hand: Hand<PART1>,
    pub bid: usize,
}

impl<const PART1: bool> Entry<PART1> {
    pub fn parse_string(s: String) -> Self {
        let (hand, bid) = s.trim().split_once(' ').expect("Failed to split entry");

        Self {
            hand: Hand::from(hand),
            bid: bid.parse().expect("Failed to parse bid"),
        }
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Card<const PART1: bool> {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl<const PART1: bool> From<char> for Card<PART1> {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => {
                if PART1 {
                    Card::Jack
                } else {
                    Card::Joker
                }
            }
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum HandType {
    HighCard = 1,     // all distinct
    OnePair = 2,      // one matching pair, rest distinct
    TwoPair = 3,      // two matching pair, rest distinct
    ThreeOfAKind = 4, // three same, others distinct,
    FullHouse = 5,    // triple and a pair
    FourOfAKind = 6,  // quadruple and 1 distinct,
    FiveOfAKind = 7,  // all the same
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hand<const PART1: bool> {
    cards: ArrayVec<Card<PART1>, 5>,
}

impl<const PART1: bool> Hand<PART1> {
    pub fn hand_type(&self) -> HandType {
        let mut counts = [0; 5];

        let mut jokers = 0;

        for (i, c) in self.cards.iter().enumerate() {
            if !PART1 && c == &Card::<PART1>::Joker {
                jokers += 1;
                continue;
            }

            match self.cards[0..i].iter().position(|cj| c == cj) {
                Some(j) => counts[j] += 1,
                None => counts[i] += 1,
            };
        }

        counts.sort_by_key(|w| Reverse(*w));

        counts[0] += jokers;

        match counts {
            [5, 0, 0, 0, 0] => HandType::FiveOfAKind,
            [4, 1, 0, 0, 0] => HandType::FourOfAKind,
            [3, 2, 0, 0, 0] => HandType::FullHouse,
            [3, 1, 1, 0, 0] => HandType::ThreeOfAKind,
            [2, 2, 1, 0, 0] => HandType::TwoPair,
            [2, 1, 1, 1, 0] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    pub fn compare_hand(&self, o: &Self) -> Ordering {
        match self.hand_type().cmp(&o.hand_type()) {
            Ordering::Equal => zip(&self.cards, &o.cards)
                .map(|(l, r)| l.cmp(r))
                .find(|ord| ord.is_ne())
                .unwrap_or(Ordering::Equal),
            cmp => cmp,
        }
    }
}

impl<const PART1: bool> From<&str> for Hand<PART1> {
    fn from(cards: &str) -> Self {
        assert_eq!(cards.len(), 5);

        Self {
            cards: cards.chars().map(Card::from).collect(),
        }
    }
}

#[cfg(test)]
mod test_part_1 {
    use std::cmp::Ordering;

    type Hand = super::Hand<true>;

    #[test]
    fn test_cmp_hand_greater() {
        let h1 = Hand::from("33332");
        let h2 = Hand::from("2AAAA");

        assert_eq!(h1.compare_hand(&h2), Ordering::Greater)
    }

    #[test]
    fn test_cmp_hand_greater_2() {
        let h1 = Hand::from("77888");
        let h2 = Hand::from("77788");

        assert_eq!(h1.compare_hand(&h2), Ordering::Greater)
    }

    #[test]
    fn test_cmp_hand_less() {
        let h1 = Hand::from("AAAAA");
        let h2 = Hand::from("23232");

        assert_eq!(h2.compare_hand(&h1), Ordering::Less)
    }

    #[test]
    fn test_cmp_hand_eq() {
        let h1 = Hand::from("44332");
        let h2 = Hand::from("44332");

        assert_eq!(h2.compare_hand(&h1), Ordering::Equal)
    }
}

#[cfg(test)]
mod test_part_2 {
    use std::cmp::Ordering;

    type Hand = super::Hand<false>;

    #[test]
    fn test_joker() {
        let h1 = Hand::from("QQQQJ");
        let h2 = Hand::from("QQQQ2");

        assert_eq!(h1.compare_hand(&h2), Ordering::Greater)
    }
}

/* DEBUG SUFF */

impl<const PART1: bool> std::fmt::Debug for Hand<PART1> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hand: ")?;

        for c in &self.cards {
            c.fmt(f)?;
        }

        Ok(())
    }
}

impl<const PART1: bool> std::fmt::Debug for Card<PART1> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Joker => {
                if PART1 {
                    unreachable!()
                } else {
                    write!(f, "J")
                }
            }
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "T"),
            Self::Jack => {
                if !PART1 {
                    unreachable!()
                } else {
                    write!(f, "J")
                }
            }
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}
