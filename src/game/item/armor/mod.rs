use super::Item;

pub struct Armor {
    id: u32,
    name: String,
    value: i32,
}

impl Armor {
    pub fn new(id: u32, name: String, value: i32) -> Armor {
        Armor {
            id: id,
            name: name,
            value: value,
        }
    }
}

impl Item for Armor {
    fn print_description(&self) {
        println!("Armor: ID = {} | Name = {} | Value = {}", self.id,self. name, self.value);
    }
}