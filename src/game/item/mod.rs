pub mod consumable;
pub mod weapon;
pub mod armor;

pub trait Item {
    fn print_description(&self);
}