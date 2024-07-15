use crate::identity::Identity;
use crate::position::Position;
use crate::inventory::Inventory;

pub struct Character {
    identity: Identity,
    position: Position,
    inventory: Inventory,
}

impl Character {
    pub fn new(identity: Identity) -> Character {
        Character {
            identity: identity,
            position: Position::new(),
            inventory: Inventory::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.identity.name
    }
}