use crate::game::utils::input_u32;

pub enum Action {
    Move,
    Rest,
    ViewStat,
    Quit,
}

impl Action {
    pub fn decide() -> Action {
        let mut input: String = String::new();
        
        loop {
            println!("1) Move 2) Rest 3) View Stats 4) Quit");

            let action: u32 = input_u32(&mut input);
    
            // Do stuff based on choice
            match Self::from_u32(action) {
                Some(action) => return action,
                None => println!("Invalid action."),
            }
    
            input.clear();
        }
    }

    fn from_u32(value: u32) -> Option<Action> {
        match value {
            1 => Some(Action::Move),
            2 => Some(Action::Rest),
            3 => Some(Action::ViewStat),
            4 => Some(Action::Quit),
            _ => None,
        }
    }
}