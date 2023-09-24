use crate::world::room::Room;


pub struct GameManager {
    current_room: Room
}


impl GameManager {
    
    // Create a new game manager - singleton
    pub fn new() -> Self {
        Self {
            current_room: Room::new()
        }
    }

    // Return the room the player is currently in
    pub fn get_current_room(&self) -> &Room {
        &self.current_room
    }

    // Updates the room the player is currently in
    pub fn set_current_room(&mut self, room: Room) {
        self.current_room = room;
    }
}
