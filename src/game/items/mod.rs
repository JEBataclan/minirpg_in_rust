mod item;

use std::{fs::File, io::Read, path::Path};
use item::weapon::Weapon;
use item::armor::Armor;

pub struct Items {
    weapons: Vec<Weapon>,
    armors: Vec<Armor>,
}

impl Items {
    pub fn new() -> Items {
        Items {
            weapons: Vec::new(),
            armors: Vec::new(),
        }
    }

    pub fn load_weapons(&mut self) {
        let path: &Path = Path::new("src/game/items/weapons.txt");

        let mut file: File = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to open file with error: {}", err);
                return;
            },
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to read contents of file with error: {}", err);
                return;
            },
        }

        for (i, line) in content.lines().enumerate() {
            // println!("Parsine line {}: {}", i, line);

            let split = line.split(",");

            match Weapon::from_split_string(split) {
                Ok(weapon) => self.weapons.push(weapon),
                Err(err) => println!("Failed to parse line to a struct Weapon. Line = \"{}\", Error = \"{}\"", line, err),
            }
        }
    }

    pub fn display_weapons(&mut self) {
        for weapon in &self.weapons {
            println!("{:?}", weapon);
        }
    }

    pub fn load_armors(&mut self) {
        let path: &Path = Path::new("src/game/items/armors.txt");

        let mut file: File = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to open file with error: {}", err);
                return;
            },
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to read contents of file with error: {}", err);
                return;
            },
        }

        for (i, line) in content.lines().enumerate() {
            // println!("Parsine line {}: {}", i, line);

            let split = line.split(",");

            match Armor::from_split_string(split) {
                Ok(armor) => self.armors.push(armor),
                Err(err) => println!("Failed to parse line to a struct Armor. Line = \"{}\", Error = \"{}\"", line, err),
            }
        }
    }

    pub fn display_armors(&mut self) {
        for armor in &self.armors {
            println!("{:?}", armor);
        }
    }
}