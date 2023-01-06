use crate::days::common::Result;
use std::collections::HashSet;
use std::fs;

use ndarray::Array2;
use ndarray::ArrayView2;
use nom::character::complete::char;
use nom::character::complete::i32;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::{bytes::complete::tag, combinator::map};

use ndarray::Array;

type Point = (i32, i32);

const INPUT: &str = "./input/day14.txt";

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 14 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
    Ok(())
}

fn part_one(input: &str) -> i32 {
    let scan = render_scan(input, 0);
    simulate_sand(scan)
}

fn part_two(input: &str) -> i32 {
    let scan = render_scan(input, 150);
    simulate_sand(scan)
}

fn render_scan(input: &str, padding: i32) -> Array2<char> {
    let points: Vec<Point> = input
        .lines()
        .flat_map(parse_filled_points)
        .map(|(_, res)| res)
        .flatten()
        .collect();
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();
    let point_set: HashSet<&Point> = points.iter().collect();
    Array::from_shape_fn(
        (
            (max_y + padding + 1) as usize,
            (max_x + padding + 1) as usize,
        ),
        |(i, j)| {
            if i == (max_y + 2) as usize {
                '#'
            } else if point_set.contains(&(j as i32, i as i32)) {
                '#'
            } else {
                '.'
            }
        },
    )
}

fn simulate_sand(mut scan: Array2<char>) -> i32 {
    let mut count = 0;
    for i in 0.. {
        scan = if let Some(scan) = add_sand(500, scan) {
            match move_sand(0, 500, scan) {
                Some(scan) => scan,
                None => {
                    count = i;
                    break;
                }
            }
        } else {
            count = i;
            break;
        };
    }
    count
}

fn add_sand(col: usize, mut array: Array2<char>) -> Option<Array2<char>> {
    let start = array.get_mut([0, col]).unwrap();
    if start == &'o' {
        None
    } else {
        *start = 'o';
        Some(array)
    }
}

fn move_sand(row: usize, col: usize, mut array: Array2<char>) -> Option<Array2<char>> {
    if let Some(below) = array.get_mut([row + 1, col]) {
        if below == &'.' {
            *below = 'o';
            array[[row, col]] = '.';
            return move_sand(row + 1, col, array);
        } else if let Some(below_left) = array.get_mut([row + 1, col - 1]) {
            if below_left == &'.' {
                *below_left = 'o';
                array[[row, col]] = '.';
                return move_sand(row + 1, col - 1, array);
            } else {
                if let Some(below_right) = array.get_mut([row + 1, col + 1]) {
                    if below_right == &'.' {
                        *below_right = 'o';
                        array[[row, col]] = '.';
                        return move_sand(row + 1, col + 1, array);
                    }
                }
            }
        }
        Some(array)
    } else {
        None
    }
}

// Parse 498,4 -> 498,6 -> 496,6 into [(498, 4), (498, 6), (496, 6)]
fn parse_points(line: &str) -> IResult<&str, Vec<Point>> {
    separated_list0(tag(" -> "), separated_pair(i32, char(','), i32))(line)
}

fn parse_filled_points(line: &str) -> IResult<&str, Vec<Point>> {
    map(parse_points, fill_points)(line)
}

// Take a vec of points fill in the lines between them.
// [(498, 4), (498, 6), (496, 6)] -> [(498, 4), (498, 5), (498, 6), (497,6), (496, 6)]
fn fill_points(points: Vec<Point>) -> Vec<Point> {
    let mut filled_points: Vec<Point> = vec![];
    for pair in points.windows(2) {
        filled_points.pop();
        if let [(x1, y1), (x2, y2)] = *pair {
            if x2 > x1 {
                for x in x1..=x2 {
                    filled_points.push((x, y1))
                }
            }
            if x2 < x1 {
                for x in (x2..=x1).rev() {
                    filled_points.push((x, y1))
                }
            }
            if y2 > y1 {
                for y in y1..=y2 {
                    filled_points.push((x1, y))
                }
            }
            if y2 < y1 {
                for y in (y2..=y1).rev() {
                    filled_points.push((x1, y))
                }
            }
        }
    }
    filled_points
}

// Helper function for debugging
fn print_cave(array: ArrayView2<char>) {
    for row in array.rows() {
        println!["{:?}", row.to_string()];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_points() -> Result<()> {
        let input = "498,4 -> 498,6 -> 496,6";
        assert_eq!(parse_points(&input)?.1, [(498, 4), (498, 6), (496, 6)]);
        Ok(())
    }

    #[test]
    fn test_fill_points() -> Result<()> {
        let input = vec![(498, 4), (498, 6), (496, 6), (496, 4)];
        let set_of_points = fill_points(input);
        let expected = vec![
            (498, 4),
            (498, 5),
            (498, 6),
            (497, 6),
            (496, 6),
            (496, 5),
            (496, 4),
        ];
        assert_eq!(set_of_points, expected);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_one(&input), 779);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_two(&input), 27426);
        Ok(())
    }
}
