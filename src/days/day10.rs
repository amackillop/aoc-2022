use crate::days::common::Result;
use std::fs;

const INPUT: &str = "./input/day10.txt";

pub fn solution<'a>() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 9 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2:");
    part2(&input);
    Ok(())
}

fn part1(input: &str) -> i32 {
    let instructions = input.lines().flat_map(parse_instruction).flatten();
    let mut register = 1;
    let mut sum_of_signal_strengths = 0;
    for (cycle, instruction) in (1 as i32..).zip(instructions) {
        if (cycle + 20) % 40 == 0 {
            sum_of_signal_strengths += register * cycle;
        }
        if let Instruction::ADDX(amount) = instruction {
            register += amount;
        }
    }
    sum_of_signal_strengths
}

fn part2(input: &str) {
    let instructions = input.lines().flat_map(parse_instruction).flatten();
    let mut sprite = 0b11100 as u64;
    let mut draw_pos = 0b100 as u64;
    let mut draws: Vec<u64> = vec![];
    for instruction in instructions {
        draws.push(sprite & draw_pos);
        if draw_pos == (2 << 40) {
            let line = draws.drain(..).reduce(|a, b| a ^ b).unwrap() >> 2;
            println!("{}", format_output(line));
            draw_pos = 0b10;
        }
        if let Instruction::ADDX(amount) = instruction {
            if amount > 0 {
                sprite = sprite << amount;
            } else {
                sprite = sprite >> amount.abs();
            }
        }
        draw_pos = draw_pos << 1;
    }
}

fn format_output(binary_line: u64) -> String {
    format!("{:#042b}", binary_line)
        .chars()
        .rev()
        .take(40)
        .map(|c| match c {
            '0' => '.',
            '1' => '#',
            _ => panic!("Line not binary formatted."),
        })
        .collect()
}

#[repr(u8)]
enum Instruction {
    NOOP,
    ADDX(i32),
}

fn parse_instruction(line: &str) -> Option<Vec<Instruction>> {
    let mut parts = line.split_whitespace();
    match parts.next()? {
        "noop" => Some(vec![Instruction::NOOP]),
        "addx" => Some(vec![
            Instruction::NOOP,
            Instruction::ADDX(parts.next()?.parse().ok()?),
        ]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part1(&input), 12880);
        Ok(())
    }
}

// #[cfg(test)]
// mod benches {
//     use super::*;
//     extern crate test;
//     use test::Bencher;

//     #[bench]
//     fn bench_part2(b: &mut Bencher) -> Result<()> {
//         let input = fs::read_to_string(INPUT)?;
//         b.iter(|| part2(&input));
//         Ok(())
//     }
// }
