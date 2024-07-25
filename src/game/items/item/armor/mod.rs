use std::{error::Error, str::Split};

use super::ItemIdentity;

#[derive(Debug)]
pub struct Armor {
    identity: ItemIdentity,
    value: i32,
}

impl Armor {
    pub fn new(id: u32, name: String, value: i32) -> Armor {
        Armor {
            identity: ItemIdentity {
                id: id,
                name: name,
            },
            value: value,
        }
    }

    pub fn from_split_string(mut string: Split<&str>) -> Result<Armor, Box<dyn Error>> {        
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

        // Parse Value
        let value: i32 = match string.next() {
            Some(string) => {
                let id: i32 = match string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse Minimum Damage.")?,
                };

                id
            },
            None => return Err("Failed to parse Minimum Damage.")?,
        };
        
        Ok(Armor::new(id, name, value))
    }

    pub fn id(&self) -> u32 {
        self.identity.id
    }
}

impl Clone for Armor  {
    fn clone(&self) -> Self {
        Self { identity: self.identity.clone(), value: self.value.clone() }
    }
}