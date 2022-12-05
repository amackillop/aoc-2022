use crate::common::{self, Result};

pub fn day4() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 4 ~~~~~~~~~~~~~");
    let lines = common::get_input_lines("day4")?;
    println!("Part 1: {}", part1(lines));
    let lines = common::get_input_lines("day4")?;
    println!("Part 2: {}", part2(lines));
    Ok(())
}

// Input: 2-4,6-8 represents two ranges: [2, 3, 4] and [6, 7, 8]
// Need to find if either range is contained in the other
// Can compare boundaries to check. If the start and end of either range is
// between the start and end of the other than it is contained.
// Example:
//   S . . . E
//     S . E
fn part1(lines: impl Iterator<Item = String>) -> u16 {
    parse_ranges(lines).fold(0, |acc, ranges| {
        if let Some((first, second)) = ranges {
            if first.0 >= second.0 && first.1 <= second.1 {
                acc + 1
            } else if second.0 >= first.0 && second.1 <= first.1 {
                acc + 1
            } else {
                acc
            }
        } else {
            // Ignoring bad input
            acc
        }
    })
}

// now we need to find if there is any overlap at all between the ranges.
// Example:
//   S . . . E
//       S . . E
fn part2(lines: impl Iterator<Item = String>) -> u16 {
    parse_ranges(lines).fold(0, |acc, ranges| {
        if let Some((first, second)) = ranges {
            if first.0 >= second.0 && first.0 <= second.1 {
                acc + 1
            } else if first.1 >= second.0 && first.1 <= second.1 {
                acc + 1
            } else if second.0 >= first.0 && second.0 <= first.1 {
                acc + 1
            } else if second.1 >= first.0 && second.1 <= first.1 {
                acc + 1
            } else {
                acc
            }
        } else {
            // Ignoring bad input
            acc
        }
    })
}

// Parse "2-4,6-8" into ((2,4), (6,8))
fn parse_ranges(
    lines: impl Iterator<Item = String>,
) -> impl Iterator<Item = Option<((u8, u8), (u8, u8))>> {
    lines.map(|line| {
        let (first, second) = line.split_once(',')?;
        Some((parse_range(first)?, parse_range(second)?))
    })
}

fn parse_range(range_str: &str) -> Option<(u8, u8)> {
    let (start, stop) = range_str.split_once('-')?;
    Some((start.parse().ok()?, stop.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day4")?;
        assert_eq!(part1(lines), 651);
        Ok(())
    }

    #[test]
    fn test_part_2_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day4")?;
        assert_eq!(part2(lines), 956);
        Ok(())
    }
}
