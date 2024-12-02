use std::fs::File;
use std::io::{self, BufRead};

fn check_report(numbers: Vec<i32>) -> i32 {
    // checks the report for the following criteria
    // all values are increasing or decreasing
    // all values differences are between 1 and 3
    let mut previous_gradient = numbers[0] - numbers[1];
    let mut count = 0;
    for i in 1..numbers.len() - 1 {
        let change = numbers[i] - numbers[i + 1];
        // still same gradient check
        if change * previous_gradient == -1 {
            // different gradient
            return 0;
        }
        if change.abs() > 3 || change.abs() < 1 {
            return 0;
        }
        previous_gradient = change;
    }
    return 1;
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|line| line.parse().expect("parse error"))
            .collect();
        sum += check_report(numbers);
    }
    println!("the sum is: {}", sum);

    Ok(())
}
