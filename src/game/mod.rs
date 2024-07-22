mod items;
// mod monster;

use std::io::{self, Write};
use items::Items;
use rand::Rng;

enum Choice {
    Move,
    Rest,
    ViewStat,
    Quit,
}

impl Choice {
    pub fn from_u32(value: u32) -> Option<Choice> {
        match value {
            1 => Some(Choice::Move),
            2 => Some(Choice::Rest),
            3 => Some(Choice::ViewStat),
            4 => Some(Choice::Quit),
            _ => None,
        }
    }
}

pub fn start() {
    let mut items: Items = Items::new();
    items.load_weapons();


    // let mut input = String::new();

    // loop {
    //     println!("1) Move 2) Rest 3) View Stats 4) Quit");

    //     let choice: u32 = input_u32(&mut input);

    //     // Do stuff based on choice
    //     match Choice::from_u32(choice) {
    //         None => println!("Invalid choice."),
    //         Some(Choice::Move) => move_player(),
    //         Some(Choice::Rest) => rest(),
    //         Some(Choice::ViewStat) => {},
    //         Some(Choice::Quit) => break,
    //     }

    //     input.clear();
    // }
}

// fn move_player() {
//     let _monster = match monster::check_random_encounter() {
//         Some(monster) => println!("Encountered {}", monster.name()),
//         None => (),
//     };
// }

// fn rest() {
//     let num = rand::thread_rng().gen_range(0..=100);
    
//     if num >= 0 && num <= 25 {
//         // Battle
//         let _monster = match monster::check_random_encounter() {
//             None => (),
//             Some(monster) => {
//                 println!("Encountered {}", monster.name())
//             },
//         };
//     } else {
//         // Rest
//     }
// }

fn input_string(input: &mut String) {
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

fn input_u32(input: &mut String) -> u32 {
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