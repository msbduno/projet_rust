use rand::Rng;

pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub level: u32,
    pub attack: u32,
    pub defense: u32,
}

impl Monster {
    pub fn new(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        let level = rng.gen_range(1..=3);
        
        Monster {
            x,
            y,
            level,
            attack: level * 5,
            defense: level * 3,
        }
    }
}