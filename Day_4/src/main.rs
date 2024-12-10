fn depth_first_search(data: Vec<Vec<&str>>, word: Vec<&str>) {
    // directions
    //  word = ["X", "M", "A", "S"];

    for i in 0..2 {
        for j in 0..2 {
            // loops over all directions
            // i-1 is the loop
            // j-1 is the loop
            let mut letter_counter = 0;
            let stop = false;
            // check letter matches
            if data[i][j] == word[letter_counter] {
                letter_counter += 1;
                // check if it's end of the word
                if letter_counter < word.len() - 1 {
                    // continue
                } else if letter_counter == word.len() - 1 {
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
