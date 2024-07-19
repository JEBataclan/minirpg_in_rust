use rand::Rng;

struct MonsterIdentity {
    name: String,
}

pub struct Monster {
    identity: MonsterIdentity,
}

impl Monster {
    pub fn name(&self) -> &String {
        &self.identity.name
    }

    pub fn new(name: String) -> Monster {
        Monster {
            identity: MonsterIdentity {
                name: name,
            }
        }
    }

    pub fn new_orc() -> Monster {
        Self::new(String::from("Orc"))
    }

    pub fn new_goblin() -> Monster {
        Self::new(String::from("Goblin"))
    }

    pub fn new_ogre() -> Monster {
        Self::new(String::from("Ogre"))
    }

    pub fn new_orc_lord() -> Monster {
        Self::new(String::from("Orc Lord"))
    }
}

pub fn check_random_encounter() -> Option<Monster> {
    let num: i32 = rand::thread_rng().gen_range(0..=20);

    let monster = {
        if num >= 6 && num <= 10 {
            Some(Monster::new_orc())
        } else if num >= 11 && num <= 15 {
            Some(Monster::new_goblin())
        } else if num >= 16 && num <= 19 {
            Some(Monster::new_ogre())
        } else if num == 20 {
            Some(Monster::new_orc_lord())
        } else {
            None
        }
    };

    monster
}