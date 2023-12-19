#![allow(dead_code)]

use itertools::Itertools;
use std::{collections::BTreeMap, ops::Range};

#[derive(Clone, Debug, Default)]
pub struct Entry {
    end: usize,
    offset: isize,
}

impl Entry {
    pub fn parse_line(line: &String) -> (usize, Self) {
        let nums = line
            .split_whitespace()
            .flat_map(str::parse)
            .collect::<Vec<usize>>();

        let [dst, src, len] = nums[..] else {
            panic!("EXPECTED 3 ELEMENTS IN LINE")
        };

        (
            src,
            Self {
                end: (src + len),
                offset: dst as isize - src as isize,
            },
        )
    }

    pub fn contains(&self, start: usize, v: &usize) -> bool {
        (start..self.end).contains(v)
    }

    pub fn translate(&self, v: usize) -> usize {
        (v as isize + self.offset) as usize
    }
}

#[derive(Clone, Debug)]
pub struct SeedMap {
    entries: BTreeMap<usize, Entry>,
}

impl SeedMap {
    pub fn parse_map(lines: &[String]) -> Self {
        Self {
            entries: lines[1..]
                .iter()
                .map(Entry::parse_line)
                .sorted_by_key(|(start, _)| *start)
                .collect(),
        }
    }

    fn get(&self, v: usize) -> Option<&Entry> {
        if let Some(entry) = self.entries.get(&v) {
            return Some(entry);
        }
        let range = self.entries.range(..v);
        if let Some((&start, entry)) = range.last() {
            let range = start..entry.end;
            if range.contains(&v) {
                return Some(entry);
            }
        }
        None
    }

    fn search_closest(&self, v: usize) -> Result<(&usize, &Entry), Option<(&usize, &Entry)>> {
        if let Some((start, entry)) = self.entries.range(..=v).next_back() {
            if entry.contains(*start, &v) {
                return Ok((start, entry));
            }
        }

        Err(self.entries.range(v..).next())
    }

    pub fn translate(&self, v: usize) -> usize {
        self.get(v).map(|entry| entry.translate(v)).unwrap_or(v)
    }

    pub fn translate_range(&self, range: Range<usize>) -> TranslateRangeIntervalTreeIterator<'_> {
        TranslateRangeIntervalTreeIterator {
            start: range.start,
            end: range.end,
            map: self,
        }
    }
}

pub struct TranslateRangeIntervalTreeIterator<'a> {
    start: usize,
    end: usize,
    map: &'a SeedMap,
}

impl<'a> Iterator for TranslateRangeIntervalTreeIterator<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let result;

        match self.map.search_closest(self.start) {
            Ok((_, entry)) => {
                // translate sub-range
                result = (entry.translate(self.start))..(entry.translate(entry.end.min(self.end)));
                self.start = entry.end;
            }
            Err(Some((&entry_start, _))) => {
                // no translation
                result = self.start..entry_start;
                self.start = entry_start;
            }
            Err(None) => {
                result = self.start..self.end;
                self.start = self.end;
            }
        }

        Some(result)
    }
}
