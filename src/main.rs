use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() -> Result<(), Box<dyn Error>> {
    let (mut calorie_totals, _) = read_lines("./input/day1.txt")?
    .fold((vec![], 0), |(mut acc, total), calorie_count| {
        if let Ok(count) = calorie_count.unwrap().parse::<i32>() {
            (acc, total + count)
        } else {
            acc.push(total);
            (acc, 0)
        }
    });
    // Part 1
    if let Some(maximum) = calorie_totals.iter().max() {
        println!("{}", maximum);
    }

    // Part 2
    calorie_totals.sort_unstable_by(|a, b| b.cmp(a));
    let top_3_total = calorie_totals.iter().take(3).sum::<i32>();
    println!("{}", top_3_total);
    Ok(())
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}