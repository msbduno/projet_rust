use rand::Rng;
use crate::monster::Monster;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Espece {
    Nain,
    Sorciere,
    Elfe,
    Chevalier,
}

pub struct Player {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub points_de_vie: i32,
    pub max_health: i32,
    pub level: u32,
    pub attack: i32,
    pub defense: i32,
    pub espece: Espece,
    pub attaque_speciale: bool,
    pub potions: i32,
}

impl Default for Player {
    fn default() -> Self {
        Player::new("Aventurier")
    }
}

impl Player {


    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();
        let espece = match rng.gen_range(0..4) {
            0 => Espece::Nain,
            1 => Espece::Sorciere,
            2 => Espece::Elfe,
            _ => Espece::Chevalier,
        };

        let (base_health, base_attack, base_defense) = match espece {
            Espece::Nain => (120, 15, 10),
            Espece::Sorciere => (80, 12, 5),
            Espece::Elfe => (100, 18, 7),
            Espece::Chevalier => (150, 16, 12),
        };

        Player {
            name: name.to_string(),
            x: 0,
            y: 0,
            points_de_vie: base_health,
            max_health: base_health,
            level: 1,
            attack: base_attack,
            defense: base_defense,
            espece,
            attaque_speciale: true,
            potions: 3,
        }
    }

    pub fn attack(&mut self, monster: &mut Monster) -> i32 {
        let mut rng = rand::thread_rng();
        let base_damage = std::cmp::max(1, self.attack - monster.defense);
        let critical_chance: i32 = rng.gen_range(0..10);
        
        let damage = if critical_chance == 0 {
            println!("Coup critique de {}!", self.name);
            base_damage * 2
        } else {
            base_damage
        };

        println!("{} attaque et inflige {} dégâts!", self.name, damage);
        damage
    }

    pub fn use_special_attack(&mut self, monster: &mut Monster) -> i32 {
        if !self.attaque_speciale {
            println!("Attaque spéciale non disponible!");
            return self.attack;
        }

        self.attaque_speciale = false;
        match self.espece {
            Espece::Nain => {
                println!("{} (nain) effectue une attaque spéciale!", self.name);
                monster.receive_damage(self.attack * 2);
                self.attack * 2
            },
            Espece::Sorciere => {
                println!("{} (sorcière) effectue une attaque spéciale!", self.name);
                let damage = self.attack;
                self.points_de_vie = std::cmp::min(self.points_de_vie + 20, self.max_health);
                damage
            },
            Espece::Elfe => {
                println!("{} (elfe) effectue une attaque spéciale!", self.name);
                self.attack *= 2;
                self.attack
            },
            Espece::Chevalier => {
                println!("{} (chevalier) effectue une attaque spéciale!", self.name);
                self.points_de_vie = std::cmp::min(self.points_de_vie + 10, self.max_health);
                self.attack * 2
            },
        }
    }

    pub fn receive_damage(&mut self, damage: i32) {
        self.points_de_vie = std::cmp::max(0, self.points_de_vie - damage);
        println!("{} reçoit {} points de dégâts!", self.name, damage);
    }

    pub fn drink_potion(&mut self) {
        if self.potions > 0 {
            self.points_de_vie = std::cmp::min(self.points_de_vie + 30, self.max_health);
            self.potions -= 1;
            println!("{} boit une potion et récupère 30 points de vie!", self.name);
        } else {
            println!("{} n'a plus de potions!", self.name);
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 20;
        self.points_de_vie = self.max_health;
        self.attack += 5;
        self.defense += 3;
        self.attaque_speciale = true;
        self.potions += 1;
        println!("{} monte au niveau {} !", self.name, self.level);
    }
}