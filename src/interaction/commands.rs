
pub enum Action {
    MOVE,
    HELP,
    LOOK,
    QUIT,
    UNDEFINED
}

pub fn get_command(s: &str) -> Action {
    let command = match s.trim() {
        "help" => Action::HELP,
        "move" => Action::MOVE,
        "look" => Action::LOOK,
        "quit" => Action::QUIT,
        _ => Action::UNDEFINED
    };

    return command;
}