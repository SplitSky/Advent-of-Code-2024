use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn unpack_string(input: &str) -> Vec<&str> {
    // this takes in an input string and returns a vector of multiples
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let matches: Vec<&str> = re
        .find_iter(input) // Find all matches in the input string
        .map(|mat| mat.as_str()) // Convert each match to a string slice
        .collect();

    return matches; // ["mul(123,456)", "mul(78,910)", "mul(11,22)"]
}

fn unpack_mul(input: &str) -> i32 {
    // unpack and return the multiple
    println!("Internal input check: {}", input);
    let inner_content = &input[4..input.len() - 1];
    let parts: Vec<&str> = inner_content.split(',').collect();
    let num1: i32 = parts[0].trim().parse().unwrap();
    let num2: i32 = parts[1].trim().parse().unwrap();
    return num1 * num2;
}

fn main() -> io::Result<()> {
    // define sum variable
    let mut sum: i32 = 0;
    // loop over a line in a file and process it and add to the sum
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut keep_going: bool = true;
    for line in reader.lines() {
        let line = line?;
        let operations: Vec<&str> = unpack_string(&line);
        for operation in operations {
            // check whether it's a do or don't
            println!("operation {}", operation);
            println!("result of comparison {}", operation == "do()");
            println!("result of comparison {}", operation == "don't()");
            if operation == "do()" {
                keep_going = true;
            } else if operation == "don't()" {
                keep_going = false;
            } else if keep_going == true {
                sum += unpack_mul(operation);
            } // loop over each multiple
        }
    }
    println!("The total is: {}", sum);
    Ok(())
}
