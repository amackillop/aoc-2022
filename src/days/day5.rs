use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::iter::from_fn;
use std::{collections::VecDeque, ops::Add};
use std::rc::Rc;

use crate::common::{self, Result};
use lazy_static::lazy_static;
use regex::Regex;

type Crate = char;

type Amount = usize;
type From = usize;
type To = usize;
type Step = (Amount, From, To);

pub fn solution() -> Result<()> {
    let day = "day5";
    let crate_lines = common::get_input_lines(day)?.take_while(|line| line != "");
    let steps_lines = common::get_input_lines(day)?.skip_while(|line| !line.starts_with("move"));
    let stacks = parse_stacks(crate_lines.collect()).ok_or("Invalid input.")?;
    let steps = steps_lines.flat_map(parse_step);
    println!("{}", compute_outcome(stacks, steps));
    Ok(())
}

// Parse:
//     [D]
// [N] [C]     -->  [['Z', 'N'], ['M', 'C', 'D'], ['P']]
// [Z] [M] [P]
//  1   2   3
fn parse_stacks(crate_lines: Vec<String>) -> Option<Vec<VecDeque<Crate>>> {
    let (index_line, crate_lines) = crate_lines.split_last()?;
    let width = index_line.chars().next_back()?.to_digit(10)?;
    let mut stacks = vec![VecDeque::<Crate>::new(); width as usize];
    for line in crate_lines {
        for (index, maybe_crate) in parse_crate_line(line).iter().enumerate() {
            if let Some(crate_) = maybe_crate {
                stacks[index].push_front(*crate_);
            }
        }
    }
    Some(stacks)
}

// Parse: "    [G] [R]     [P]" into [None, Some('G'), Some('R'), None, Some('P')]
fn parse_crate_line(line: &String) -> Vec<Option<Crate>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s{4})|(\w{1})").unwrap();
    }
    RE.captures_iter(&line)
        .map(|c| match &c[0] {
            "    " => None,
            _ => c[0].chars().next(),
        })
        .collect()
}

// Parse "move 1 from 2 to 3" into (1, 1, 2). (Converting to zero-index values)
fn parse_step(step_line: String) -> Option<Step> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let mut numbers = RE.find_iter(&step_line).flat_map(|m| m.as_str().parse());
    Some((numbers.next()?, numbers.next()? - 1, numbers.next()? - 1))
}

fn compute_outcome(
    stacks: Vec<VecDeque<Crate>>,
    steps: impl Iterator<Item = Step>,
) -> String {
    let stacks: Vec<RefCell<VecDeque<Crate>>> = stacks.into_iter().map(|s| RefCell::new(s)).collect();
    let final_stacks = steps.fold(stacks, |stacks, step| {
        let (amount, from, to) = step;
        // Part 1
        // for _ in 0 .. amount {
            // stacks[to].borrow_mut().push_back(stacks[from].borrow_mut().pop_back().unwrap());
    //  }
        let index = stacks[from].borrow().len() - amount;
        let mut moved = stacks[from].borrow_mut().split_off(index);
        stacks[to].borrow_mut().append(&mut moved);
        stacks
    });

    let mut outcome = "".to_string();
    for stack in final_stacks.into_iter() {
        outcome.push(*stack.take().back().unwrap());
    }
    outcome
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
            parse_crate_line(&line),
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
            compute_outcome(stacks, steps.into_iter()),
            "CMZ"
        );
        Ok(())
    }
}
