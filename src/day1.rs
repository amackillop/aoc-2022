use std::error::Error;

use crate::common;

pub fn day1() -> Result<(), Box<dyn Error>> {
    println!("~~~~~~~~~~~~~ Day 1 ~~~~~~~~~~~~~");
    let calorie_counts = parse_input("day1");
    println!("Part 1: {:?}", part1(&calorie_counts)?);
    println!("Part 2: {:?}", part2(calorie_counts));
    Ok(())
}

// Find the maximum calorie count
fn part1(calorie_counts: &Vec<i32>) -> Result<&i32, &str> {
    calorie_counts.iter().max().ok_or("Empty calorie_counts.")
}

// Find the sum of the top 3 calorie counts
fn part2(mut calorie_counts: Vec<i32>) -> i32 {
    calorie_counts.sort_unstable_by(|a, b| b.cmp(a));
    calorie_counts.iter().take(3).sum::<i32>()
}

/*
Parse input into calorie count totals. For example:
100,33, ,25,25,50, ,10 --> [133, 75, 10]

Panics on missing file or bad input.
*/
fn parse_input(day: &str) -> Vec<i32> {
    common::get_input_lines(day)
        .unwrap()
        .fold((vec![], 0), |(mut acc, total), line| {
            if let Ok(count) = line.unwrap().parse::<i32>() {
                (acc, total + count)
            } else {
                acc.push(total);
                (acc, 0)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_gives_correct_answer() {
        let calorie_counts = parse_input("day1");

        assert!(part1(&calorie_counts) == Ok(&69836));
    }

    #[test]
    fn test_part_2_gives_correct_answer() {
        let calorie_counts = parse_input("day1");

        assert!(part2(calorie_counts) == 207968);
    }

}

