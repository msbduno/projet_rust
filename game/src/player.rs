pub struct Player {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
    pub attack: u32,
    pub defense: u32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            x: 0,
            y: 0,
            health: 100,
            max_health: 100,
            level: 1,
            attack: 10,
            defense: 5,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new("Unknown")
    }
}