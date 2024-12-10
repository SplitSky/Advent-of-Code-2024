use std::fs::File;
use std::io::{self, BufRead};

fn dps(data: Vec<Vec<String>>, word: Vec<&str>) {
    let mut word_counter = 0;

    // All 8 possible directions
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    for x in 0..data.len() {
        for y in 0..data[x].len() {
            for &(dx, dy) in &directions {
                let mut curr_x = x as isize;
                let mut curr_y = y as isize;
                let mut letter_counter = 0;

                while letter_counter < word.len() {
                    // Check out of bounds
                    if curr_x < 0
                        || curr_x >= data.len() as isize
                        || curr_y < 0
                        || curr_y >= data[curr_x as usize].len() as isize
                    {
                        break;
                    }

                    // Check if the current letter matches
                    if word[letter_counter] != data[curr_x as usize][curr_y as usize] {
                        break;
                    }

                    // Move to the next letter and position
                    letter_counter += 1;
                    curr_x += dx;
                    curr_y += dy;
                }

                // If the entire word was matched
                if letter_counter == word.len() {
                    word_counter += 1;
                }
            }
        }
    }

    println!("found it {} times", word_counter);
}

fn part2_cross_search(data: Vec<Vec<String>>) {
    let mut cross_counter = 0;

    for x in 0..data.len() {
        for y in 0..data[x].len() {
            // Check cross boundaries
            if x + 2 >= data.len() || y < 2 {
                continue;
            }

            // Check for 4 cross patterns
            if data[x][y] == "M"
                && data[x + 2][y] == "S"
                && data[x][y - 2] == "M"
                && data[x + 2][y - 2] == "S"
                && data[x + 1][y - 1] == "A"
            {
                cross_counter += 1;
            }
            if data[x][y] == "M"
                && data[x + 2][y] == "M"
                && data[x][y - 2] == "S"
                && data[x + 2][y - 2] == "S"
                && data[x + 1][y - 1] == "A"
            {
                cross_counter += 1;
            }
            if data[x][y] == "S"
                && data[x + 2][y] == "M"
                && data[x][y - 2] == "S"
                && data[x + 2][y - 2] == "M"
                && data[x + 1][y - 1] == "A"
            {
                cross_counter += 1;
            }
            if data[x][y] == "S"
                && data[x + 2][y] == "S"
                && data[x][y - 2] == "M"
                && data[x + 2][y - 2] == "M"
                && data[x + 1][y - 1] == "A"
            {
                cross_counter += 1;
            }
        }
    }

    println!("Found it {} times", cross_counter);
}

fn split_into_chars(input: &str) -> Vec<String> {
    input
        .char_indices()
        .map(|(i, _)| input[i..i + input[i..].chars().next().unwrap().len_utf8()].to_string())
        .collect()
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
    dps(data_in.clone(), word);
    part2_cross_search(data_in);

    Ok(())
}
