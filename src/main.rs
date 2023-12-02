use std::env;

use aoc_core::*;
use day1::Day1;
use day2::Day2;
pub fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    let mut sols = days!(Day1, Day2);
    sols.get_mut(day)
        .unwrap()
        .solve()
        .expect(format!("Failed to run day {day}").as_str());
}
