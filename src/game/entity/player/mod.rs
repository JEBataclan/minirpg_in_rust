mod identity;
pub mod position;
mod class;
// mod inventory;

use class::PlayerClass;
use identity::Identity;
use position::{direction::Direction, Position};

use crate::game::items::item::weapon::Weapon;
use crate::game::items::item::armor::Armor;
use crate::game::items::Items;
use super::health::{self, Health};
use super::Damageable;

// use inventory::Inventory;

pub struct Player {
    identity: Identity,
    position: Position,
    class: PlayerClass,
    health: Health,
    weapon: Weapon,
    armor: Armor,
    // inventory: Inventory,
}

impl Player {
    pub fn create(items: &Items) -> Player {
        let identity: Identity = Identity::create();

        let class: PlayerClass = PlayerClass::choose();

        let health: Health;
        let weapon: Option<&Weapon>;
        let armor: Option<&Armor>;

        match class {
            PlayerClass::Fighter => {
                health = Health::new(20);
                weapon = items.get_weapon_with_id(0);
                armor = items.get_armor_with_id(0);
            },

            PlayerClass::Wizard => {
                health = Health::new(10);
                weapon = items.get_weapon_with_id(1);
                armor = items.get_armor_with_id(1);
            },

            PlayerClass::Cleric => {
                health = Health::new(15);
                weapon = items.get_weapon_with_id(2);
                armor = items.get_armor_with_id(2);
            },

            PlayerClass::Thief => {
                health = Health::new(12);
                weapon = items.get_weapon_with_id(3);
                armor = items.get_armor_with_id(3);
            },

        }

        Player {
            identity: identity,
            position: Position::new(),
            class: class,
            health: health,
            weapon: weapon.unwrap().clone(), // TODO: Handle this
            armor: armor.unwrap().clone(), // TODO: Handle this
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

impl Damageable for Player {
    fn receive_damage(&mut self, amount: i32) {
        self.health.decrease(amount);
    }
}