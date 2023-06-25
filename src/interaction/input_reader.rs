use std::io::{self, Write};

pub fn read_user_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    return user_input;
}
