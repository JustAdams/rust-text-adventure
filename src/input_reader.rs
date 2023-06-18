

use std::io;

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    return user_input;
}
