use crate::game::utils::input_u32;

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn decide() -> Direction {
        let mut input: String = String::new();
        
        loop {
            // What if to retract moving our player? (thinking)
            println!("1) North 2) East 3) West 4) South");

            let direction: u32 = input_u32(&mut input);
    
            match Self::from_u32(direction) {
                Some(direction) => return direction,
                None => println!("Invalid direction."),
            }
    
            input.clear();
        }
    }

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