pub struct Room {
    pub name: String,
    pub description: String
}

impl Room {
    pub fn new() -> Self {
        Self {
            name: "Empty Void".to_string(),
            description: "Blackness of the empty void surrounds you. You feel you shouldn't be here.".to_string()
        }
    }

    pub fn get_description(&self) -> () {
        println!("{}", self.description);
    }
}
