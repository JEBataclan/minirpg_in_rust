pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_u32(value: u32) -> Option<Direction> {
        match value {
            1 => Some(Direction::North),
            2 => Some(Direction::South),
            3 => Some(Direction::East),
            4 => Some(Direction::West),
            _ => None,
        }
    }
}