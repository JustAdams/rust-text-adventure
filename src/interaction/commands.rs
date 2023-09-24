pub enum Action {
    MOVE,
    HELP,
    LOOK,
    QUIT,
    UNDEFINED,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    UP,
    DOWN,
    UNDEFINED,
}

pub fn get_command(s: &str) -> Action {
    let command = match s.trim() {
        "help" => Action::HELP,
        "move" => Action::MOVE,
        "look" => Action::LOOK,
        "quit" => Action::QUIT,
        _ => Action::UNDEFINED,
    };

    command
}

// convert input direction into the enum for standardization
pub fn get_direction(s: &str) -> Direction {
    let direction = match s.trim().to_lowercase().as_str() {
        "north" => Direction::NORTH,
        "south" => Direction::SOUTH,
        "east" => Direction::EAST,
        "west" => Direction::WEST,
        "up" => Direction::UP,
        "down" => Direction::DOWN,
        _ => Direction::UNDEFINED,
    };

    direction
}
