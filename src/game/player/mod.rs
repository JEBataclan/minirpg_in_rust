mod identity;
pub mod position;
// mod inventory;

use identity::Identity;
use position::{direction::Direction, Position};
// use inventory::Inventory;

pub struct Player {
    identity: Identity,
    position: Position,
    // inventory: Inventory,
}

impl Player {
    pub fn create() -> Player {
        Player {
            identity: Identity::create(),
            position: Position::new(),
            // inventory: Inventory::new(),
        }
    }

    pub fn move_position(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.position.move_north(),
            Direction::South => self.position.move_south(),
            Direction::East => self.position.move_east(),
            Direction::West => self.position.move_west(),
        }
    }
}