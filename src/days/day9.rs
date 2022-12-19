use crate::days::common::Result;
use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::repeat,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(i16, i16);

impl Point {
    fn difference(&self, other: &Point) -> (i16, i16) {
        (self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const INPUT: &str = "./input/day9.txt";
pub fn solution<'a>() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 9 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", count_unique_tail_positions(&input, 2));
    println!("Part 2: {}", count_unique_tail_positions(&input, 10));
    Ok(())
}

fn count_unique_tail_positions(input: &str, size_of_rope: usize) -> usize {
    let mut rope = VecDeque::from_iter(repeat(Point(0, 0)).take(size_of_rope));
    let mut visited: HashSet<Point> = HashSet::new();
    for direction in input.lines().flat_map(parse_moves).flatten() {
        let head = rope.pop_front().unwrap();
        let new_head = match direction {
            Direction::Up => Point(head.0, head.1 + 1),
            Direction::Down => Point(head.0, head.1 - 1),
            Direction::Left => Point(head.0 - 1, head.1),
            Direction::Right => Point(head.0 + 1, head.1),
        };
        let next_knot = rope.front().unwrap();
        let (x_diff, y_diff) = new_head.difference(&next_knot);
        rope.push_front(new_head);
        if x_diff.abs() == 2 || y_diff.abs() == 2 {
            rope = move_rope(rope);
            visited.insert(*rope.back().unwrap());
        };
    }
    visited.len()
}

fn move_rope(rope: VecDeque<Point>) -> VecDeque<Point> {
    let head = rope[0];
    let mut new_rope = VecDeque::from([head]);
    let mut previous_knot = head;
    for knot in rope.into_iter().skip(1) {
        let (x_diff, y_diff) = previous_knot.difference(&knot);
        if x_diff.abs() == 2 || y_diff.abs() == 2 {
            let moved_knot = Point(knot.0 + x_diff.signum(), knot.1 + y_diff.signum());
            previous_knot = moved_knot;
            new_rope.push_back(moved_knot);
        } else {
            previous_knot = knot;
            new_rope.push_back(knot);
        }
    }
    new_rope
}

fn parse_moves(line: &str) -> Option<Vec<Direction>> {
    let (direction, magnitude) = line.split_once(" ")?;
    let magnitude = magnitude.parse().ok()?;
    let direction = match direction {
        "U" => Some(Direction::Up),
        "D" => Some(Direction::Down),
        "L" => Some(Direction::Left),
        "R" => Some(Direction::Right),
        _ => None,
    }?;
    let moves = repeat(direction).take(magnitude);
    Some(moves.collect())
}

#[cfg(test)]
mod tests {
    // extern crate test;
    // use test::Bencher;
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(count_unique_tail_positions(&input, 2), 6406);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(count_unique_tail_positions(&input, 10), 2643);
        Ok(())
    }

    // #[bench]
    // fn bench_count_tail_positions(b: &mut Bencher) -> Result<()> {
    //     let input = fs::read_to_string(INPUT)?;
    //     b.iter(|| count_unique_tail_positions(&input, 10));
    //     Ok(())
    // }
}
