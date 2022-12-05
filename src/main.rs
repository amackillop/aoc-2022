mod common;
mod day1;
mod day2;
mod day3;
mod day4;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day1()?;
    day2()?;
    day3()?;
    day4()
}
