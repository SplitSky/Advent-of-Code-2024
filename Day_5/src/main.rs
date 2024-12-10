// function to get the page ordering rules
// function to store the page ordering rules in a hash map
// function to store the manuals
// function to validate that a single manual does not validate any of the rules - return true/false
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn validate_single_manual(data: Vec<&str>, rules: ) {
    
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut data_in: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<String> = split_into_chars(&line);
        data_in.push(parts);
    }
    let word = vec!["X", "M", "A", "S"]; // Word to search

    Ok(())
}
