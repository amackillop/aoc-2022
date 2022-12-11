use crate::days::common::{self, Result};

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 2 ~~~~~~~~~~~~~");
    let lines = common::get_input_lines("day2")?;
    println!("Part 1: {}", total_score(lines.map(first_parser)));
    let lines = common::get_input_lines("day2")?;
    println!("Part 2: {}", total_score(lines.map(second_parser)));
    Ok(())
}

// Parse an individual line into a Round using the first interpretation
fn first_parser(line: String) -> Option<Round> {
    if let Some((opponent, myself)) = match line.as_str() {
        "A X" => Some((Selection::Rock, Selection::Rock)),
        "A Y" => Some((Selection::Rock, Selection::Paper)),
        "A Z" => Some((Selection::Rock, Selection::Scissors)),
        "B X" => Some((Selection::Paper, Selection::Rock)),
        "B Y" => Some((Selection::Paper, Selection::Paper)),
        "B Z" => Some((Selection::Paper, Selection::Scissors)),
        "C X" => Some((Selection::Scissors, Selection::Rock)),
        "C Y" => Some((Selection::Scissors, Selection::Paper)),
        "C Z" => Some((Selection::Scissors, Selection::Scissors)),
        _ => None,
    } {
        Some(Round { myself, opponent })
    } else {
        None
    }
}

// Parse an individual line into a Round using the second interpretation
fn second_parser(line: String) -> Option<Round> {
    if let Some((opponent, outcome)) = match line.as_str() {
        "A X" => Some((Selection::Rock, Outcome::Lose)),
        "A Y" => Some((Selection::Rock, Outcome::Draw)),
        "A Z" => Some((Selection::Rock, Outcome::Win)),
        "B X" => Some((Selection::Paper, Outcome::Lose)),
        "B Y" => Some((Selection::Paper, Outcome::Draw)),
        "B Z" => Some((Selection::Paper, Outcome::Win)),
        "C X" => Some((Selection::Scissors, Outcome::Lose)),
        "C Y" => Some((Selection::Scissors, Outcome::Draw)),
        "C Z" => Some((Selection::Scissors, Outcome::Win)),
        _ => None,
    } {
        Some(Round {
            myself: my_selection(&opponent, &outcome),
            opponent,
        })
    } else {
        None
    }
}

/*
Find the selction to satisify the outcome. Could embed this logic directly in
the decoding but this is more clear.
*/
fn my_selection(opponent: &Selection, outcome: &Outcome) -> Selection {
    match (opponent, outcome) {
        (Selection::Rock, Outcome::Win) => Selection::Paper,
        (Selection::Rock, Outcome::Lose) => Selection::Scissors,
        (Selection::Paper, Outcome::Win) => Selection::Scissors,
        (Selection::Paper, Outcome::Lose) => Selection::Rock,
        (Selection::Scissors, Outcome::Win) => Selection::Rock,
        (Selection::Scissors, Outcome::Lose) => Selection::Paper,
        // Rest are draws
        _ => *opponent,
    }
}

struct Round {
    myself: Selection,
    opponent: Selection,
}

#[derive(Copy, Clone)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn choice_score(selection: &Selection) -> i32 {
    return match selection {
        Selection::Rock => 1,
        Selection::Paper => 2,
        Selection::Scissors => 3,
    };
}

fn outcome_score(myself: &Selection, opponent: &Selection) -> i32 {
    match (myself, opponent) {
        (Selection::Rock, Selection::Paper) => 0,
        (Selection::Rock, Selection::Scissors) => 6,
        (Selection::Paper, Selection::Rock) => 6,
        (Selection::Paper, Selection::Scissors) => 0,
        (Selection::Scissors, Selection::Rock) => 0,
        (Selection::Scissors, Selection::Paper) => 6,
        // Rest are draws
        _ => 3,
    }
}

fn total_score(rounds: impl Iterator<Item = Option<Round>>) -> i32 {
    rounds
        .map(|round| {
            if let Some(Round { myself, opponent }) = round {
                choice_score(&myself) + outcome_score(&myself, &opponent)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day2")?;
        assert_eq!(total_score(lines.map(first_parser)), 14264);
        Ok(())
    }

    #[test]
    fn test_part_2_gives_correct_answer() -> Result<()> {
        let lines = common::get_input_lines("day2")?;
        assert_eq!(total_score(lines.map(second_parser)), 12382);
        Ok(())
    }
}
