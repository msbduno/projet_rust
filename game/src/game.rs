
use crate::map::Map;
use crate::player::Player;
use crate::monster::Monster;

#[derive(PartialEq)] 
pub enum GameState {
    Running,
    Combat,
    GameOver,
}

pub struct Game {
    pub map: Map,
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub state: GameState,
    pub score: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            map: Map::new(10, 10),
            player: Player::default(),
            monsters: Vec::new(),
            state: GameState::Running,
            score: 0,
        }
    }

    pub fn initialize_player(&mut self, name: &str) {
        self.player = Player::new(name);
        self.map.place_player(self.player.x, self.player.y);
    }

    pub fn spawn_random_monster(&mut self) {
        let mut _rng = rand::thread_rng();
        let (x, y) = self.map.get_random_empty_position();
        let monster = Monster::new(x, y);
        self.monsters.push(monster);
        self.map.place_monster(x, y);
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let new_x = (self.player.x as i32 + dx) as usize;
        let new_y = (self.player.y as i32 + dy) as usize;

        if self.map.is_valid_move(new_x, new_y) {
            self.map.clear_position(self.player.x, self.player.y);
            self.player.x = new_x;
            self.player.y = new_y;
            self.map.place_player(new_x, new_y);

            // Vérifier s'il y a un monstre à la nouvelle position
            if let Some(monster_idx) = self.find_monster_at(new_x, new_y) {
                self.start_combat(monster_idx);
            }
        }
    }

    pub fn find_monster_at(&self, x: usize, y: usize) -> Option<usize> {
        self.monsters.iter().position(|m| m.x == x && m.y == y)
    }

    pub fn start_combat(&mut self, monster_idx: usize) {
        self.state = GameState::Combat;
        let monster = &self.monsters[monster_idx];
        println!("Combat contre un monstre niveau {}!", monster.level);

        // Logique de combat simple
        if self.player.attack > monster.defense {
            println!("Victoire! +{} points", monster.level * 10);
            self.score += monster.level * 10;
            self.monsters.remove(monster_idx);
        } else {
            println!("Défaite...");
            self.state = GameState::GameOver;
        }
    }

    pub fn display(&self) {
        //effacer l'ecran
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("\nScore: {}", self.score);
        println!("Vie: {}/{}", self.player.health, self.player.max_health);
        self.map.display();
    }

    pub fn show_inventory(&self) {
        println!("Inventaire de {}:", self.player.name);
        println!("Niveau: {}", self.player.level);
        println!("Attaque: {}", self.player.attack);
        println!("Défense: {}", self.player.defense);
    }

    pub fn show_help(&self) {
        println!("Commandes:");
        println!("z/s/q/d - Se déplacer");
        println!("i - Afficher l'inventaire");
        println!("h - Afficher l'aide");
        println!("x - Quitter");
        println!("\nSymboles:");
        println!("@ - Joueur");
        println!("M - Monstre");
        println!("# - Mur");
    }
}