use std::io::{self, Write};

pub fn input_string(input: &mut String) {
    loop {
        print!(": "); 
        let _ = io::stdout().flush();

        // Validate line
        match io::stdin().read_line(input) {
            Ok(_) => break,
            Err(err) => {
                println!("Failed to read line with Error: {}", err);
                input.clear();
                continue;
            },
        }
    }
}

pub fn input_u32(input: &mut String) -> u32 {
    loop {
        input_string(input);

        // Validate input
        let _choice: u32 = match input.trim().parse() {
            Ok(num) => {
                input.clear();
                return num;
            },
            Err(err) => {
                println!("Failed to parse input with Error: {}", err);
                input.clear();
                continue;
            },
        };
        
    }
}