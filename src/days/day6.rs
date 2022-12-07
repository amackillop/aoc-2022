use crate::common::Result;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};
extern crate test;

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

// Use a sliding window and simply check if the window contains distinct values
fn find_first_marker(input: &String, buffer_size: usize) -> Option<usize> {
    for (index, buf) in input.as_bytes().windows(buffer_size).enumerate() {
        if buf.iter().collect::<HashSet<&u8>>().len() == buffer_size {
            return Some(index + buffer_size);
        }
    }
    return None;
}

fn find_first_marker_optimized(input: &String, buffer_size: usize) -> Option<usize> {
    let mut unique = HashMap::<u8, usize>::new();
    for byte in input.bytes().take(buffer_size) {
        unique.entry(byte).and_modify(|c| *c += 1).or_insert(1);
    }

    // Use a FIFO
    let mut buf: VecDeque<u8> = input.bytes().take(buffer_size).collect();
    for (index, byte) in input.bytes().skip(buffer_size).enumerate() {
        if unique.len() == buffer_size {
            return Some(index + buffer_size);
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
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

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
