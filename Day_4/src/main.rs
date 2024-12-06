 n check_square(square: Vec<[&str; 4]>) {
    // checks the array of strings
    let mut x = 0; // x-coordinate
    let mut y = 0; // y-coordinate
    let word = "XMAS";
    let word2 = "SMAX";
    // perform check vertically on the first row - two words
    // perform check horizontally on the first column - two words
    // perform a check diagonally on the left to right - two words
    // perform a check diagonally on the right to left - two words
}

fn DPS(data: Vec<Vec<&str>>, word: Vec<&str>) {
    // directions
    //  word = ["X", "M", "A", "S"];
    for i 

    for i -1..=1 {
        for j -1..=1 {
            // loops over all directions
            let mut letter_counter = 0;
            let stop = false;
            // check letter matches
            if data[i][j] = word[letter_counter] {
                letter_counter += 1;
                // check if it's end of the word
                if letter_counter < word.len() - 1 {
                     // continue
                } else if letter_counter == word.len() -1 {
                    // check if the last letter is correct
                }
            }
            // if 
            //      Move counter by one
            //      Check if it's at the end of the word
            //      else -> stop
            //  else
            //      stop
            //  check move criteria
            //  if out of bounds
            //      stop
            //  else
            //      nothing

        }
    }
}

fn main() {
    println!("Hello, world!");
}
