extern crate aoc_2022;
use aoc_2022::days::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day1::solution()?;
    day2::solution()?;
    day3::solution()?;
    day4::solution()?;
    day5::solution()?;
    day6::solution()?;
    day7::solution()?;
    day8::solution()?;
    day9::solution()?;
    day10::solution()?;
    day11::solution()?;
    day12::solution()
}
