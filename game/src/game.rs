use crate::map::Map;
use crate::player::{Player, Espece};
use crate::monster::{Monster, MonsterSpecies};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)] 
pub enum GameState {
    Running,
    Combat,
    GameOver,
    Win,
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

    pub fn initialize_player(&mut self, name: &str, espece: Espece) {
        self.player = Player::new_with_class(name, espece);
        self.map.place_player(0, 0, &self.player);
    }

    pub fn spawn_random_monster(&mut self) {
        let mut _rng = rand::thread_rng();
        if self.monsters.len() < 10 {  // Limit number of monsters
            let (x, y) = self.map.get_random_empty_position();
            let  monster = Monster::new(x, y);
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
            self.map.place_player(new_x, new_y, &self.player);
    
            // Check for special icons
            if self.map.is_health_icon(new_x, new_y) {
                self.player.points_de_vie = std::cmp::min(
                    self.player.points_de_vie + 10, 
                    self.player.max_health
                );
                self.map.clear_special_icon(new_x, new_y);
                print!("❤️ Vous récupérez 10 points de vie!\r\n");
            }
    
            if self.map.is_damage_icon(new_x, new_y) {
                self.player.points_de_vie -= 50;
                self.map.clear_special_icon(new_x, new_y);
                print!("🔥 Vous subissez 50 points de dégâts!\r\n");
    
                // Check if player dies
                if self.player.points_de_vie <= 0 {
                    self.state = GameState::GameOver;
                }
            }
    
            if let Some(monster_idx) = self.find_monster_at(new_x, new_y) {
                self.start_combat(monster_idx);
            }
        }
    
        // Existing door check remains the same
        if self.map.is_on_door(self.player.x, self.player.y) {
            self.generate_new_map();
        }
    }

    pub fn start_combat(&mut self, monster_idx: usize) {
        self.state = GameState::Combat;
        self.current_monster_index = Some(monster_idx);
        
        // Effacer l'écran
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
        let monster_name = match self.monsters[monster_idx].species {
            MonsterSpecies::Goblin => "Gobelin",
            MonsterSpecies::Orc => "Orc",
            MonsterSpecies::Skeleton => "Squelette",
            MonsterSpecies::Dragon => "Dragon",
        };
        
        // Afficher l'introduction du combat avec une pause
        print!("\r\n⚔️  Un {} niveau {} vous attaque!\r\n", 
            monster_name,
            self.monsters[monster_idx].level
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        // Afficher les statistiques initiales
        print!("\r\n=== DÉBUT DU COMBAT ===\r\n");
        std::thread::sleep(std::time::Duration::from_millis(500));
        let mut icon = ' ';
        if self.player.espece == Espece::Homme {
            icon = '🧑';
        } else if self.player.espece  == Espece::Sorciere {
            icon = '🧙';
        } else if self.player.espece  == Espece::Elfe {
            icon  = '🧚';
        } else if self.player.espece  == Espece::Chevalier {
            icon = '🧝';
        }
        print!("\r\n {} {}\r\n", icon, self.player.name);
        print!("❤️  Points de vie: {}/{}\r\n", self.player.points_de_vie, self.player.max_health);
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        print!("\r\n👾 {}\r\n", monster_name);
        print!("❤️  Points de vie: {}/{}\r\n", 
            self.monsters[monster_idx].health,
            self.monsters[monster_idx].max_health
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        print!("\r\nPréparez-vous au combat!\r\n");
        
        // Pause finale pour s'assurer que tout est lisible
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn combat_turn(&mut self, player_action: PlayerCombatAction) {
        if self.state != GameState::Combat || self.current_monster_index.is_none() {
            return;
        }
    
        // Clear screen at the start of each combat turn
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
        let monster_idx = self.current_monster_index.unwrap();
        let monster = &mut self.monsters[monster_idx];
    
        // Fonction helper pour faire une pause
        fn combat_pause() {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

    
        // Player's turn
        print!("\r\n🗡️  Tour de {} !\r\n", self.player.name);
        combat_pause();
    
        match player_action {
            PlayerCombatAction::Attack => {
                let damage = self.player.attack(monster);
                print!("➜ {} prépare son attaque...\r\n", self.player.name);
                combat_pause();
                
                monster.receive_damage(damage);
                print!("➜ {} frappe et inflige {} points de dégâts au {} !\r\n", 
                    self.player.name, 
                    damage,
                    match monster.species {
                        MonsterSpecies::Goblin => "Gobelin",
                        MonsterSpecies::Orc => "Orc",
                        MonsterSpecies::Skeleton => "Squelette",
                        MonsterSpecies::Dragon => "Dragon",
                    }
                );
                combat_pause();
                
                print!("PV restants du monstre: {}\r\n", monster.health);
                combat_pause();
            },
            PlayerCombatAction::SpecialAttack => {
                if self.player.attaque_speciale {
                    print!("➜ {} prépare une attaque spéciale...\r\n", self.player.name);
                    combat_pause();
                    
                    let damage = self.player.use_special_attack(monster);
                    print!("➜ {} déchaîne sa puissance et inflige {} points de dégâts !\r\n", 
                        self.player.name, 
                        damage
                    );
                    combat_pause();
                    
                    print!("PV restants du monstre: {}\r\n", monster.health);
                    combat_pause();
                } else {
                    print!("❌ Attaque spéciale non disponible!\r\n");
                    combat_pause();
                    return;
                }
            },
            PlayerCombatAction::Drink => {
                let old_hp = self.player.points_de_vie;
                print!("➜ {} sort une potion...\r\n", self.player.name);
                combat_pause();
                
                self.player.drink_potion();
                if self.player.points_de_vie > old_hp {
                    print!("➜ {} boit la potion et récupère {} points de vie!\r\n", 
                        self.player.name,
                        self.player.points_de_vie - old_hp
                    );
                    combat_pause();
                    
                    print!("Nouveaux PV: {}\r\n", self.player.points_de_vie);
                    combat_pause();
                }
            }
        }
    
        // Monster's turn if still alive
        if monster.is_alive() {
            print!("\r\n👾 Tour du monstre:\r\n");
            combat_pause();
            
            let mut rng = rand::thread_rng();
            let monster_action: i32 = rng.gen_range(0..10);
    
            let monster_name = match monster.species {
                MonsterSpecies::Goblin => "Gobelin",
                MonsterSpecies::Orc => "Orc",
                MonsterSpecies::Skeleton => "Squelette",
                MonsterSpecies::Dragon => "Dragon",
            };
    
            print!("➜ Le {} se prépare à attaquer...\r\n", monster_name);
            combat_pause();
    
            let monster_damage = if monster_action < 2 && monster.special_attack_available {
                monster.special_attack()
            } else {
                let damage = monster.attack(self.player.defense);
                print!("➜ Le {} attaque et inflige {} points de dégâts!\r\n", 
                    monster_name,
                    damage
                );
                damage
            };
            combat_pause();
    
            self.player.receive_damage(monster_damage);
            print!("PV restants de {}: {}\r\n", self.player.name, self.player.points_de_vie);
            combat_pause();
        }
    
        // Check combat end conditions
        if !monster.is_alive() {
            print!("\r\n💫 Victoire!\r\n");
            combat_pause();
            print!("➜ +{} points d'expérience\r\n", monster.level * 10);
            combat_pause();
            self.end_combat(monster_idx);
        } else if self.player.points_de_vie <= 0 {
            print!("\r\n💀 Vous avez été vaincu!\r\n");
            combat_pause();
            self.state = GameState::GameOver;
        }
    
        // Final pause before next turn
        combat_pause();
    }

    fn end_combat(&mut self, monster_idx: usize) {
        print!("Victoire! +{} points\r\n", self.monsters[monster_idx].level * 10);
        self.score += self.monsters[monster_idx].level * 10;
        
        
        // Remove monster from map and list
        let monster = self.monsters.remove(monster_idx);
        self.map.clear_position(monster.x, monster.y);
        
        self.state = GameState::Running;
        self.current_monster_index = None;
    }

    pub fn display(&self) {
        print!("Joueur: {} (Niveau {})\r\n", self.player.name, self.player.level);
        print!("Score: {}\r\n", self.score);
        
        if let Some(monster_idx) = self.current_monster_index {
            let monster = &self.monsters[monster_idx];
            print!("\r\nCombat contre {} (Niveau {})\r\n", 
                match monster.species {
                    MonsterSpecies::Goblin => "Gobelin",
                    MonsterSpecies::Orc => "Orc",
                    MonsterSpecies::Skeleton => "Squelette",
                    MonsterSpecies::Dragon => "Dragon",
                },
                monster.level
            );
            print!("\r\n");
            print!("Monstre -> Points de vie {}/{}\r\n", monster.health, monster.max_health);
            print!("{} -> Points de vie {}/{}\r\n", self.player.name, self.player.points_de_vie, self.player.max_health);
            print!("\r\n");
            
        }
        
    
        self.map.display();
    }

    pub fn show_inventory(&mut self) {
        print!("Inventaire de {}\r\n", self.player.name);
        print!("Points de vie: {}/{}\r\n", self.player.points_de_vie, self.player.max_health);
        print!("Potions: {}\r\n", self.player.potions);
        print!("Espèce: {}\r\n", match self.player.espece {
            Espece::Homme => "Hommme",
            Espece::Sorciere => "Sorcière",
            Espece::Elfe => "Elfe",
            Espece::Chevalier => "Chevalier",
        });
        print!("Attaque: {}\r\n", self.player.attack);
        print!("Défense: {}\r\n", self.player.defense);
    }

    pub fn show_help(&mut self) {
        print!("En mode normal:\r\n");
        print!("z/flèche haut: Monter\r\n");
        print!("s/flèche bas: Descendre\r\n");
        print!("q/flèche gauche: Aller à gauche\r\n");
        print!("d/flèche droite: Aller à droite\r\n");
        print!("i: Afficher l'inventaire\r\n");
        print!("h: Afficher l'aide\r\n");
        print!("x: Quitter le jeu\r\n");
        
        print!("\r\nEn mode combat:\r\n");
        print!("a: Attaque simple\r\n");
        print!("s: Attaque spéciale\r\n");
        print!("p: Boire une potion\r\n");
    }
    // generate a new map if the player is on a door tile 


    pub fn generate_new_map(&mut self) {
        self.player.level_up();
        
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        // Display level up message with benefits
        print!("\r\n🆙 PASSAGE AU NIVEAU {} 🆙\r\n", self.player.level);
        print!(" \r\n");
        print!("• Points de vie max augmentés\r\n");
        print!("• Attaque améliorée\r\n");
        print!("• Défense renforcée\r\n");
        print!("• Attaque spéciale réinitialisée\r\n");
        print!("• Une nouvelle potion ajoutée\r\n");
        
        // Pause to let the player read the message
        std::thread::sleep(std::time::Duration::from_secs(3));
    
        // Check if the game is won
        if self.player.level == 5 {
            self.state = GameState::Win;
        } else {
            // Generate a new map with the same size
            self.map = Map::new(10, 10);
        
            // Place the player at the starting position
            self.map.place_player(0, 0, &self.player);
            self.player.x = 0;
            self.player.y = 0;
        
            // Reset monsters
            self.monsters.clear();
            self.spawn_random_monster();
        }
    }

    
}