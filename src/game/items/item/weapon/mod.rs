use std::{error::Error, str::Split};
use rand::Rng;

use super::ItemIdentity;

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn random_value(&self) -> i32 {
        rand::thread_rng().gen_range(self.min..=self.max)
    }
}

impl Clone for Range {
    fn clone(&self) -> Self {
        Self { min: self.min.clone(), max: self.max.clone() }
    }
}

pub struct Weapon {
    identity: ItemIdentity,
    range: Range,
}

impl Weapon {
    pub fn new(id: u32, name: String, min_damage: i32, max_damage: i32) -> Weapon {
        Weapon {
            identity: ItemIdentity {
                id: id,
                name: name,
            },
            range: Range {
                min: min_damage,
                max: max_damage,
            }
        }
    }

    pub fn from_split_string(mut string: Split<&str>) -> Result<Weapon, Box<dyn Error>> {        
        // Parse ID
        let id: u32 = match string.next() {
            Some(string) => {
                let id: u32 = match string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse ID.")?,
                };

                id
            },
            None => return Err("Failed to parse ID.")?,
        };

        // Parse Name
        let name: String = match string.next() {
            Some(string) => string.to_string(),
            None => return Err("Failed to parse Name.")?,
        };

        // Parse Mininimum Damage
        let min_damage: i32 = match string.next() {
            Some(string) => {
                let id: i32 = match string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse Minimum Damage.")?,
                };

                id
            },
            None => return Err("Failed to parse Minimum Damage.")?,
        };

        // Parse Maximum Damage
        let max_damage: i32 = match string.next() {
            Some(string) => {
                let id: i32 = match string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse Maximum Damage.")?,
                };

                id
            },
            None => return Err("Failed to parse Maximum Damage.")?,
        };
        
        Ok(Weapon::new(id, name, min_damage, max_damage))
    }

    pub fn id(&self) -> u32 {
        self.identity.id
    }
}

impl Clone for Weapon {
    fn clone(&self) -> Self {
        Self { identity: self.identity.clone(), range: self.range.clone() }
    }
}