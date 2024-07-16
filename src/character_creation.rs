mod character;
mod identity;

use character::Character;

pub struct CharacterCreation;

impl CharacterCreation {
    pub fn create_character() -> Character {
        Character::new()
    }
}