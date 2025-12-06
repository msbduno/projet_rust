# projet_rust

Un mini-RPG développé en Rust avec gestion de la concurrence et système de combat.



https://github.com/user-attachments/assets/bd4da190-bd98-468c-9d53-a38b974fcc01



## Table des matières

- [À propos](#à-propos)
- [Structure du projet](#structure-du-projet)
- [Fonctionnalités](#fonctionnalités)
- [Architecture technique](#architecture-technique)
- [Installation](#installation)
- [Utilisation](#utilisation)

## À propos

Ce projet est un mini-RPG développé en Rust qui met en œuvre des concepts avancés de programmation système, notamment la gestion de la concurrence avec des threads et des mutex.

## Structure du projet

```
src/
├── main.rs      # Point d'entrée et logique principale
├── game.rs      # Gestion de l'état global du jeu
├── map.rs       # Gestion de la carte et des tuiles
├── player.rs    # Définition et comportement du joueur
└── monster.rs   # Définition et comportement des monstres
```

## Fonctionnalités

- **Système de combat** avec différentes espèces de personnages
- **Génération automatique de monstres** toutes les 5 secondes
- **Carte interactive** avec exploration
- **Gestion d'états** (Running, Combat, GameOver, Win)
- **Espèces jouables** : Homme, Sorcière, Elfe, Chevalier

## Architecture technique

### Gestion de la concurrence

Le jeu utilise un modèle de concurrence thread-safe avec :

- **`Arc<Mutex<Game>>`** pour le partage sécurisé de l'état du jeu entre threads
- **Thread secondaire** dédié au spawn de monstres

```rust
let game = Arc::new(Mutex::new(Game::new()));

thread::spawn(move || {
    loop {
        thread::sleep(Duration::from_secs(5));
        let mut game = game_clone.lock().unwrap();
        if game.state == GameState::Running {
            game.spawn_random_monster();
        }
    }
});
```

**Justification des choix :**
- `Arc` (Atomic Reference Counting) permet le partage de la référence entre threads
- `Mutex` garantit l'accès exclusif et thread-safe à l'état du jeu

### Gestion des erreurs

Le projet utilise les types Rust natifs pour une gestion robuste :

- `Option<usize>` pour `current_monster_index`
- `Result` dans `main()` pour la gestion des erreurs de terminal

### Patterns de conception

Le projet implémente plusieurs patterns :

1. **Factory Method** : Création de personnages et monstres
2. **State Pattern** : Gestion des états via l'énumération `GameState`
3. **Strategy Pattern** : Comportements différents selon l'espèce

### Énumérations

```rust
#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Running,
    Combat,
    GameOver,
    Win,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Espece {
    Homme,
    Sorciere,
    Elfe,
    Chevalier,
}
```

## Installation

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021 ou supérieure)
- Cargo (inclus avec Rust)

### Commandes

```bash
# Cloner le repository
git clone https://github.com/votre-username/mini-rpg-rust.git
cd mini-rpg-rust

# Compiler le projet
cargo build --release

# Lancer le jeu
cargo run --release
```

## Utilisation

Une fois le jeu lancé, suivez les instructions à l'écran pour :

1. Choisir votre espèce de personnage
2. Explorer la carte
3. Combattre les monstres
4. Survivre et gagner !


