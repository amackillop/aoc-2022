use crate::days::common::Result;
use std::cmp::Ordering;
use std::fs;

use nom::character::complete::char;
use nom::character::complete::u8;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{branch::alt, IResult};
use nom::{bytes::complete::tag, combinator::map};

const INPUT: &str = "./input/day13.txt";

pub fn solution() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 13 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
    Ok(())
}

fn part_one(input: &str) -> usize {
    let packets: Vec<Data> = input.lines().flat_map(parse_data).map(|p| p.1).collect();
    packets
        .chunks(2)
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair[0].cmp(&pair[1]) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let divider_packets = vec![
        Data::List(vec![Data::Integer(2)]),
        Data::List(vec![Data::Integer(6)]),
    ];
    let mut packets: Vec<Data> = input.lines().flat_map(parse_data).map(|p| p.1).collect();
    packets.push(Data::List(vec![Data::Integer(2)]));
    packets.push(Data::List(vec![Data::Integer(6)]));
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if divider_packets.contains(p) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

fn parse_data(input: &str) -> IResult<&str, Data> {
    alt((integer, list))(input)
}

fn integer(input: &str) -> IResult<&str, Data> {
    map(u8, Data::Integer)(input)
}

fn list(input: &str) -> IResult<&str, Data> {
    map(
        delimited(char('['), separated_list0(tag(","), parse_data), char(']')),
        Data::List,
    )(input)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Data {
    Integer(u8),
    List(Vec<Data>),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::Integer(a), Data::Integer(b)) => a.cmp(b),
            (Data::Integer(a), list @ Data::List(_)) => {
                Data::List(vec![Data::Integer(*a)]).cmp(list)
            }
            (list @ Data::List(_), Data::Integer(a)) => {
                list.cmp(&Data::List(vec![Data::Integer(*a)]))
            }
            (Data::List(a_vec), Data::List(b_vec)) => {
                for (a, b) in a_vec.iter().zip(b_vec) {
                    match a.cmp(b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }
                a_vec.len().cmp(&b_vec.len())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Data::*;
    use super::*;

    #[test]
    fn test_parse_integer() -> Result<()> {
        assert_eq!(parse_data("1")?.1, Integer(1));
        assert_eq!(parse_data("10")?.1, Integer(10));

        Ok(())
    }

    #[test]
    fn test_parse_empty_list() -> Result<()> {
        assert_eq!(parse_data("[]")?.1, List(vec![]));
        Ok(())
    }
    #[test]
    fn test_parse_simple_list() -> Result<()> {
        assert_eq!(
            parse_data("[1,2,3]")?.1,
            List(vec![Integer(1), Integer(2), Integer(3)])
        );
        Ok(())
    }

    #[test]
    fn test_parse_nested_list() -> Result<()> {
        assert_eq!(
            parse_data("[1,[2,[3,4],5],6]")?.1,
            List(vec![
                Integer(1),
                List(vec![
                    Integer(2),
                    List(vec![Integer(3), Integer(4)]),
                    Integer(5)
                ]),
                Integer(6)
            ])
        );
        Ok(())
    }

    #[test]
    fn test_data_compare_1() -> Result<()> {
        let left = &parse_data("[1,1,3,1,1]")?.1;
        let right = &parse_data("[1,1,5,1,1]")?.1;
        assert_eq!(left.cmp(right), Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_data_compare_2() -> Result<()> {
        let left = &parse_data("[[1],[2,3,4]]")?.1;
        let right = &parse_data("[[1],4]")?.1;
        assert_eq!(left.cmp(right), Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_data_compare_3() -> Result<()> {
        let left = &parse_data("[9]")?.1;
        let right = &parse_data("[[8,7,6]]")?.1;
        assert_eq!(left.cmp(right), Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_data_compare_4() -> Result<()> {
        let left = &parse_data("[[4,4],4,4]")?.1;
        let right = &parse_data("[[4,4],4,4,4]")?.1;
        assert_eq!(left.cmp(right), Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_data_compare_5() -> Result<()> {
        let left = &parse_data("[7,7,7,7]")?.1;
        let right = &parse_data("[7,7,7]")?.1;
        assert_eq!(left.cmp(right), Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_data_compare_6() -> Result<()> {
        let left = &parse_data("[]")?.1;
        let right = &parse_data("[3]")?.1;
        assert_eq!(left.cmp(right), Ordering::Less);
        Ok(())
    }

    #[test]
    fn test_data_compare_7() -> Result<()> {
        let left = &parse_data("[[[]]]")?.1;
        let right = &parse_data("[[]]")?.1;
        assert_eq!(left.cmp(right), Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_data_compare_8() -> Result<()> {
        let left = &parse_data("[1,[2,[3,[4,[5,6,7]]]],8,9]")?.1;
        let right = &parse_data("[1,[2,[3,[4,[5,6,0]]]],8,9]")?.1;
        assert_eq!(left.cmp(right), Ordering::Greater);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_one(&input), 5292);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_two(&input), 23868);
        Ok(())
    }
}
