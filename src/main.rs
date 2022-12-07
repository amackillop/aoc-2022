mod common;
mod days;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    days::day1::solution()?;
    days::day2::solution()?;
    days::day3::solution()?;
    days::day4::solution()?;
    days::day5::solution()?;
    days::day6::solution()
}
