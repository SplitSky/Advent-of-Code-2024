use std::fs::File;
use std::io::{self, BufRead};

struct Location {
    id: i32,
    frequency: i32,
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    // Process each line
    for line in reader.lines() {
        let line = line?;
        // if let Some((left, right)) = line.split_once('') {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<i32>(), right.parse::<i32>()) {
                list_1.push(left_num);
                list_2.push(right_num);
            } else {
                eprintln!("Failed to parse numbers in line: {}", line);
            }
        } else {
            eprintln!("Malformed line: {}", line);
        }
    }

    list_1.sort();
    list_2.sort();

    // get the distances between the numbers
    let mut distances = Vec::new();
    for i in 0..list_1.len() {
        distances.push((list_1[i] - list_2[i]).abs())
    }

    let mut sum = 0;
    for number in distances {
        sum += number;
    }
    println!("sum is: {}", sum);
    println!("List 1: {:?}", list_1.len());
    println!("List 2: {:?}", list_2.len());

    Ok(())
}
