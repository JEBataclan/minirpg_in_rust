pub struct Position {
    x_pos: i32,
    y_pos: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x_pos: 0,
            y_pos: 0,
        }
    }

    pub fn move_up(&mut self) {
        self.y_pos += 1;
    }

    pub fn move_down(&mut self) {
        self.y_pos -= 1;
    }

    pub fn move_left(&mut self) {
        self.x_pos -= 1;
    }

    pub fn move_right(&mut self) {
        self.x_pos += 1;
    }
}