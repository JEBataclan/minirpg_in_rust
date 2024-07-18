use super::Item;

use rand::Rng;

pub struct Weapon {
    id: u32,
    name: String,
    damage: RandRange,
}

impl Weapon {
    pub fn new(id: u32, name: String, min: i32, max: i32) -> Weapon {
        Weapon {
            id: id,
            name: name,
            damage: RandRange::new(min, max),
        }
    }

    pub fn random_damage(&self) -> i32 {
        rand::thread_rng().gen_range(self.damage.min()..self.damage.max())
    }
}

impl Item for Weapon {
    fn print_description(&self) {
        println!("Weapon: ID = {} | Name = {} | Min Damage = {} | Max Damage = {}", self.id, self.name, self.damage.min(), self.damage.max());
    }
}

pub struct RandRange {
    min: i32,
    max: i32,
}

impl RandRange {
    pub fn new(min: i32, max: i32) -> RandRange {
        RandRange{
            min: min,
            max: max,
        }
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn max(&self) -> i32 {
        self.max
    }
}