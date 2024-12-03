use std::fs::File;
use std::io::{self, BufRead};

fn check_report(numbers: Vec<i32>) -> i32 {
    // checks the report for the following criteria
    // all values are increasing or decreasing
    // all values differences are between 1 and 3
    let mut previous_gradient = numbers[0] - numbers[1];
    for i in 0..numbers.len() - 1 {
        let change = numbers[i] - numbers[i + 1];
        println!("change = {}", change);
        println!("numbers: {:?}", numbers);
        // still same gradient check
        println!("grad check:: {}", change * previous_gradient);
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
        println!("line converted to numbers: {:?}", numbers);
        sum += check_report(numbers);
    }
    let mut line = "1 1 3 2 7 9".to_string();
    //    let numbers: Vec<i32> = line
    //        .split_whitespace()
    //        .map(|line| line.parse().expect("parse error"))
    //        .collect();
    //    sum = check_report(numbers);
    println!("the sum is: {}", sum);

    Ok(())
}
