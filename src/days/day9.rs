use crate::days::common::Result;
use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::repeat,
};
extern crate test;

const INPUT: &str = "./input/day9.txt";

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(i16, i16);

impl Point {
    fn difference(&self, other: &Point) -> (i16, i16) {
        (self.0 - other.0, self.1 - other.1)
    }
}

pub fn solution<'a>() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 9 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> usize {
    let (visited, _, _) = input.lines().flat_map(parse_moves).flatten().fold(
        (HashSet::from([Point(0, 0)]), Point(0, 0), Point(0, 0)),
        |(mut visited, head, tail), direction| {
            let new_head = match direction {
                Direction::Up => Point(head.0, head.1 + 1),
                Direction::Down => Point(head.0, head.1 - 1),
                Direction::Left => Point(head.0 - 1, head.1),
                Direction::Right => Point(head.0 + 1, head.1),
            };
            let (x_diff, y_diff) = new_head.difference(&tail);
            if x_diff.abs() == 2 || y_diff.abs() == 2 {
                visited.insert(head);
                (visited, new_head, head)
            } else {
                (visited, new_head, tail)
            }
        },
    );
    visited.len()
}

fn part2(input: &str) -> usize {
    let rope = VecDeque::from_iter(repeat(Point(0, 0)).take(10));
    let (visited, _) = input.lines().flat_map(parse_moves).flatten().fold(
        (HashSet::<Point>::new(), rope),
        |(mut visited, mut rope), direction| {
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
                // Tail could move so log position.
                let tail = rope.back().unwrap();
                visited.insert(*tail);
                (visited, move_rope(rope))
            } else {
                (visited, rope)
            }
        },
    );
    visited.len() + 1
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

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part1(&input), 6406);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part2(&input), 2643);
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        b.iter(|| part1(&input));
        Ok(())
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        b.iter(|| part2(&input));
        Ok(())
    }
}
