use crate::common::Result;
use std::{collections::HashSet, fs};

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 6 ~~~~~~~~~~~~~");

    let input = fs::read_to_string(format!("./input/day6.txt"))?;
    println!(
        "Part 1: {}",
        find_first_marker(&input, 4).ok_or("No marker found.")?
    );
    println!(
        "Part 2: {}",
        find_first_marker(&input, 14).ok_or("No marker found.")?
    );
    Ok(())
}

fn find_first_marker(input: &String, buffer_size: usize) -> Option<usize> {
    for (index, buf) in input.as_bytes().windows(buffer_size).enumerate() {
        if buf.iter().collect::<HashSet<&u8>>().len() == buffer_size {
            return Some(index + buffer_size);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(format!("./input/day6.txt"))?;
        assert_eq!(find_first_marker(&input, 4), Some(1544));
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(format!("./input/day6.txt"))?;
        assert_eq!(find_first_marker(&input, 14), Some(2145));
        Ok(())
    }
}
