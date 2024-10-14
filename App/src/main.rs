use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Read the file
    let path = Path::new("./src/numbers.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Collect numbers into a vector
    let mut numbers: Vec<i32> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    // Sort the numbers using rayon
    numbers.par_sort();

    // Print the sorted numbers
    for number in numbers {
        println!("{}", number);
    }

    Ok(())
}