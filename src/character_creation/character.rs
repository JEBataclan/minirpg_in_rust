mod position;
mod inventory;

use std::io::{self, Write};
use super::identity::Identity;
use position::Position;
use inventory::Inventory;

pub struct Character {
    identity: Identity,
    position: Position,
    inventory: Inventory,
}

impl Character {
    pub fn new() -> Character {
        Character {
            identity: Self::create_identity(),
            position: Position::new(),
            inventory: Inventory::new(),
        }
    }

    fn create_identity() -> Identity {
        loop {
            print!("Input a name for your character: ");
            let _ = io::stdout().flush();
        
            let mut input = String::new();
        
            match io::stdin().read_line(&mut input) {
                Ok(_) => return Identity::new(input),
                Err(err) => {
                    println!("Failed to read line with Error: {}", err);
                    input.clear();
                    continue;
                },
            }
        }
    }

    pub fn name(&self) -> &String {
        &self.identity.name
    }
}