pub mod utils;
mod items;
mod entity;
mod action;
mod battle;

use items::Items;
use entity::player::{position::direction::Direction, Player};
use entity::monster;
use action::Action;
use rand::Rng;

pub fn start() {
    let mut items: Items = Items::new();
    items.load_weapons();
    items.load_armors();

    let mut player: Player = Player::create(&items);

    loop {
        match Action::decide() {
            Action::Move => move_player(&mut player),
            Action::Rest => rest(),
            Action::ViewStat => {},
            Action::Quit => break,
        }
    }
}

fn move_player(player: &mut Player) {
    // TODO: Maybe just say player.move() then let the
    // deciding happen internatlly instead of here?
    // That sounds a lot better than deciding it here.
    let direction: Direction = Direction::decide();
    player.move_position(direction);

    match monster::check_random_encounter() {
        Some(mut monster) => battle::commence(player, &mut monster),
        None => return,
    };
}

fn rest() {
    let num = rand::thread_rng().gen_range(0..=100);
    
    if num >= 0 && num <= 25 {
        // Battle
        let _monster = match monster::check_random_encounter() {
            None => (),
            Some(monster) => {
                println!("Encountered {}", monster.name())
            },
        };
    } else {
        // Rest
    }
}