
pub enum Action {
    MOVE,
    HELP,
    QUIT,
    UNDEFINED
}

pub fn get_command(s: &str) -> Action {
    let command = match s.trim() {
        "help" => Action::HELP,
        "move" => Action::MOVE,
        "quit" => Action::QUIT,
        _ => Action::UNDEFINED
    };

    return command;
}