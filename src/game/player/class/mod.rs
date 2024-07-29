use std::io::{self, Write};

pub enum PlayerClass {
    Fighter,
    Wizard,
    Cleric,
    Thief,
}

impl PlayerClass {
    pub fn choose() -> PlayerClass {
        println!("Choose a class:");
        println!("1) Fighter 2) Wizard 3) Cleric 4) Thief");

        let mut input = String::new();

        loop {
            let choice: u32 = input_u32(&mut input);
    
            // Do stuff based on choice
            match PlayerClass::from_u32(choice) {
                None => println!("Invalid choice."),
                Some(PlayerClass::Fighter) => return PlayerClass::Fighter,
                Some(PlayerClass::Wizard) => return PlayerClass::Wizard,
                Some(PlayerClass::Cleric) => return PlayerClass::Cleric,
                Some(PlayerClass::Thief) => return PlayerClass::Thief,
            }
    
            input.clear();
        }
    }

    pub fn from_u32(value: u32) -> Option<PlayerClass> {
        match value {
            1 => Some(PlayerClass::Fighter),
            2 => Some(PlayerClass::Wizard),
            3 => Some(PlayerClass::Cleric),
            4 => Some(PlayerClass::Thief),
            _ => None,
        }
    }
}

fn input_string(input: &mut String) {
    loop {
        print!(": "); 
        let _ = io::stdout().flush();

        // Validate line
        match io::stdin().read_line(input) {
            Ok(_) => break,
            Err(err) => {
                println!("Failed to read line with Error: {}", err);
                input.clear();
                continue;
            },
        }
    }
}

fn input_u32(input: &mut String) -> u32 {
    loop {
        input_string(input);

        // Validate input
        let _choice: u32 = match input.trim().parse() {
            Ok(num) => {
                input.clear();
                return num;
            },
            Err(err) => {
                println!("Failed to parse input with Error: {}", err);
                input.clear();
                continue;
            },
        };
        
    }
}