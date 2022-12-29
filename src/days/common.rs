use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

// Result alias with an implicit boxed error to clean up type sigs
pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

// Returns an Iterator to the Reader of the lines of the file.
pub fn get_input_lines(day: &str) -> Result<impl Iterator<Item = String>> {
    let file = File::open(format!("./input/{day}.txt"))?;
    Ok(io::BufReader::new(file).lines().map(|l| l.unwrap()))
}

// Transpose a matrix
pub fn transpose<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if let Some(first_row) = matrix.first() {
        let ncols = first_row.len();
        let mut row_iters: Vec<_> = matrix.into_iter().map(|n| n.into_iter()).collect();
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
        let grid = [vec![1, 2, 3], vec![4, 5, 6]];
        let transposed = [[1, 4], [2, 5], [3, 6]];
        assert_eq!(transpose(&grid), transposed);
        Ok(())
    }
}
