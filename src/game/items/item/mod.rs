use std::{error::Error, fmt::{self, Display}, str::Split};
use rand::Rng;

struct ItemIdentity {
    id: u32,
    name: String,
}

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn random_value(&self) -> i32 {
        rand::thread_rng().gen_range(self.min..=self.max)
    }
}

struct WeaponDeserializeError(String);

impl fmt::Display for WeaponDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
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
            None => todo!(),
        };

        // Parse Name
        let name: String = match string.next() {
            Some(string) => string.to_string(),
            None => todo!(),
        };

        // Parse Mininimum Damage
        let min_damage: i32 = match string.next() {
            Some(string) => {
                let id: i32 = match string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse ID.")?,
                };

                id
            },
            None => todo!(),
        };

        // Parse Maximum Damage
        let max_damage: i32 = match string.next() {
            Some(string) => {
                let id: i32 = match string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return Err("Failed to parse ID.")?,
                };

                id
            },
            None => todo!(),
        };
        
        Ok(Weapon::new(id, name, min_damage, max_damage))
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("id: {}, name: {}, min_damage: {}, max_damage: {}", self.identity.id, self.identity.name, self.range.min, self.range.max);
        Ok(())
    }
}

struct Armor {
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
}
