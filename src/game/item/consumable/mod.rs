use super::Item;

pub struct Consumable {
    id: u32,
    name: String,
    value: i32,
}

impl Consumable {
    pub fn new(id: u32, name: String, value: i32) -> Consumable {
        Consumable {
            id: id,
            name: name,
            value: value,
        }
    }
}

impl Item for Consumable {
    fn print_description(&self) {
        println!("Consumable: ID = {} | Name = {} | Value = {}", self.id, self.name, self.value);
    }
}