use std::{process, io::{stdout, Write}};

mod interaction {
    pub mod input_reader;
    pub mod commands;
}

mod world {
    pub mod room;
}

use crate::{world::room::Room, interaction::commands::{Action, self}};

fn main() {
    println!("Starting game...");
    
    let mut play_game = true;

    let sample_room = Room {
        name: String::from("Sample Room"),
        description: String::from("This is the description for the sample room.")
    };

    while play_game {

        // get user input
        let user_input = interaction::input_reader::read_user_input();

        // process users input by matching to a known command
        match commands::get_command(&user_input) {
            Action::HELP => println!("You need help"),
            Action::MOVE => println!("You move"),
            Action::LOOK => sample_room.get_description(),
            Action::QUIT => { play_game = false },
            _ => println!("I'm not sure what you mean by {}", user_input)
        }
    }
    
    quit_game();
}

// End the game
fn quit_game() {
    println!("Thank you for playing!");
    process::exit(0);
}
