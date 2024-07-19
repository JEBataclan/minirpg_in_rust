use rand::Rng;

struct ItemIdentity {
    id: u32,
    name: String,
}

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn random_value(&self) -> i32 {
        rand::thread_rng().gen_range(self.min..=self.max)
    }
}

struct Weapon {
    identity: ItemIdentity,
    range: Range,
}

impl Weapon {
    pub fn new(id: u32, name: String, min_damage: i32, max_damage: i32) -> Weapon {
        Weapon {
            identity: ItemIdentity {
                id: id,
                name: name,
            },
            range: Range {
                min: min_damage,
                max: max_damage,
            }
        }
    }
}

struct Armor {
    identity: ItemIdentity,
    value: i32,
}

impl Armor {
    pub fn new(id: u32, name: String, value: i32) -> Armor {
        Armor {
            identity: ItemIdentity {
                id: id,
                name: name,
            },
            value: value,
        }
    }
}
