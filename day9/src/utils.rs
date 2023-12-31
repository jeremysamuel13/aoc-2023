use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Report {
    entries: Vec<History>,
}

impl Report {
    pub fn from_iter<I: Iterator<Item = String>>(iter: I) -> Self {
        Self {
            entries: iter.map(History::from_line).collect(),
        }
    }

    pub fn from_iter_rev<I: Iterator<Item = String>>(iter: I) -> Self {
        Self {
            entries: iter.map(History::from_line_rev).collect(),
        }
    }

    pub fn solve(&self) -> i64 {
        self.entries.iter().map(History::next_value).sum()
    }
}

#[derive(Clone, Debug)]
pub struct History {
    history: Vec<i64>,
}

impl FromIterator<i64> for History {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let history = iter.into_iter().collect();
        Self { history }
    }
}

impl History {
    pub fn from_line(s: String) -> Self {
        s.split_whitespace().flat_map(str::parse).collect()
    }

    pub fn from_line_rev(s: String) -> Self {
        s.split_whitespace().flat_map(str::parse).rev().collect()
    }

    pub fn get_sub_history(&self) -> Self {
        self.history
            .iter()
            .tuple_windows()
            .map(|(l, r)| r - l)
            .collect()
    }

    pub fn is_terminal(&self) -> bool {
        self.history.iter().all(|v| *v == 0)
    }

    pub fn next_value(&self) -> i64 {
        if self.is_terminal() {
            return 0;
        }

        let last = self.history.last().expect("No elements found in array?");
        let subhist = self.get_sub_history();
        let sublast = subhist.next_value();

        last + sublast
    }
}
