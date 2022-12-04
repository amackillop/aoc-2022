use crate::common::{get_input_lines, AocResult};

pub fn day1() -> AocResult<()> {
    println!("~~~~~~~~~~~~~ Day 1 ~~~~~~~~~~~~~");
    let input = get_input_lines("day1")?;
    let calorie_counts = parse_input(input);
    println!("Part 1: {:?}", part1(&calorie_counts)?);
    println!("Part 2: {:?}", part2(calorie_counts));
    Ok(())
}

// Find the maximum calorie count
fn part1(calorie_counts: &Vec<u32>) -> Result<&u32, &str> {
    calorie_counts.iter().max().ok_or("Empty calorie_counts.")
}

// Find the sum of the top 3 calorie counts
fn part2(mut calorie_counts: Vec<u32>) -> u32 {
    calorie_counts.sort_unstable_by(|a, b| b.cmp(a));
    calorie_counts.iter().take(3).sum::<u32>()
}

/*
Parse input into calorie count totals. For example:
100,33, ,25,25,50, ,10 --> [133, 75, 10]

Panics on missing file or bad input.
*/
fn parse_input(input: impl Iterator<Item = String>) -> Vec<u32> {
    input
        .fold((vec![], 0), |(mut acc, total), line| {
            if let Ok(count) = line.parse::<u32>() {
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

    fn calorie_counts() -> AocResult<Vec<u32>> {
        Ok(parse_input(get_input_lines("day1")?))
    }

    #[test]
    fn test_part_1_gives_correct_answer() -> AocResult<()> {
        assert_eq!(part1(&calorie_counts()?)?, &69836);
        Ok(())
    }

    #[test]
    fn test_part_2_gives_correct_answer() -> AocResult<()> {
        assert_eq!(part2(calorie_counts()?), 207968);
        Ok(())
    }
}
