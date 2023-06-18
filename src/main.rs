pub mod input_reader;

fn main() {
    println!("Starting game...");
    
    let mut play_game = true;

    while play_game {
        print!(">");

        // get user input
        let mut user_input = input_reader::read_user_input();

        println!("{}", user_input);

        // process users input by matching to a known command
        match user_input {
            Commands::HELP => println!("You need help");
            _ => {}
        }

        break;
    }
}
