use std::io::{self, Write};

pub struct Identity {
    pub name: String,
}

impl Identity {
    pub fn new(name: String) -> Identity {
        Identity {
            name: name,
        }
    }

    pub fn create() -> Identity {
        loop {
            print!("Input a name for your character: ");
            let _ = io::stdout().flush();
        
            let mut input = String::new();
        
            match io::stdin().read_line(&mut input) {
                Ok(_) => return Identity::new(input),
                Err(err) => {
                    println!("Failed to read line with Error: {}", err);
                    input.clear();
                    continue;
                },
            }
        }
    }
}