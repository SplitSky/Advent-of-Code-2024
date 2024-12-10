fn dps(data: Vec<Vec<&str>>, word: Vec<&str>) {
    // loop over all tiles
    let mut word_counter = 0;
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            // go over all directions
            for i in 0..2 {
                let mut curr_x = x;
                let mut curr_y = y;
                // declare current position variables
                for j in 0..2 {
                    // loop over directions
                    // check the letter agrees
                    let mut stop = false;
                    let mut letter_counter = 0;
                    while stop != true {
                        // logic for comparisons
                        // check the letter matches
                        if word[letter_counter] != data[x][y] {
                            stop = true;
                        } else {
                            letter_counter += 1; // move to the next letter in the word
                                                 // move the direction
                                                 // check if it's the end of the word
                            if letter_counter == word.len() {
                                // success word has been found
                                word_counter += 1;
                            }
                            // make a move!
                            curr_x += i-1;
                            curr_y += j-1;
                            // check out of bounds
                            if curr_x < 0 || curr_x > data[x].len() {
                                // stop
                                stop = true;
                            }
                            if curr_y < 0 || curr_y > data.len() {
                                stop = true;
                            }
                            // else keep going
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let data_in = [[]]
}
