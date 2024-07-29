pub mod weapon;
pub mod armor;

struct ItemIdentity {
    id: u32,
    name: String,
}

impl Clone for ItemIdentity {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone() }
    }
}