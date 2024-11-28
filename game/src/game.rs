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
                println!("❤️ Vous récupérez 10 points de vie!");
            }
    
            if self.map.is_damage_icon(new_x, new_y) {
                self.player.points_de_vie -= 50;
                self.map.clear_special_icon(new_x, new_y);
                println!("🔥 Vous subissez 50 points de dégâts!");
    
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
        println!("\n⚔️  Un {} niveau {} vous attaque!", 
            monster_name,
            self.monsters[monster_idx].level
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        // Afficher les statistiques initiales
        println!("\n=== DÉBUT DU COMBAT ===");
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
        println!("\n {} {}", icon, self.player.name);
        println!("❤️  Points de vie: {}/{}", self.player.points_de_vie, self.player.max_health);
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        println!("\n👾 {}", monster_name);
        println!("❤️  Points de vie: {}/{}", 
            self.monsters[monster_idx].health,
            self.monsters[monster_idx].max_health
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        println!("\nPréparez-vous au combat!");
        
        // Pause finale pour s'assurer que tout est lisible
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    pub fn combat_turn(&mut self, player_action: PlayerCombatAction) {
        if self.state != GameState::Combat || self.current_monster_index.is_none() {
            return;
        }
    
        let monster_idx = self.current_monster_index.unwrap();
        let monster = &mut self.monsters[monster_idx];
    
        // Fonction helper pour faire une pause
        fn combat_pause() {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

    
        // Player's turn
        println!("\n🗡️  Tour de {} !", self.player.name);
        combat_pause();
    
        match player_action {
            PlayerCombatAction::Attack => {
                let damage = self.player.attack(monster);
                println!("➜ {} prépare son attaque...", self.player.name);
                combat_pause();
                
                monster.receive_damage(damage);
                println!("➜ {} frappe et inflige {} points de dégâts au {} !", 
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
                
                println!("PV restants du monstre: {}", monster.health);
                combat_pause();
            },
            PlayerCombatAction::SpecialAttack => {
                if self.player.attaque_speciale {
                    println!("➜ {} prépare une attaque spéciale...", self.player.name);
                    combat_pause();
                    
                    let damage = self.player.use_special_attack(monster);
                    println!("➜ {} déchaîne sa puissance et inflige {} points de dégâts !", 
                        self.player.name, 
                        damage
                    );
                    combat_pause();
                    
                    println!("PV restants du monstre: {}", monster.health);
                    combat_pause();
                } else {
                    println!("❌ Attaque spéciale non disponible!");
                    combat_pause();
                    return;
                }
            },
            PlayerCombatAction::Drink => {
                let old_hp = self.player.points_de_vie;
                println!("➜ {} sort une potion...", self.player.name);
                combat_pause();
                
                self.player.drink_potion();
                if self.player.points_de_vie > old_hp {
                    println!("➜ {} boit la potion et récupère {} points de vie!", 
                        self.player.name,
                        self.player.points_de_vie - old_hp
                    );
                    combat_pause();
                    
                    println!("Nouveaux PV: {}", self.player.points_de_vie);
                    combat_pause();
                }
            }
        }
    
        // Monster's turn if still alive
        if monster.is_alive() {
            println!("\n👾 Tour du monstre:");
            combat_pause();
            
            let mut rng = rand::thread_rng();
            let monster_action: i32 = rng.gen_range(0..10);
    
            let monster_name = match monster.species {
                MonsterSpecies::Goblin => "Gobelin",
                MonsterSpecies::Orc => "Orc",
                MonsterSpecies::Skeleton => "Squelette",
                MonsterSpecies::Dragon => "Dragon",
            };
    
            println!("➜ Le {} se prépare à attaquer...", monster_name);
            combat_pause();
    
            let monster_damage = if monster_action < 2 && monster.special_attack_available {
                monster.special_attack()
            } else {
                let damage = monster.attack(self.player.defense);
                println!("➜ Le {} attaque et inflige {} points de dégâts!", 
                    monster_name,
                    damage
                );
                damage
            };
            combat_pause();
    
            self.player.receive_damage(monster_damage);
            println!("PV restants de {}: {}", self.player.name, self.player.points_de_vie);
            combat_pause();
        }
    
        // Check combat end conditions
        if !monster.is_alive() {
            println!("\n💫 Victoire!");
            combat_pause();
            println!("➜ +{} points d'expérience", monster.level * 10);
            combat_pause();
            self.end_combat(monster_idx);
        } else if self.player.points_de_vie <= 0 {
            println!("\n💀 Vous avez été vaincu!");
            combat_pause();
            self.state = GameState::GameOver;
        }
    
        // Final pause before next turn
        combat_pause();
    }

    fn end_combat(&mut self, monster_idx: usize) {
        println!("Victoire! +{} points", self.monsters[monster_idx].level * 10);
        self.score += self.monsters[monster_idx].level * 10;
        
        
        // Remove monster from map and list
        let monster = self.monsters.remove(monster_idx);
        self.map.clear_position(monster.x, monster.y);
        
        self.state = GameState::Running;
        self.current_monster_index = None;
    }

    pub fn display(&self) {
        println!("Joueur: {} (Niveau {})", self.player.name, self.player.level);
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
            println!();
            println!("Monstre -> Points de vie {}/{}", monster.health, monster.max_health);
            println!("{} -> Points de vie {}/{}", self.player.name, self.player.points_de_vie, self.player.max_health);
            println!();
            
        }
        
    
        self.map.display();
    }

    pub fn show_inventory(&mut self) {
        println!("Inventaire de {}", self.player.name);
        println!("Points de vie: {}/{}", self.player.points_de_vie, self.player.max_health);
        println!("Potions: {}", self.player.potions);
        println!("Espèce: {}", match self.player.espece {
            Espece::Homme => "Hommme",
            Espece::Sorciere => "Sorcière",
            Espece::Elfe => "Elfe",
            Espece::Chevalier => "Chevalier",
        });
        println!("Attaque: {}", self.player.attack);
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
    // generate a new map if the player is on a door tile 


    pub fn generate_new_map(&mut self) {
        self.player.level_up();
        
        // Clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
        // Display level up message with benefits
        println!("\n🆙 PASSAGE AU NIVEAU {} 🆙", self.player.level);
        println!(" ");
        println!("• Points de vie max augmentés");
        println!("• Attaque améliorée");
        println!("• Défense renforcée");
        println!("• Attaque spéciale réinitialisée");
        println!("• Une nouvelle potion ajoutée");
        
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