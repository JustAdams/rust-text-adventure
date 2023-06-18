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
        match Commands::get_command(&user_input) {
            "help" => println!("You need help"),
            _ => println!("I'm not sure what you mean by {}.", user_input)
        }

        break;
    }
}
