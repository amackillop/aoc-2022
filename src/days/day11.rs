use crate::days::common::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, fs};

const INPUT: &str = "./input/day11.txt";

pub fn solution<'a>() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 11 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", part_one(&input, 20, 3));
    Ok(())
}

fn part_one(input: &str, rounds: u16, divisor: u32) -> u128 {
    let lines: Vec<&str> = input.lines().collect();
    let monkeys: Vec<RefCell<Monkey>> = lines
        .chunks(7)
        .flat_map(parse_monkey)
        .map(|monkey| RefCell::new(monkey))
        .collect();
    let mut inspection_counts: HashMap<usize, u128> = HashMap::new();
    for _ in 0..rounds {
        for monkey in monkeys.iter() {
            let mut monkey = monkey.borrow_mut();
            let items: Vec<u32> = monkey.items.drain(..).collect();
            for item in items {
                inspection_counts
                    .entry(monkey.id)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                let new_item: u32 = apply_op(&item, monkey.op) / divisor;
                if (new_item % monkey.divisor) == u32::from(0 as u8) {
                    monkeys[monkey.if_true].borrow_mut().items.push(new_item);
                } else {
                    monkeys[monkey.if_false].borrow_mut().items.push(new_item);
                }
            }
        }
    }
    let mut final_inpection_counts = inspection_counts.into_values().collect::<Vec<_>>();
    final_inpection_counts.sort_by(|a, b| b.cmp(a));
    final_inpection_counts
        .into_iter()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[derive(PartialEq, Debug)]
struct Monkey {
    id: usize,
    items: Vec<u32>,
    op: Operation,
    divisor: u32,
    if_true: usize,
    if_false: usize,
}

fn parse_monkey(input: &[&str]) -> Option<Monkey> {
    lazy_static! {
        static ref NUMBERS: Regex = Regex::new(r"(\d+)").unwrap();
        static ref OPERATION: Regex = Regex::new(r"(\+|\*) (\d+|old)").unwrap();
    }
    let mut input = input.iter();
    let id = NUMBERS.find(input.next()?)?.as_str().parse().ok()?;
    let items: Vec<u32> = NUMBERS
        .find_iter(input.next()?)
        .flat_map(|num| num.as_str().parse().ok())
        .collect();
    let captures = OPERATION.captures(input.next()?)?;
    let op = match (captures.get(1)?.as_str(), captures.get(2)?.as_str()) {
        ("*", "old") => Some(Operation::SQUARE),
        ("*", value @ _) => Some(Operation::MULTIPLY(value.parse().ok()?)),
        ("+", value @ _) => Some(Operation::ADD(value.parse().ok()?)),
        _ => None,
    }?;
    let test_divisor = NUMBERS.find(input.next()?)?.as_str().parse().ok()?;
    let if_true = NUMBERS.find(input.next()?)?.as_str().parse().ok()?;
    let if_false = NUMBERS.find(input.next()?)?.as_str().parse().ok()?;
    Some(Monkey {
        id,
        items,
        op,
        divisor: test_divisor,
        if_true,
        if_false,
    })
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Operation {
    ADD(u32),
    MULTIPLY(u32),
    SQUARE,
}

fn apply_op(worry: &u32, operation: Operation) -> u32 {
    match operation {
        Operation::ADD(value) => worry + value,
        Operation::MULTIPLY(value) => worry * value,
        Operation::SQUARE => worry.pow(2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_one(&input, 20, 3), 99840);
        Ok(())
    }

    #[test]
    fn test_parse_monkey() -> Result<()> {
        let chunk = [
            "Monkey 0:",
            "Starting items: 79, 98",
            "Operation: new = old * 19",
            "Test: divisible by 23",
            "  If true: throw to monkey 2",
            "  If false: throw to monkey 3",
            "",
        ];
        assert_eq!(
            parse_monkey(&chunk),
            Some(Monkey {
                id: 0,
                items: vec![79, 98],
                op: Operation::MULTIPLY(19),
                divisor: 23,
                if_true: 2,
                if_false: 3
            })
        );
        Ok(())
    }
}
