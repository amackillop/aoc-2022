use std::{
    fs::File,
    io::{self, BufRead}, error::Error
};

// Result alias with an implicit boxed error to clean up type sigs
pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

// Returns an Iterator to the Reader of the lines of the file.
pub fn get_input_lines(day: &str) -> Result<impl Iterator<Item = String>> {
    let file = File::open(format!("./input/{day}.txt"))?;
    Ok(io::BufReader::new(file).lines().map(|l| l.unwrap()))
}
