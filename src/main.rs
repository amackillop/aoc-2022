mod common;
mod day1;

use crate::day1::day1;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    day1()
}
