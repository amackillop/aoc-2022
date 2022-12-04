use std::{collections::HashSet, error::Error};

use crate::common;

type AocResult<T> = Result<T, Box<dyn Error>>;

pub fn day3() -> AocResult<()> {
    println!("~~~~~~~~~~~~~ Day 3 ~~~~~~~~~~~~~");
    println!("Part 1: {:?}", part1()?);
    println!("Part 2: {}", part2()?);
    Ok(())
}

// Input: GwrhJPDJCZFRcwfZWV represents two compartments
// Length is even number
// First half represents first compartment, second half represents second
// Create two sets and take the intersection to find the item in each
// Then calculate and sum the priorities
// a -> 1 A -> 27
// u8 reps are 97 and 65
fn part1() -> AocResult<u16> {
    let total = parse_input1()?
        .iter()
        .map(|(first, second)| {
            if let Some(item) = find_item(first, second) {
                to_priority(&item)
            } else {
                0
            }
        })
        .sum();
    Ok(total)
}

fn parse_input1() -> AocResult<Vec<(HashSet<u8>, HashSet<u8>)>> {
    common::get_input_lines("day3")?
        .flat_map(|res| {
            res.map(|line| {
                let half = line.len() / 2;
                let first = line.bytes().take(half).collect();
                let second = line.bytes().skip(half).collect();
                Ok((first, second))
            })
        })
        .collect()
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
fn part2() -> AocResult<u16> {
    let total = parse_input2()?[..]
        .chunks(3)
        .map(|sets| {
            // This should always match given correct input
            if let [first, second, third] = sets {
                if let Some(badge) = find_badge(first, second, third) {
                    to_priority(&badge)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    Ok(total)
}

fn parse_input2() -> AocResult<Vec<HashSet<u8>>> {
    common::get_input_lines("day3")?
        .flat_map(|res| res.map(|line| Ok(line.bytes().collect())))
        .collect()
}

fn find_badge(first: &HashSet<u8>, second: &HashSet<u8>, third: &HashSet<u8>) -> Option<u8> {
    let first_intersection: HashSet<u8> = first.intersection(&second).cloned().collect();
    first_intersection
        .intersection(&third)
        .next()
        .map(|item| *item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_gives_correct_answer() {
        if let Ok(answer) = part1() {
            assert_eq!(answer, 8298)
        } else {
            panic!("Bad input.")
        }
    }

    #[test]
    fn test_part_2_gives_correct_answer() {
        if let Ok(answer) = part2() {
            assert_eq!(answer, 2708)
        } else {
            panic!("Bad input.")
        }
    }
}
