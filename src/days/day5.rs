use std::collections::VecDeque;

use crate::common::{self, Result};
use lazy_static::lazy_static;
use regex::Regex;

type Crate = char;
type Stacks = Vec<VecDeque<Crate>>;

type Amount = usize;
type From = usize;
type To = usize;
type Move = (Amount, From, To);

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 5 ~~~~~~~~~~~~~");
    let day = "day5";

    let crate_lines = common::get_input_lines(day)?.take_while(|line| line != "");
    let steps_lines = common::get_input_lines(day)?.skip_while(|line| !line.starts_with("move"));
    let stacks = parse_stacks(crate_lines.collect()).ok_or("Invalid input.")?;
    let moves = steps_lines.flat_map(parse_step);
    println!("Part 1: {}", compute_outcome(part1_move_op, stacks, moves));

    let crate_lines = common::get_input_lines(day)?.take_while(|line| line != "");
    let steps_lines = common::get_input_lines(day)?.skip_while(|line| !line.starts_with("move"));
    let stacks = parse_stacks(crate_lines.collect()).ok_or("Invalid input.")?;
    let moves = steps_lines.flat_map(parse_step);
    println!("Part 2: {}", compute_outcome(part2_move_op, stacks, moves));
    Ok(())
}

// Crates can only be moved one at a time
fn part1_move_op(mut stacks: Stacks, move_: Move) -> Stacks {
    let (amount, from, to) = move_;
    for _ in 0..amount {
        let crate_ = stacks[from].pop_back().unwrap();
        stacks[to].push_back(crate_);
    }
    stacks
}

// Can move N crates at once
fn part2_move_op(mut stacks: Stacks, move_: Move) -> Stacks {
    let (amount, from, to) = move_;
    let index = stacks[from].len() - amount;
    let mut moved = stacks[from].split_off(index);
    stacks[to].append(&mut moved);
    stacks
}

// Parse:
//     [D]
// [N] [C]     -->  [['Z', 'N'], ['M', 'C', 'D'], ['P']]
// [Z] [M] [P]
//  1   2   3
fn parse_stacks(crate_lines: Vec<String>) -> Option<Stacks> {
    let (index_line, crate_lines) = crate_lines.split_last()?;
    let width = index_line.chars().next_back()?.to_digit(10)?;
    let mut stacks = vec![VecDeque::<Crate>::new(); width as usize];
    for line in crate_lines {
        for (index, maybe_crate) in parse_crate_line(line).enumerate() {
            if let Some(crate_) = maybe_crate {
                stacks[index].push_front(crate_);
            }
        }
    }
    Some(stacks)
}

// Parse: "    [G] [R]     [P]" into [None, Some('G'), Some('R'), None, Some('P')]
fn parse_crate_line(line: &String) -> impl Iterator<Item = Option<Crate>> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s{4})|(\w{1})").unwrap();
    }
    RE.captures_iter(&line).map(|c| match &c[0] {
        "    " => None,
        _ => c[0].chars().next(),
    })
}

// Parse "move 1 from 2 to 3" into (1, 1, 2). (Converting to zero-index values)
fn parse_step(move_line: String) -> Option<Move> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    if let [amount, from, to] = RE
        .find_iter(&move_line)
        .flat_map(|m| m.as_str().parse())
        .collect::<Vec<usize>>()[..]
    {
        Some((amount, from - 1, to - 1))
    } else {
        None
    }
}

// Take the initial stacks, apply the moves, then return the string formed from
// the characters at the top of the stacks
fn compute_outcome<F>(move_op: F, stacks: Stacks, moves: impl Iterator<Item = Move>) -> String
where
    F: Fn(Stacks, Move) -> Stacks,
{
    let final_stacks = moves.fold(stacks, |stacks, move_| move_op(stacks, move_));
    final_stacks
        .iter()
        .fold("".to_string(), |mut outcome, stack| {
            outcome.push(*stack.back().unwrap());
            outcome
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_step() -> Result<()> {
        assert_eq!(
            parse_step("move 1 from 2 to 3".to_string()).unwrap(),
            (1, 1, 2)
        );
        Ok(())
    }

    #[test]
    fn test_parse_crate_line() -> Result<()> {
        let line = "    [G] [R]         [P]".to_string();
        assert_eq!(
            parse_crate_line(&line).collect::<Vec<Option<Crate>>>(),
            vec![None, Some('G'), Some('R'), None, None, Some('P')]
        );
        Ok(())
    }

    #[test]
    fn test_parse_stacks() -> Result<()> {
        let input = vec![
            "    [D]".to_string(),
            "[N] [C]".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3".to_string(),
        ];
        assert_eq!(
            parse_stacks(input),
            Some(vec![
                VecDeque::from(['Z', 'N']),
                VecDeque::from(['M', 'C', 'D']),
                VecDeque::from(['P'])
            ])
        );
        Ok(())
    }

    #[test]
    fn test_compute_outcome() -> Result<()> {
        let stacks = vec![
            VecDeque::from(['Z', 'N']),
            VecDeque::from(['M', 'C', 'D']),
            VecDeque::from(['P']),
        ];
        let steps = vec![(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)];
        assert_eq!(
            compute_outcome(part1_move_op, stacks, steps.into_iter()),
            "CMZ"
        );
        Ok(())
    }


    #[test]
    fn test_part_one() -> Result<()> {
        let day = "day5";
        let crate_lines = common::get_input_lines(day)?.take_while(|line| line != "");
        let steps_lines = common::get_input_lines(day)?.skip_while(|line| !line.starts_with("move"));
        let stacks = parse_stacks(crate_lines.collect()).ok_or("Invalid input.")?;
        let moves = steps_lines.flat_map(parse_step);
        assert_eq!(compute_outcome(part1_move_op, stacks, moves), "VCTFTJQCG");
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let day = "day5";
        let crate_lines = common::get_input_lines(day)?.take_while(|line| line != "");
        let steps_lines = common::get_input_lines(day)?.skip_while(|line| !line.starts_with("move"));
        let stacks = parse_stacks(crate_lines.collect()).ok_or("Invalid input.")?;
        let moves = steps_lines.flat_map(parse_step);
        assert_eq!(compute_outcome(part2_move_op, stacks, moves), "GCFGLDNJZ");
        Ok(())
    }


}
