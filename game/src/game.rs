use crate::map::Map;
use crate::player::{Player, Espece};
use crate::monster::{Monster, MonsterSpecies};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)] 
pub enum GameState {
    Running,
    Combat,
    GameOver,
}

#[derive(PartialEq)]
pub enum PlayerCombatAction {
    Attack,
    SpecialAttack,
    Drink,
}

pub struct Game {
    pub map: Map,
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub state: GameState,
    pub score: u32,
    pub current_monster_index: Option<usize>,
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            map: Map::new(10, 10),
            player: Player::new("Default"),
            monsters: Vec::new(),
            state: GameState::Running,
            score: 0,
            current_monster_index: None,
        }
    }

    pub fn initialize_player(&mut self, name: &str) {
        self.player = Player::new(name);
        self.map.place_player(0, 0);
    }

    pub fn spawn_random_monster(&mut self) {
        let mut rng = rand::thread_rng();
        if self.monsters.len() < 5 {  // Limit number of monsters
            let (x, y) = self.map.get_random_empty_position();
            let mut monster = Monster::new(x, y);
            self.map.place_monster(x, y);
            self.monsters.push(monster);
        }
    }

    pub fn find_monster_at(&self, x: usize, y: usize) -> Option<usize> {
        self.monsters.iter().position(|m| m.x == x && m.y == y)
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.state == GameState::Combat {
            return;
        }

        let new_x = (self.player.x as i32 + dx) as usize;
        let new_y = (self.player.y as i32 + dy) as usize;

        if self.state == GameState::Running && self.map.is_valid_move(new_x, new_y) {
            self.map.clear_position(self.player.x, self.player.y);
            self.player.x = new_x;
            self.player.y = new_y;
            self.map.place_player(new_x, new_y);

            if let Some(monster_idx) = self.find_monster_at(new_x, new_y) {
                self.start_combat(monster_idx);
            }
        }
    }

    pub fn start_combat(&mut self, monster_idx: usize) {
        self.state = GameState::Combat;
        self.current_monster_index = Some(monster_idx);
        println!("Combat contre un {} niveau {}!", 
            match self.monsters[monster_idx].species {
                MonsterSpecies::Goblin => "Gobelin",
                MonsterSpecies::Orc => "Orc",
                MonsterSpecies::Skeleton => "Squelette",
                MonsterSpecies::Dragon => "Dragon",
            }, 
            self.monsters[monster_idx].level
        );
    }

    pub fn combat_turn(&mut self, player_action: PlayerCombatAction) {
        if self.state != GameState::Combat || self.current_monster_index.is_none() {
            return;
        }

        let monster_idx = self.current_monster_index.unwrap();
        let monster = &mut self.monsters[monster_idx];

        // Player's turn
        match player_action {
            PlayerCombatAction::Attack => {
                let damage = self.player.attack(monster);
                monster.receive_damage(damage);
            },
            PlayerCombatAction::SpecialAttack => {
                if self.player.attaque_speciale {
                    let damage = self.player.use_special_attack(monster);
                    monster.receive_damage(damage);
                } else {
                    println!("Attaque spéciale non disponible!");
                }
            },
            PlayerCombatAction::Drink => {
                self.player.drink_potion();
            }
        }

        // Monster's turn if still alive
        if monster.is_alive() {
            let mut rng = rand::thread_rng();
            let monster_action: i32 = rng.gen_range(0..10);

            let monster_damage = if monster_action < 2 && monster.special_attack_available {
                monster.special_attack()
            } else {
                monster.attack(self.player.defense)
            };

            self.player.receive_damage(monster_damage);
        }

        // Check combat end conditions
        if !monster.is_alive() {
            self.end_combat(monster_idx);
        } else if self.player.points_de_vie <= 0 {
            self.state = GameState::GameOver;
            println!("Vous avez été vaincu!");
        }
    }

    fn end_combat(&mut self, monster_idx: usize) {
        println!("Victoire! +{} points", self.monsters[monster_idx].level * 10);
        self.score += self.monsters[monster_idx].level * 10;
        
        // Level up player
        self.player.level_up();
        
        // Remove monster from map and list
        let monster = self.monsters.remove(monster_idx);
        self.map.clear_position(monster.x, monster.y);
        
        self.state = GameState::Running;
        self.current_monster_index = None;
    }

    pub fn display(&self) {
        println!("Nom: {} (Niveau {})", self.player.name, self.player.level);
        println!("Points de vie: {}/{}", self.player.points_de_vie, self.player.max_health);
        println!("Potions: {}", self.player.potions);
        println!("Score: {}", self.score);
        
        if let Some(monster_idx) = self.current_monster_index {
            let monster = &self.monsters[monster_idx];
            println!("\nCombat contre {} (Niveau {})", 
                match monster.species {
                    MonsterSpecies::Goblin => "Gobelin",
                    MonsterSpecies::Orc => "Orc",
                    MonsterSpecies::Skeleton => "Squelette",
                    MonsterSpecies::Dragon => "Dragon",
                },
                monster.level
            );
            println!("Points de vie du monstre: {}", monster.health);
        }
        
        println!("\nCarte:");
        self.map.display();
    }

    pub fn show_inventory(&mut self) {
        println!("Inventaire de {}", self.player.name);
        println!("Points de vie: {}/{}", self.player.points_de_vie, self.player.max_health);
        println!("Potions: {}", self.player.potions);
        println!("Espèce: {}", match self.player.espece {
            Espece::Nain => "Nain",
            Espece::Sorciere => "Sorcière",
            Espece::Elfe => "Elfe",
            Espece::Chevalier => "Chevalier",
        });
        println!("Niveau d'attaque: {}", self.player.attack);
        println!("Défense: {}", self.player.defense);
    }

    pub fn show_help(&mut self) {
        println!("En mode normal:");
        println!("z/flèche haut: Monter");
        println!("s/flèche bas: Descendre");
        println!("q/flèche gauche: Aller à gauche");
        println!("d/flèche droite: Aller à droite");
        println!("i: Afficher l'inventaire");
        println!("h: Afficher l'aide");
        println!("x: Quitter le jeu");
        
        println!("\nEn mode combat:");
        println!("a: Attaque simple");
        println!("s: Attaque spéciale");
        println!("p: Boire une potion");
    }
}