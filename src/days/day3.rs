use std::collections::HashSet;

use crate::days::common::{self, Result};

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 3 ~~~~~~~~~~~~~");
    let lines = common::get_input_lines("day3")?;
    println!("Part 1: {}", part1(lines));
    let lines = common::get_input_lines("day3")?;
    println!("Part 2: {}", part2(lines));
    Ok(())
}

// Input: GwrhJPDJCZFRcwfZWV represents two compartments
// Length is even number
// First half represents first compartment, second half represents second
// Create two sets and take the intersection to find the item in each
// Then calculate and sum the priorities
// a -> 1 A -> 27
// u8 reps are 97 and 65
fn part1(lines: impl Iterator<Item = String>) -> u16 {
    lines
        .map(parse_compartments)
        .map(|(first, second)| {
            if let Some(item) = find_item(&first, &second) {
                to_priority(&item)
            } else {
                // Ignore bad input
                0
            }
        })
        .sum()
}

fn parse_compartments(line: String) -> (HashSet<u8>, HashSet<u8>) {
    let half = line.len() / 2;
    let first = line.bytes().take(half).collect();
    let second = line.bytes().skip(half).collect();
    (first, second)
}

fn find_item(first: &HashSet<u8>, second: &HashSet<u8>) -> Option<u8> {
    // Should only be one item
    first.intersection(second).next().map(|item| *item)
}

fn to_priority(byte: &u8) -> u16 {
    if byte.is_ascii_lowercase() {
        (byte - 96).into()
    } else {
        (byte - 38).into()
    }
}

// Input: GwrhJPDJCZFRcwfZWV represents a sack
// Create a set out of each sack
// Chunk the sets in groups of three and take the intersection to find the badge
// Then calculate and sum the priorities
fn part2(lines: impl Iterator<Item = String>) -> u16 {
    lines
        .map(|line| line.bytes().collect::<HashSet<u8>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|sets| {
            // This should always match given correct input
            if let Some(badge) = find_badge(sets) {
                to_priority(&badge)
            } else {
                0
            }
        })
        .sum()
}

fn find_badge(sets: &[HashSet<u8>]) -> Option<u8> {
    if let [first, second, third] = &sets[..] {
        let first_intersection: HashSet<u8> = first.intersection(second).cloned().collect();
        first_intersection
            .intersection(third)
            .next()
            .map(|item| *item)
    } else {
        None
    }
}

// Same as other one except uses iterator directly. Unfortunately there seems to
// be no direct method for chunking an iterator
fn part2_v2(lines: impl Iterator<Item = String>) -> u16 {
    lines
        .map(|line| line.bytes().collect::<HashSet<u8>>())
        .fold((0, vec![]), |(total, mut chunk), set| {
            chunk.push(set);
            if chunk.len() == 3 {
                if let Some(badge) = find_badge(&chunk) {
                    println!("{}", total + to_priority(&badge));
                    (total + to_priority(&badge), vec![])
                } else {
                    (total, vec![])
                }
            } else {
                (total, chunk)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day3")?;
        assert_eq!(part1(lines), 8298);
        Ok(())
    }

    #[test]
    fn test_part_2_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day3")?;
        assert_eq!(part2(lines), 2708);
        Ok(())
    }

    #[test]
    fn test_part_2_v2_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day3")?;
        assert_eq!(part2_v2(lines), 2708);
        Ok(())
    }
}
