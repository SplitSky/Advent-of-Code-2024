use std::fs::File;
use std::io::{self, BufRead};

fn check_report(numbers: Vec<i32>) -> i32 {
    // checks the report for the following criteria
    // all values are increasing or decreasing
    // all values differences are between 1 and 3
    let mut previous_gradient = numbers[0] - numbers[1];
    for i in 0..numbers.len() - 1 {
        let change = numbers[i] - numbers[i + 1];
        // still same gradient check
        if change * previous_gradient < 0 {
            // different gradient
            return 0;
        }
        if change.abs() > 3 || change.abs() == 0 {
            return 0;
        }
        previous_gradient = change;
    }
    return 1;
}

fn remove_index(numbers: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut result = numbers.clone();
    result.remove(index);
    return result;
}

fn check_report2(numbers: &Vec<i32>) -> i32 {
    // Check if the report is valid as-is
    if check_report(numbers.clone()) == 1 {
        return 1;
    } else {
        // Verify if removing a single number fixes the report
        let mut success_count: i32 = 0;
        for i in 0..numbers.len() {
            let modified_numbers = remove_index(numbers, i);
            if check_report(modified_numbers) == 1 {
                success_count += 1;
            }
        }
        // If exactly one fix exists, return 1. Otherwise, return 0.
        if success_count > 0 {
            return 1;
        } else {
            return 0;
        }
    }
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
        sum += check_report2(&numbers);
    }
    //let line = "8 6 4 4 1".to_string();
    //println!("initial line: {}", line);
    //let numbers: Vec<i32> = line
    //    .split_whitespace()
    //    .map(|line| line.parse().expect("parse error"))
    //    .collect();
    //sum = check_report2(&numbers);
    println!("the sum is: {}", sum);

    Ok(())
}
