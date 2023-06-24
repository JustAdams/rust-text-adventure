pub struct Room {
    pub name: String,
    pub description: String
}

impl Room {
    pub fn get_description(&self) -> () {
        println!("{}", self.description);
    }
}
