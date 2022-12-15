use crate::days::common::Result;
use std::{
    collections::{vec_deque, HashSet, VecDeque},
    fs,
    iter::repeat,
};
extern crate test;

const INPUT: &str = "./input/day9.txt";

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(i16, i16);

impl Point {
    fn distance_between(&self, other: &Point) -> f32 {
        let Point(x0, y0) = other;
        let Point(x1, y1) = self;
        f32::sqrt(((x1 - x0).pow(2) + (y1 - y0).pow(2)).into())
    }

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
    let head = Point(0, 0);
    let tail = Point(0, 0);
    let (visited, _, _) = input.lines().flat_map(parse_moves).flatten().fold(
        (HashSet::from([Point(0, 0)]), head, tail),
        |(mut visited, head, tail), direction| {
            let new_head = match direction {
                Direction::Up => Point(head.0, head.1 + 1),
                Direction::Down => Point(head.0, head.1 - 1),
                Direction::Left => Point(head.0 - 1, head.1),
                Direction::Right => Point(head.0 + 1, head.1),
            };
            if new_head.distance_between(&tail) < 2.0 {
                (visited, new_head, tail)
            } else {
                visited.insert(head);
                (visited, new_head, head)
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
            rope.push_front(new_head);
            let tail = rope.pop_back().unwrap();
            visited.insert(tail);
            rope.push_back(tail);
            (visited, move_rope(rope, direction))
        },
    );
    visited.len() + 1
}

fn move_rope(mut rope: VecDeque<Point>, direction: Direction) -> VecDeque<Point> {
    let head = rope.pop_front().unwrap();
    if let Some(next_knot) = rope.pop_front() {
            let (move_x, move_y) = match head.difference(&next_knot) {
                (0, 2) => (0, 1),
                (1, 2) => (1, 1),
                (2, 2) => (1, 1),
                (2, 1) => (1, 1),
                (2, 0) => (1, 0),
                (2, -1) => (1, -1),
                (2, -2) => (1, -1),
                (1, -2) => (1, -1),
                (0, -2) => (0, -1),
                (-1, -2) => (-1, -1),
                (-2, -2) => (-1, -1),
                (-2, -1) => (-1, -1),
                (-2, 0) => (-1, 0),
                (-2, 1) => (-1, 1),
                (-2, 2) => (-1, 1),
                (-1, 2) => (-1, 1),
                _ => (0, 0)
            };
            let new_knot = Point(next_knot.0 + move_x, next_knot.1 + move_y);
            rope.push_front(new_knot);
            let mut rope = move_rope(rope, direction);
            rope.push_front(head);
            rope
    } else {
        rope.push_front(head);
        rope
    }
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
