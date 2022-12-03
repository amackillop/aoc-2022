use core::fmt;
use std::{
    fs::File,
    io::{self, BufRead}, error::Error,
};

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
    pub fn boxed(msg: &str) -> Box<Self> {
        Box::new(ParseError(msg.into()))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse: {}", self.0)
    }
}

impl Error for ParseError {}

// Returns an Iterator to the Reader of the lines of the file.
pub fn get_input_lines(day: &str) -> io::Result<io::Lines<io::BufReader<File>>>{
    let file = File::open(format!("./input/{day}.txt"))?;
    Ok(io::BufReader::new(file).lines())
}

