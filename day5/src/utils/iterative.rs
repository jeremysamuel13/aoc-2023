#![allow(dead_code)]

use itertools::Itertools;
use std::{cmp::Ordering, iter::Peekable, ops::Range, slice::Iter};

#[derive(Clone, Debug)]
pub struct Entry {
    range: Range<usize>,
    offset: isize,
}

impl Entry {
    pub fn parse_line(line: &String) -> Self {
        let nums = line
            .split_whitespace()
            .flat_map(str::parse)
            .collect::<Vec<usize>>();

        let [dst, src, len] = nums[..] else {
            panic!("EXPECTED 3 ELEMENTS IN LINE")
        };

        Self {
            range: src..(src + len),
            offset: dst as isize - src as isize,
        }
    }

    pub fn translate(&self, v: usize) -> usize {
        (v as isize + self.offset) as usize
    }
}

#[derive(Clone, Debug)]
pub struct SeedMap {
    entries: Vec<Entry>,
}

impl SeedMap {
    pub fn parse_map(lines: &[String]) -> Self {
        Self {
            entries: lines[1..]
                .iter()
                .map(Entry::parse_line)
                .sorted_by_key(|e| e.range.start)
                .collect(),
        }
    }

    fn binary_search(&self, v: usize, index_start: usize) -> Result<usize, usize> {
        self.entries[index_start..].binary_search_by(|entry| {
            if entry.range.contains(&v) {
                return Ordering::Equal;
            }

            if v < entry.range.start {
                Ordering::Greater
            } else if v >= entry.range.end {
                Ordering::Less
            } else {
                unreachable!()
            }
        })
    }

    fn get(&self, v: usize) -> Option<&Entry> {
        self.binary_search(v, 0).map(|idx| &self.entries[idx]).ok()
    }

    pub fn translate(&self, v: usize) -> usize {
        self.get(v).map(|entry| entry.translate(v)).unwrap_or(v)
    }

    pub fn translate_range(&self, range: Range<usize>) -> TranslateRangeIterator<'_> {
        TranslateRangeIterator {
            start: range.start,
            end: range.end,
            iter: self.entries.iter().peekable(),
        }
    }
}

pub struct TranslateRangeIterator<'a> {
    start: usize,
    end: usize,
    iter: Peekable<Iter<'a, Entry>>,
}

impl<'a> Iterator for TranslateRangeIterator<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        self.iter
            .find_map(|entry| {
                if entry.range.contains(&self.start) {
                    let end_min = entry.range.end.min(self.end);
                    let result = (entry.translate(self.start))..(entry.translate(end_min));
                    self.start = end_min;
                    Some(result)
                } else if self.start < entry.range.start {
                    let end_min = self.end.min(entry.range.start);
                    let result = self.start..end_min;
                    self.start = end_min;
                    Some(result)
                } else {
                    None
                }
            })
            .or_else(|| {
                let result = self.start..self.end;
                self.start = self.end;
                Some(result)
            })
    }
}
