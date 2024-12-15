use std::fs::File;
use std::io::{self, BufRead};

struct Guard {
    x_pos: i32,
    y_pos: i32,
    direction: (i32, i32),
}

impl Guard {
    fn new(x_pos: i32, y_pos: i32, direction: (i32, i32)) -> Guard {
        // initialise the guard struct
        Self {
            x_pos,
            y_pos,
            direction,
        }
    }
    fn rotate(&mut self) {
        // matrix multiplication
        //[0, -1] [x]= [-y]
        //[1,  0] [y]= [ x]
        self.direction.0 = self.direction.1;
        self.direction.1 = -self.direction.0;
    }
    fn step(&mut self) {
        // take step forward
        self.x_pos += self.direction.0;
        self.y_pos += self.direction.1;
    }
    fn print_things(&mut self) {
        println!("position is: {},{}", self.x_pos, self.y_pos);
        println!("direction is: {}, {}", self.direction.0, self.direction.1);
    }
}

fn main() -> io::Result<()> {
    // grab and populate the board
    let file = File::open("input2.txt")?;
    let reader = io::BufReader::new(file);
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut guard: Option<Guard> = None;

    // Process each line
    for line in reader.lines() {
        let line = line?;
        let parts = line.chars().collect();
        board.push(parts);
    }
    // find the guard
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == '^' {
                // found guard initialise
                guard = Some(Guard::new(
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    (0, -1),
                ));
                board[i][j] = 'X';
                break;
            }
        }
        if guard.is_some() {
            break;
        }
    }
    let mut guard = guard.expect("Guard not found on the board");
    println!("board {:?}", board);
    let mut colored_fields = 0;
    while guard.y_pos + guard.direction.1 >= 0
        && guard.y_pos + guard.direction.1 < board.len() as i32
        && guard.x_pos + guard.direction.0 >= 0
        && guard.x_pos + guard.direction.1 < board[guard.y_pos as usize].len() as i32
    {
        println!("guard x struct {}", guard.x_pos);
        println!("guard y struct {}", guard.y_pos);
        // next position
        let x = (guard.x_pos + guard.direction.0) as usize;
        let y = (guard.y_pos + guard.direction.1) as usize;
        let ahead = board[y][x];
        //        println!("guard x {}", x);
        //        println!("guard y {}", y);

        if board[guard.y_pos as usize][guard.x_pos as usize] != 'X' {
            board[guard.y_pos as usize][guard.x_pos as usize] = 'X';
            colored_fields += 1;
        }
        match ahead {
            '.' => {
                // do nothing
                guard.step();
                guard.print_things();
            }
            '#' => {
                // obstacle
                guard.rotate();
                guard.print_things();
            }
            _ => {
                guard.print_things();
            } // wildcard
        }
    }
    println!("Final Board:");
    for row in &board {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Total colored fields: {}", colored_fields);

    Ok(())
}
