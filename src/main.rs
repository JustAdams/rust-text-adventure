use std::process;

mod interaction {
    pub mod commands;
    pub mod input_reader;
}

mod world {
    pub mod room;
}

use crate::{
    interaction::commands::{self, Action},
    world::room::Room,
};

fn main() {
    println!("Starting game...");

    let mut play_game = true;

    // TODO: move these into a room builder
    let second_room = Room::new(
        String::from("The Second Room"),
        String::from("This looks like the second room. The world's a lot bigger than you thought!"),
    );

    let sample_room = Room::new(
        String::from("Sample Room"),
        String::from("This is the description for the sample room."),
    );

    while play_game {
        // get user input
        let user_input = interaction::input_reader::read_user_input();

        // process users input by matching to a known command
        match commands::get_command(&user_input) {
            Action::HELP => println!("You need help"),
            Action::MOVE => println!("You move"),
            Action::LOOK => sample_room.get_description(),
            Action::QUIT => play_game = false,
            _ => println!("I'm not sure what you mean by {}", user_input),
        }
    }

    quit_game();
}

// End the game
fn quit_game() {
    println!("Thank you for playing!");
    process::exit(0);
}
