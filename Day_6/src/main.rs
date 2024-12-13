// get the board set up
// make struct for guard
// make a while loop and on every field
// 1. check what it is
//  if empty
//      nothing
//  if obstacle
//      rotate 90 degrees
//      nothing
//
//  make move
//  check out of bounds conditions
//  x_coord and y_coord

struct Guard {
    x_pos: i32,
    y_pos: i32,
    direction: (i8, i8),
}

impl Guard {
    fn new(&mut self, x_pos: i32, y_pos: i32, direction: (i8, i8)) {
        // initialise the guard struct
        self.x_pos = x_pos;
        self.y_pos = y_pos;
        self.direction = direction;
    }
    fn rotate(&mut self) {
        // matrix multiplication
        //[0, -1] [x]= [-x]
        //[1,  0] [y]= [ y]
        self.x_pos = -self.x_pos;
    }
}

fn main() {
    println!("Hello, world!");
}
