use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};

struct Guard {
    x_pos: i32,
    y_pos: i32,
    direction: (i32, i32), // (dx, dy)
    turn_position: VecDeque<(i32, i32)>,
}

impl Guard {
    fn new(x_pos: i32, y_pos: i32, direction: (i32, i32)) -> Guard {
        Self {
            x_pos,
            y_pos,
            direction,
            turn_position: VecDeque::new(),
        }
    }

    fn rotate(&mut self) {
        let (dx, dy) = self.direction;
        self.direction = (-dy, dx);
        self.turn_position.push_back((self.x_pos, self.y_pos));
        if self.turn_position.len() > 3 {
            self.turn_position.pop_front();
        }
    }

    fn step(&mut self) {
        self.x_pos += self.direction.0;
        self.y_pos += self.direction.1;
    }

    fn print_things(&self) {
        println!(
            "Position: ({}, {}), Direction: ({}, {})",
            self.x_pos, self.y_pos, self.direction.0, self.direction.1
        );
    }
    fn calculate_fourth_point(&self) -> Option<(i32, i32)> {
        if self.turn_position.len() == 3 {
            let &(x1, y1) = &self.turn_position[0];
            let &(x2, y2) = &self.turn_position[1];
            let &(x3, y3) = &self.turn_position[2];

            let x4 = x1 + x3 - x2;
            let y4 = y1 + y3 - y2;
            Some((x4, y4))
        } else {
            None
        }
    }
    fn check_position(&self, pos_to_check: (i32, i32)) -> bool {
        return pos_to_check.0 == self.x_pos && pos_to_check.1 == self.y_pos;
    }
}

/// Writes the current state of the board to a file
fn write_board_to_file(board: &Vec<Vec<char>>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for row in board {
        let line: String = row.iter().collect();
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut board: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut guard: Option<Guard> = None;

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == '^' {
                guard = Some(Guard::new(x as i32, y as i32, (0, -1)));
                board[y][x] = '.';
                break;
            }
        }
        if guard.is_some() {
            break;
        }
    }

    if guard.is_none() {
        eprintln!("Error: Guard not found on the board.");
        std::process::exit(1);
    }

    let mut guard = guard.unwrap();
    let mut colored_fields = 0;
    let mut obstacle_counter = 0;
    let mut fourth_point: Option<(i32, i32)>;

    while guard.y_pos >= 0
        && guard.y_pos < board.len() as i32
        && guard.x_pos >= 0
        && guard.x_pos < board[guard.y_pos as usize].len() as i32
    {
        fourth_point = guard.calculate_fourth_point();
        guard.print_things();
        write_board_to_file(&board, "debut_board.txt");

        let x = guard.x_pos as usize;
        let y = guard.y_pos as usize;

        if board[y][x] != 'X' {
            board[y][x] = 'X';
            colored_fields += 1;
        }

        let next_x = guard.x_pos + guard.direction.0;
        let next_y = guard.y_pos + guard.direction.1;

        if next_y < 0
            || next_y >= board.len() as i32
            || next_x < 0
            || next_x >= board[next_y as usize].len() as i32
        {
            break; // Out of bounds
        }

        let ahead = board[next_y as usize][next_x as usize];

        // if the point ahead would result in a cycle then add 1 to the counter
        if guard.check_position(fourth_point.expect("wrong position")) == true {
            obstacle_counter += 1;
        }

        match ahead {
            '.' => {
                guard.step();
            }
            '#' => {
                guard.rotate();
                // appends the fourth point in the function
            }
            _ => {
                guard.step();
            }
        }
    }
    println!("Final Board:");
    for row in &board {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Total colored fields: {}", colored_fields);

    // do the simulation again but every time

    Ok(())
}
