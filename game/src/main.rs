use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod game;
mod map;
mod player;
mod monster;
mod combat;

use game::{Game, GameState};


fn main() {
    let game = Arc::new(Mutex::new(Game::new()));
    let game_clone = Arc::clone(&game);

    // Thread pour les événements aléatoires (spawn de monstres)
    thread::spawn(move || {
        let mut _rng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_secs(5));
            let mut game = game_clone.lock().unwrap();
            if game.state == GameState::Running {
                game.spawn_random_monster();
            }
        }
    });

    let mut input = String::new();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);//effacer l'ecran
    println!("Bienvenue dans le Mini-RPG!");
    println!("Entrez votre nom:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    {
        let mut game = game.lock().unwrap();
        game.initialize_player(&name);
    }

    loop {
        {
            let  game = game.lock().unwrap();
            game.display();

            if game.state == GameState::Combat {
                continue;
            }

            if game.state == GameState::GameOver {
                println!("Game Over! Score final: {}", game.score);
                break;
            }

            println!("\nCommandes: (z)haut (s)bas (q)gauche (d)droite (i)inventaire (h)aide (x)quitter");
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();

        let mut game = game.lock().unwrap();
        match command.as_str() {
            "z" => game.move_player(0, -1),
            "s" => game.move_player(0, 1),
            "q" => game.move_player(-1, 0),
            "d" => game.move_player(1, 0),
            "i" => game.show_inventory(),
            "h" => game.show_help(),
            "x" => break,
            _ => println!("Commande invalide!"),
        }
    }
}