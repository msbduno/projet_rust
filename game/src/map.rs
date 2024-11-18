use rand::Rng;

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map {
            width,
            height,
            tiles: vec![vec!['.'; width]; height],
        };
        map.generate_walls();
        map
    }

    fn generate_walls(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.width {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            self.tiles[y][x] = '#';
        }
    }

    pub fn display(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{} ", tile);
            }
            println!();
        }
    }
//emoji for walls
    pub fn is_valid_move(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.tiles[y][x] != '#'
    }
//emoji for player
    pub fn place_player(&mut self, x: usize, y: usize) {
        self.tiles[y][x] = '😃';
    }
//emogi for monster
    pub fn place_monster(&mut self, x: usize, y: usize) {
        self.tiles[y][x] = '👾';
    }

    pub fn clear_position(&mut self, x: usize, y: usize) {
        self.tiles[y][x] = '.';
    }

    pub fn get_random_empty_position(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if self.tiles[y][x] == '.' {
                return (x, y);
            }
        }
    }
}
