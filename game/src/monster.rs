use rand::Rng;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MonsterSpecies {
    Goblin,
    Orc,
    Skeleton,
    Dragon,
}

pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub species: MonsterSpecies,
    pub level: u32,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub special_attack_available: bool,
}

impl Monster {
    pub fn new(x: usize, y: usize) -> Self {
        let mut rng = rand::thread_rng();
        let level = rng.gen_range(1..=3);
        let species = match rng.gen_range(0..4) {
            0 => MonsterSpecies::Goblin,
            1 => MonsterSpecies::Orc,
            2 => MonsterSpecies::Skeleton,
            _ => MonsterSpecies::Dragon,
        };

        let (base_health, base_attack, base_defense) = match species {
            MonsterSpecies::Goblin => (50, 10, 5),
            MonsterSpecies::Orc => (80, 15, 8),
            MonsterSpecies::Skeleton => (40, 12, 3),
            MonsterSpecies::Dragon => (120, 20, 12),
        };

        Monster {
            x,
            y,
            species,
            level,
            health: base_health * level as i32,
            max_health: base_health * level as i32,
            attack: base_attack * level as i32,
            defense: base_defense * level as i32,
            special_attack_available: true,
        }
    }

    pub fn attack(&mut self, target_defense: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let damage = std::cmp::max(1, self.attack - target_defense);
        let critical_chance: i32 = rng.gen_range(0..10);
        
        if critical_chance == 0 {
            println!("Coup critique!");
            damage * 2
        } else {
            damage
        }
    }

    pub fn special_attack(&mut self) -> i32 {
        if !self.special_attack_available {
            return self.attack;
        }

        self.special_attack_available = false;
        match self.species {
            MonsterSpecies::Goblin => {
                println!("Le Gobelin effectue une attaque fourbe!");
                self.attack * 2
            },
            MonsterSpecies::Orc => {
                println!("L'Orc pousse un cri de guerre!");
                self.attack * 3 / 2
            },
            MonsterSpecies::Skeleton => {
                println!("Le Squelette lance une attaque spectrale!");
                self.attack * 2
            },
            MonsterSpecies::Dragon => {
                println!("Le Dragon crache des flammes!");
                self.attack * 3
            },
        }
    }

    pub fn receive_damage(&mut self, damage: i32) {
        self.health = std::cmp::max(0, self.health - damage);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}