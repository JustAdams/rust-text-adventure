
enum Commands {
    MOVE,
    WALK,
    HELP
}

impl Commands {
    fn get_command(s: &str) -> Result<Command, InputError> {
        match s {
            "help" => Ok(Commands::HELP),
            _ => Err(InputError)
        }
    }
}