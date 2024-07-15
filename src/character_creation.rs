use std::io::{self, Write};

use crate::character::Character;
use crate::identity::Identity;

pub struct CharacterCreation;

impl CharacterCreation {
    pub fn create_character() -> Character {
        println!("=== CHARACTER CREATION ===");

        let identity: Identity = Self::create_identity();

        Character::new(identity)
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
}