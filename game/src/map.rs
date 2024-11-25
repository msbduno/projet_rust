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
        map.generate_walls();
        map
    }

    fn generate_walls(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.width {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            self.tiles[y][x] = '⬜';
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

    pub fn is_valid_move(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.tiles[y][x] != '⬜'
    }

    pub fn place_player(&mut self, x: usize, y: usize) {
        // choisir le bon caractère pour le joueur
        if players.espece == Espece::Homme {
            self.tiles[y][x] = '🧑';
        } else if players.espece == Espece::Sorciere {
            self.tiles[y][x] = '🧙';
        } else if players.espece == Espece::Elfe {
            self.tiles[y][x] = '🧚';
        } else if players.espece == Espece::Chevalier {
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
}