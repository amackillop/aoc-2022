use crate::common::Result;
use std::{collections::HashSet, fs};

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 6 ~~~~~~~~~~~~~");

    let input = fs::read_to_string(format!("./input/day6.txt"))?;
    println!("Part 1: {}", find_first_marker(&input, 4));
    println!("Part 2: {}", find_first_marker(&input, 14));
    Ok(())
}

fn find_first_marker(input: &String, buffer_size: usize) -> usize {
    let mut index = buffer_size;
    for buf in input.as_bytes().windows(buffer_size) {
        if buf.iter().collect::<HashSet<&u8>>().len() == buffer_size {
            break;
        } else {
            index += 1;
        }
    }
    index
}

#[test]
fn test_part_one() -> Result<()> {
    let input = fs::read_to_string(format!("./input/day6.txt"))?;
    assert_eq!(find_first_marker(&input, 4), 1544);
    Ok(())
}

#[test]
fn test_part_two() -> Result<()> {
    let input = fs::read_to_string(format!("./input/day6.txt"))?;
    assert_eq!(find_first_marker(&input, 14), 2145);
    Ok(())
}
