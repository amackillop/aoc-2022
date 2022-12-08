use std::collections::HashMap;

use crate::common::{self, Result};

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 7 ~~~~~~~~~~~~~");
    println!("Part 1: {}", part1(common::get_input_lines("day7")?));
    println!(
        "Part 2: {}",
        part2(common::get_input_lines("day7")?).ok_or("No dirs are big enough.")?
    );
    Ok(())
}

fn part1(lines: impl Iterator<Item = String>) -> u32 {
    get_sizes(lines)
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| *size <= 100_000)
        .sum()
}

fn part2(lines: impl Iterator<Item = String>) -> Option<u32> {
    let dir_sizes = get_sizes(lines);
    let unused = 70_000_000 - dir_sizes.get("/").unwrap();
    let space_needed = 30_000_000 - unused;
    let mut sizes_vec = dir_sizes
        .into_iter()
        .map(|(_, size)| size)
        .collect::<Vec<u32>>();
    sizes_vec.sort();
    for size in sizes_vec {
        if size > space_needed {
            return Some(size);
        }
    }
    None
}

fn get_sizes(lines: impl Iterator<Item = String>) -> HashMap<String, u32> {
    let (_, dir_sizes) = lines.flat_map(parse_line).fold(
        (Vec::<String>::new(), HashMap::<String, u32>::new()),
        |(mut stack, mut dir_sizes), log| match log {
            Log::Command(command) => match command {
                Command::Cd(dir_name) => match dir_name.as_str() {
                    ".." => {
                        stack.pop();
                        (stack, dir_sizes)
                    }
                    "/" => {
                        stack.push(dir_name);
                        (stack, dir_sizes)
                    }
                    _ => {
                        let mut qualified = stack.join("");
                        qualified.push_str(&dir_name);
                        stack.push(qualified);
                        (stack, dir_sizes)
                    }
                },
                Command::Ls => (stack, dir_sizes),
            },
            Log::Dir(_) => (stack, dir_sizes),
            Log::File(size) => {
                stack.iter().for_each(|dir_name| {
                    dir_sizes
                        .entry(dir_name.to_string())
                        .and_modify(|c| *c += size)
                        .or_insert(size);
                });
                (stack, dir_sizes)
            }
        },
    );
    dir_sizes
}

#[derive(PartialEq, Debug)]
enum Log {
    Command(Command),
    Dir(String),
    File(u32),
}

#[derive(PartialEq, Debug)]
enum Command {
    Cd(String),
    Ls,
}

fn parse_line(line: String) -> Option<Log> {
    let mut parts = line.split_whitespace();
    match parts.next()? {
        "$" => match parts.next()? {
            "cd" => Some(Log::Command(Command::Cd(parts.next()?.to_string()))),
            "ls" => Some(Log::Command(Command::Ls)),
            _ => None,
        },
        "dir" => Some(Log::Dir(parts.next()?.to_string())),
        first_part @ _ => Some(Log::File(first_part.parse().ok()?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_cd() -> Result<()> {
        assert_eq!(
            parse_line("$ cd a".to_string()),
            Some(Log::Command(Command::Cd("a".to_string())))
        );
        Ok(())
    }

    #[test]
    fn test_parse_command_ls() -> Result<()> {
        assert_eq!(
            parse_line("$ ls".to_string()),
            Some(Log::Command(Command::Ls))
        );
        Ok(())
    }

    #[test]
    fn test_parse_dir() -> Result<()> {
        assert_eq!(
            parse_line("dir a".to_string()),
            Some(Log::Dir("a".to_string()))
        );
        Ok(())
    }

    #[test]
    fn test_parse_file() -> Result<()> {
        assert_eq!(parse_line("123 f".to_string()), Some(Log::File(123)));
        Ok(())
    }
}
