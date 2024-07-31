pub mod player;
pub mod monster;
pub mod health;

pub trait Damageable {
    fn receive_damage(&mut self, amount: i32);
}