use std::collections::HashMap;

use crate::interaction::commands::Direction;

pub struct Room {
    pub name: String,
    pub description: String,
    exits: HashMap<Direction, Room>,
}

impl Room {
    pub fn new(name: String, description: String) -> Room {
        Room {
            name: name,
            description: description,
            exits: HashMap::new(),
        }
    }

    pub fn get_description(&self) -> () {
        // display what the room looks like
        println!("{}", self.description);
        // display the exists and where they go for the room
        for (direction, room) in &self.exits {
            println!("You can go {:#?} to {:#?}", direction, room.name);
        }
    }

    pub fn add_exit(&self, direction: Direction, room: Room) -> () {}
}
