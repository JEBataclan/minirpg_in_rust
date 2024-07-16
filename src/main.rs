use std::io::{self, Write};

fn main() {
    // let character = CharacterCreation::create_character();

    let mut input = String::new();

    loop {
        println!("1) Move 2) Rest 3) View Stats 4) Quit");

        print!(": ");
        let _ = io::stdout().flush();

        // Validate line
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to read line with Error: {}", err);
                input.clear();
                continue;
            },
        }

        // Validate input
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Failed to parse input with Error: {}", err);
                input.clear();
                continue;
            },
        };

        // Do stuff based on choice
        match choice {
            1 => println!("Moving!"),
            2 => println!("Resting!"),
            3 => println!("Viewing stats!"),
            4 => break,
            _ => {
                println!("Invalid choice. Try again.");
                continue;
            },
        }

        input.clear();
    }
}