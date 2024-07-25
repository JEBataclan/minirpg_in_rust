pub mod item;

use std::collections::HashMap;
use std::{fs::File, io::Read, path::Path};
use item::weapon::Weapon;
use item::armor::Armor;

pub struct Items {
    weapons: HashMap<u32, Weapon>,
    armors: HashMap<u32, Armor>,
}

impl Items {
    pub fn new() -> Items {
        Items {
            weapons: HashMap::new(),
            armors: HashMap::new(),
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
                Ok(weapon) => match self.weapons.insert(weapon.id(), weapon) {
                    Some(_) => (), // Maybe print updated weapon with this id?
                    None => (), // Maybe print successfully added?
                },
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
                Ok(armor) => match self.armors.insert(armor.id(), armor) {
                    Some(_) => (), // Maybe print updated armor with this id?
                    None => (), // Maybe print successfully added?
                },
                Err(err) => println!("Failed to parse line to a struct Armor. Line = \"{}\", Error = \"{}\"", line, err),
            }
        }
    }

    pub fn display_armors(&mut self) {
        for armor in &self.armors {
            println!("{:?}", armor);
        }
    }

    pub fn get_weapon_with_id(&self, id: u32) -> Option<&Weapon> {
        self.weapons.get(&id)
    }

    pub fn get_armor_with_id(&self, id: u32) -> Option<&Armor> {
        self.armors.get(&id)
    }
}