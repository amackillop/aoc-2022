mod common;
mod day1;
mod day2;

use crate::day1::day1;
use crate::day2::day2;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day1();
    day2()
}
