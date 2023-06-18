use std::process;

use crate::Commands::Action;

mod input_reader;
mod Commands;

fn main() {
    println!("Starting game...");
    
    let mut play_game = true;

    while play_game {
        // get user input
        let user_input = input_reader::read_user_input();

        // process users input by matching to a known command
        match Commands::get_command(&user_input) {
            Action::HELP => println!("You need help"),
            Action::MOVE => println!("You move"),
            Action::QUIT => { play_game = false },
            _ => println!("I'm not sure what you mean by {}.", user_input)
        }
    }
    
    quit_game();
}

// End the game
fn quit_game() {
    println!("Thank you for playing!");
    process::exit(0);
}
