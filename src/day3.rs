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
        .map(|(first, second)| {
            if let Some(item) = find_item(&first, &second) {
                to_priority(&item)
            } else {
                0
            }
        })
        .sum();
    Ok(total)
}

fn parse_input1() -> AocResult<impl Iterator<Item = (HashSet<u8>, HashSet<u8>)>> {
    let iter = common::get_input_lines("day3")?.flat_map(|res| {
        res.map(|line| {
            let half = line.len() / 2;
            let first = line.bytes().take(half).collect();
            let second = line.bytes().skip(half).collect();
            (first, second)
        })
    });
    Ok(iter)
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
    let total = parse_input2()?.collect::<Vec<HashSet<u8>>>()[..]
        .chunks(3)
        .map(|sets| {
            // This should always match given correct input
            if let Some(badge) = find_badge(sets) {
                to_priority(&badge)
            } else {
                0
            }
        })
        .sum();
    Ok(total)
}

// Same as other one except uses iterator directly. Unfortunately there seems to
// be no direct method for chunking an iterator
fn part2_v2() -> AocResult<u16> {
    let (total, _) = parse_input2()?.fold((0, vec![]), |(total, mut chunk), set| {
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
    });
    Ok(total)
}

fn parse_input2() -> AocResult<impl Iterator<Item = HashSet<u8>>> {
    let iter =
        common::get_input_lines("day3")?.flat_map(|res| res.map(|line| line.bytes().collect()));
    Ok(iter)
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

    #[test]
    fn test_part_2_v2_gives_correct_answer() {
        if let Ok(answer) = part2_v2() {
            assert_eq!(answer, 2708)
        } else {
            panic!("Bad input.")
        }
    }
}
