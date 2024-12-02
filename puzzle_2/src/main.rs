use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};

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
    // TODO: convert list_1 into a set
    // TODO: calculate the frequency of each entry in list 1 in list 2. Get the scores and times
    // out entry from list 1 by the frequency from list 2

    let mut list_1_set: BTreeSet<_> = list_1.clone().into_iter().collect();

    // make a set for each entry make a struct.
    for number in &list_1 {
        list_1_set.insert(number.clone()); // insert into set
    }

    let mut similarity_score = 0;

    for entry in list_1_set {
        // loop over each set entry
        // loop over each number in the list 2 and count the frequency
        let mut count_of_occurance = 0;
        for number in &list_2 {
            if entry == *number {
                count_of_occurance += 1;
            }
        }
        similarity_score += count_of_occurance * entry;
    }
    // go through each struct and calculate the number of occurances of each entry in list_2
    // loop over the structs again and calculate the similarity score
    println!("sum is: {}", similarity_score);
    //println!("List 1: {:?}", list_1.len());
    //println!("List 2: {:?}", list_2.len());

    Ok(())
}
