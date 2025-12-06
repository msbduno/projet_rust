use rand::Rng;
use crate::player::{Player, Espece};

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
            tiles: vec![vec!['⬛'; width]; height],
        };
        map.tiles[map.height - 1][map.width - 1] = '🚪';
        map.generate_walls_and_icons();
        map
    }

    fn generate_walls_and_icons(&mut self) {
        let mut rng = rand::thread_rng();
        
       
        let num_hearts = rng.gen_range(1..4);
        let num_flames = rng.gen_range(1..4);

        
        for _ in 0..self.width {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if x != self.width - 1 && y != self.height - 1 {
                self.tiles[y][x] = '⬜';
            } else {
                self.tiles[y][x] = '⬛';
            }
        }

        
        for _ in 0..num_hearts {
            loop {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
                if self.tiles[y][x] == '⬛' {
                    self.tiles[y][x] = '🍗';
                    break;
                }
            }
        }

      
        for _ in 0..num_flames {
            loop {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
                if self.tiles[y][x] == '⬛' {
                    self.tiles[y][x] = '🔥';
                    break;
                }
            }
        }
    }

    pub fn display(&self) {
        for row in &self.tiles {
            for &tile in row.iter() {
                print!("{}", tile);
            }
            print!("\r\n"); // Explicit carriage return + newline
        }
    }

    pub fn is_valid_move(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.tiles[y][x] != '⬜'
    }

    pub fn place_player(&mut self, x: usize, y: usize, player: &Player) {
        
        if player.espece == Espece::Homme {
            self.tiles[y][x] = '🧑';
        } else if player.espece == Espece::Sorciere {
            self.tiles[y][x] = '🧙';
        } else if player.espece == Espece::Elfe {
            self.tiles[y][x] = '🧚';
        } else if player.espece == Espece::Chevalier {
            self.tiles[y][x] = '🧝';
        }
    }

    pub fn place_monster(&mut self, x: usize, y: usize) {
        self.tiles[y][x] = '👾';
    }

    pub fn clear_position(&mut self, x: usize, y: usize) {
        self.tiles[y][x] = '⬛';
    }

    pub fn get_random_empty_position(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if self.tiles[y][x] == '⬛' {
                return (x, y);
            }
        }
    }

    
    pub fn is_on_door(&self, x: usize, y: usize) -> bool {
        // Vérifier que les coordonnées sont dans la carte
        x < self.width && y < self.height && 
        // Vérifier spécifiquement la porte dans le coin inférieur droit
        (x == self.width - 1 && y == self.height - 1)
    }

    pub fn is_health_icon(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.tiles[y][x] == '🍗'
    }

    pub fn is_damage_icon(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.tiles[y][x] == '🔥'
    }

    pub fn clear_special_icon(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height && 
           (self.tiles[y][x] == '🍗' || self.tiles[y][x] == '🔥') {
            self.tiles[y][x] = '⬛';
        }
    }

}