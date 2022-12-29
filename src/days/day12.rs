use crate::days::common::Result;
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

use super::common::{self};

const INPUT: &str = "./input/day12.txt";

pub fn solution<'a>() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 12 ~~~~~~~~~~~~~");
    let input = fs::read_to_string(INPUT)?;
    println!("Part 1: {}", part_one(&input).ok_or("Invalid input.")?);
    println!("Part 2: {}", part_two(&input).ok_or("Invalid input.")?);
    Ok(())
}

fn part_one(input: &str) -> Option<usize> {
    let grid_rows = parse_rows(input);
    let (start, end) = find_start_end(&grid_rows);
    let graph = build_graph(grid_rows);
    find_shortest_path(&graph, start?, end?)
}

fn part_two(input: &str) -> Option<usize> {
    let grid_rows = parse_rows(input);
    let (_, end) = find_start_end(&grid_rows);
    let starts = find_all_a(&grid_rows);
    let graph = build_graph(grid_rows);
    starts
        .into_iter()
        .flat_map(|start| find_shortest_path(&graph, start, end?))
        .min()
}

fn find_start_end(grid_rows: &Vec<Vec<Node>>) -> (Option<Node>, Option<Node>) {
    let mut start = None;
    let mut end = None;
    for row in grid_rows {
        for node in row {
            match node.value {
                '`' => start = Some(*node),
                '{' => end = Some(*node),
                _ => (),
            }
        }
    }
    (start, end)
}

fn find_all_a(grid_rows: &Vec<Vec<Node>>) -> Vec<Node> {
    let mut starts = vec![];
    for row in grid_rows {
        for node in row {
            match node.value {
                'a' => starts.push(*node),
                _ => (),
            }
        }
    }
    starts
}

fn find_shortest_path(graph: &HashMap<Node, Vec<Node>>, start: Node, end: Node) -> Option<usize> {
    let mut frontier = BinaryHeap::new();
    frontier.push(NodeDistance {
        node: start,
        distance: 0,
    });
    let mut distances = HashMap::<Node, usize>::new();
    distances.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.node == end {
            return Some(current.distance);
        }
        if let Some(next_nodes) = graph.get(&current.node) {
            for next_node in next_nodes {
                let distance = current.distance + 1;
                if distance < *distances.entry(*next_node).or_insert(usize::MAX) {
                    distances.insert(*next_node, distance);
                    frontier.push(NodeDistance {
                        node: *next_node,
                        distance,
                    });
                }
            }
        }
    }
    None
}

fn parse_rows(input: &str) -> Vec<Vec<Node>> {
    let mut rows: Vec<Vec<Node>> = vec![];
    for (r_index, line) in input.lines().enumerate() {
        rows.push(
            line.chars()
                .enumerate()
                .map(|(c_index, char)| match char {
                    'S' => Node {
                        // The ASCII char before 'a'
                        value: '`',
                        r_index,
                        c_index,
                    },
                    'E' => Node {
                        // The ASCII char after 'z'
                        value: '{',
                        r_index,
                        c_index,
                    },
                    _ => Node {
                        value: char,
                        r_index,
                        c_index,
                    },
                })
                .collect(),
        );
    }
    rows
}

fn find_edges(rows: &Vec<Vec<Node>>) -> Vec<Edge> {
    let cols = common::transpose(&rows);
    let mut edges: Vec<Edge> = vec![];
    for row in rows.iter().chain(cols.iter()) {
        for pair in row.windows(2) {
            let left = pair[0];
            let right = pair[1];
            match right.value as i16 - left.value as i16 {
                -1 | 0 | 1 => {
                    edges.push(Edge {
                        from: left,
                        to: right,
                    });
                    edges.push(Edge {
                        from: right,
                        to: left,
                    });
                }
                step @ _ if step > 0 => edges.push(Edge {
                    from: right,
                    to: left,
                }),
                _ => edges.push(Edge {
                    from: left,
                    to: right,
                }),
            }
        }
    }
    edges
}

fn build_graph(grid_rows: Vec<Vec<Node>>) -> HashMap<Node, Vec<Node>> {
    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    for edge in find_edges(&grid_rows) {
        let next_nodes = graph.entry(edge.from).or_insert(vec![]);
        next_nodes.push(edge.to)
    }
    graph
}

struct Edge {
    from: Node,
    to: Node,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    value: char,
    r_index: usize,
    c_index: usize,
}

#[derive(PartialEq, Eq)]
struct NodeDistance {
    node: Node,
    distance: usize,
}
// Custom Ord implementations to prioritize by minimum in the queue.
impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}
impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_one(&input), Some(437));
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string(INPUT)?;
        assert_eq!(part_two(&input), Some(430));
        Ok(())
    }
}

// #[cfg(test)]
// mod benches {
//     use super::*;
//     extern crate test;
//     use test::Bencher;

//     #[bench]
//     fn bench_find_shortest_path(b: &mut Bencher) -> Result<()> {
//         let input = fs::read_to_string(INPUT)?;
//         let grid_rows = parse_rows(&input);
//         let (start, end) = find_start_end(&grid_rows);
//         let graph = build_graph(parse_rows(&input));
//         b.iter(|| find_shortest_path(&graph, start?, end?));
//         Ok(())
//     }
// }
