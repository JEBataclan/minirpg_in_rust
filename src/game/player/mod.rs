mod identity;
pub mod position;
mod class;
// mod inventory;

use class::PlayerClass;
use identity::Identity;
use position::{direction::Direction, Position};

use super::items::item::weapon::Weapon;
use super::items::item::armor::Armor;
use super::items::Items;
// use inventory::Inventory;

#[derive(Debug)]
pub struct Player {
    identity: Identity,
    position: Position,
    class: PlayerClass,
    weapon: Weapon,
    armor: Armor,
    // inventory: Inventory,
}

impl Player {
    pub fn create(items: &Items) -> Player {
        let identity: Identity = Identity::create();

        let class = PlayerClass::choose();

        let weapon = match class {
            PlayerClass::Fighter => items.get_weapon_with_id(0),
            PlayerClass::Wizard => items.get_weapon_with_id(1),
            PlayerClass::Cleric => items.get_weapon_with_id(2),
            PlayerClass::Thief => items.get_weapon_with_id(3),
        };

        let armor = match class {
            PlayerClass::Fighter => items.get_armor_with_id(0),
            PlayerClass::Wizard => items.get_armor_with_id(1),
            PlayerClass::Cleric => items.get_armor_with_id(2),
            PlayerClass::Thief => items.get_armor_with_id(3),
        };

        Player {
            identity: identity,
            position: Position::new(),
            class: class,
            weapon: weapon.unwrap().clone(),
            armor: armor.unwrap().clone(),
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