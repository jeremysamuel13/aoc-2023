use std::env;

use aoc_core::*;
use day1::Day1;
use day2::Day2;
use day3::Day3;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(0).expect("Please specify the day you want to run");

    let mut sols = days!(Day1, Day2, Day3);
    sols.get_mut(day)
        .unwrap()
        .solve()
        .unwrap_or_else(|_| panic!("Failed to run day {day}"));
}
