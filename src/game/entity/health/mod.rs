pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Health {
        Health {
            current: max,
            max: max,
        }
    }

    pub fn increase(&mut self, amount: i32) {
        self.current += amount;
    }

    pub fn decrease(&mut self, amount: i32) {
        self.current -= amount;
    }
}