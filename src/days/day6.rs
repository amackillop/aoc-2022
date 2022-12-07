use crate::common::Result;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};
extern crate test;

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 6 ~~~~~~~~~~~~~");

    let input = fs::read_to_string(format!("./input/day6.txt"))?;
    println!("Part 1: {}", find_first_marker(&input, 4)?);
    println!("Part 2: {}", find_first_marker(&input, 14)?);
    Ok(())
}

// Iterate through sliding windows and simply check if the window contains
// distinct values using a hash set.
fn find_first_marker(input: &String, window_size: usize) -> Result<usize> {
    for (index, buf) in input.as_bytes().windows(window_size).enumerate() {
        if buf.iter().collect::<HashSet<&u8>>().len() == window_size {
            return Ok(index + window_size);
        }
    }
    return Err("No marker found.".into());
}

// Use a FIFO queue as the sliding window and a hash map to keep track of the
// count of each item in the queue. When the count goes to zero, remove the item
// from the hash map. If the length of the hash map matches the length of the
// queue, then all items in the queue must be distinct.
fn find_first_marker_optimized(input: &String, window_size: usize) -> Result<usize> {
    let mut unique = HashMap::<u8, usize>::new();
    for byte in input.bytes().take(window_size) {
        unique.entry(byte).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut buf: VecDeque<u8> = input.bytes().take(window_size).collect();
    for (index, byte) in input.bytes().skip(window_size).enumerate() {
        if unique.len() == window_size {
            return Ok(index + window_size);
        }

        let dropped = buf.pop_front().unwrap();
        unique.entry(dropped).and_modify(|c| *c -= 1);
        if let Some(count) = unique.get(&dropped) {
            if *count == 0 {
                unique.remove(&dropped);
            }
        }

        buf.push_back(byte);
        unique.entry(byte).and_modify(|c| *c += 1).or_insert(1);
    }
    return Err("No marker found.".into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(format!("./input/day6.txt"))?;
        assert_eq!(find_first_marker(&input, 4)?, 1544);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(format!("./input/day6.txt"))?;
        assert_eq!(find_first_marker(&input, 14)?, 2145);
        Ok(())
    }

    #[bench]
    fn bench_find_first_marker(b: &mut Bencher) -> Result<()> {
        let input = fs::read_to_string(format!("./input/day6.txt"))?;
        b.iter(|| find_first_marker(&input, 14));
        Ok(())
    }

    #[bench]
    fn bench_find_first_marker_optimized(b: &mut Bencher) -> Result<()> {
        let input = fs::read_to_string(format!("./input/day6.txt"))?;
        b.iter(|| find_first_marker_optimized(&input, 14));
        Ok(())
    }
}
