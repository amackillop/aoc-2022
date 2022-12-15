use crate::days::common::Result;
use std::{collections::HashSet, fs, vec};
// extern crate test;

pub fn solution<'a>() -> Result<()> {
    println!("~~~~~~~~~~~~~ Day 8 ~~~~~~~~~~~~~");
    let input = fs::read_to_string("./input/day8.txt")?;
    println!("Part 1: {}", part1(&input).ok_or("Empty input.")?);
    println!("Part 2: {}", part2(&input).ok_or("Empty input.")?);
    Ok(())
}

fn part1(input: &str) -> Option<usize> {
    let rows = build_grid1(input);
    let cols = transpose(&rows);
    let num_edge_trees = 2 * (rows.len() + cols.len()) - 4;
    let lr_set = visible(&rows);
    let tb_set = visible(&cols);
    Some(num_edge_trees + (&lr_set | &tb_set).len())
}

fn part2(input: &str) -> Option<usize> {
    let rows = build_grid2(input);
    let cols = transpose(&rows);
    let row_score_vecs = compute_row_scores(rows);
    let col_score_vecs: Vec<Vec<usize>> = transpose(&compute_row_scores(cols));

    row_score_vecs
        .iter()
        .zip(col_score_vecs.iter())
        .flat_map(|(r_scores, c_scores)| {
            r_scores
                .iter()
                .zip(c_scores.iter())
                .map(|(r_score, c_score)| r_score * c_score)
                .max()
        })
        .max()
}

fn visible<T: Ord>(rows: &Vec<Vec<(usize, usize, T)>>) -> HashSet<(&usize, &usize)> {
    let mut visible_set = HashSet::<(&usize, &usize)>::new();
    for row in &rows[1..rows.len() - 1] {
        // Left pass
        let mut tallest = &row[0].2;
        for (i, j, tree) in &row[1..row.len() - 1] {
            if tree > tallest {
                visible_set.insert((i, j));
                tallest = tree;
            }
        }

        // Right pass
        let mut tallest = &row[row.len() - 1].2;
        for (i, j, tree) in row[1..row.len() - 1].iter().rev() {
            if visible_set.contains(&(i, j)) {
                break;
            }
            if tree > tallest {
                visible_set.insert((i, j));
                tallest = tree
            }
        }
    }
    visible_set
}

fn compute_row_scores(rows: Vec<Vec<char>>) -> Vec<Vec<usize>> {
    rows.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(index, _)| view_score(index, row))
                .collect()
        })
        .collect()
}

fn view_score<T: Ord>(index: usize, row: &Vec<T>) -> usize {
    let height = &row[index];
    let to_the_left = row.iter().take(index).rev();
    let to_the_right = row.iter().skip(index + 1);
    let mut count_left = 0;
    for tree in to_the_left {
        if height > tree {
            count_left += 1;
        } else {
            count_left += 1;
            break;
        }
    }
    let mut count_right = 0;
    for tree in to_the_right {
        if height > tree {
            count_right += 1;
        } else {
            count_right += 1;
            break;
        }
    }
    count_left * count_right
}

fn build_grid1(input: &str) -> Vec<Vec<(usize, usize, char)>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, byte)| (i, j, byte))
                .collect()
        })
        .collect()
}

fn build_grid2(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn transpose<T: Copy>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if let Some(first_row) = grid.first() {
        let ncols = first_row.len();
        let mut row_iters: Vec<_> = grid.into_iter().map(|n| n.into_iter()).collect();
        (0..ncols)
            .map(|_| {
                row_iters
                    .iter_mut()
                    .map(|n| *n.next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect()
    } else {
        vec![vec![]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test_transpose() -> Result<()> {
        let grid = build_grid1("123\n456");
        let transposed = vec![
            vec![(0, 0, '1'), (1, 0, '4')],
            vec![(0, 1, '2'), (1, 1, '5')],
            vec![(0, 2, '3'), (1, 2, '6')],
        ];
        assert_eq!(transpose(&grid), transposed);
        Ok(())
    }

    #[test]
    fn test_view_Score() -> Result<()> {
        let row = "30373".chars().collect();
        assert_eq!(view_score(3, &row), 3);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = fs::read_to_string(format!("./input/day8.txt"))?;
        assert_eq!(part1(&input), Some(1533));
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = fs::read_to_string("./input/day8.txt")?;
        assert_eq!(part2(&input), Some(345744));
        Ok(())
    }

    // #[bench]
    // fn bench_part_one(b: &mut Bencher) -> Result<()> {
    //     let input = fs::read_to_string(format!("./input/day8.txt"))?;
    //     b.iter(|| part1(&input));
    //     Ok(())
    // }

    // #[bench]
    // fn bench_part_two(b: &mut Bencher) -> Result<()> {
    //     let input = fs::read_to_string(format!("./input/day8.txt"))?;
    //     b.iter(|| part2(&input));
    //     Ok(())
    // }
}
