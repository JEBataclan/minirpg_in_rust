pub mod direction;

#[derive(Debug)]
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

    pub fn move_north(&mut self) {
        self.move_internal(0, 1);
    }

    pub fn move_south(&mut self) {
        self.move_internal(0, -1);
    }

    pub fn move_west(&mut self) {
        self.move_internal(-1, 0);
    }

    pub fn move_east(&mut self) {
        self.move_internal(1, 0);
    }

    fn move_internal(&mut self, x: i32, y: i32) {
        self.x_pos += x;
        self.y_pos += y;

        println!("New position: X = {}, Y = {}", self.x_pos, self.y_pos);
    }
}