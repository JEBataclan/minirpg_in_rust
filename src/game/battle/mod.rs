use super::entity::player::Player;
use super::entity::monster::Monster;

enum BattleAction {
    Attack,
    Run,
}

pub fn commence(player: &mut Player, monster: &mut Monster) {
    // player decide

    // if player attacks
    // player.attack(monster)

    // if player runs
    // do rng, early return on success otherwise...

    // monster.attack(player)
}