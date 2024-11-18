use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};

mod game;
mod map;
mod player;
mod monster;

use game::{Game, GameState, PlayerCombatAction};
use player::Espece;

fn main() -> crossterm::Result<()> {
    let game = Arc::new(Mutex::new(Game::new()));
    let game_clone = Arc::clone(&game);

    // Thread pour les événements aléatoires (spawn de monstres)
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let mut game = game_clone.lock().unwrap();
            if game.state == GameState::Running {
                game.spawn_random_monster();
            }
        }
    });

    let mut stdout = std::io::stdout();
    let mut input = String::new();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    println!("Bienvenue dans le Mini-RPG!");
    println!("Entrez votre nom:");
    std::io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    {
        let mut game = game.lock().unwrap();
        game.initialize_player(&name);
    }

    // Activation du mode brut pour les déplacements
    terminal::enable_raw_mode()?;
    loop {
        stdout.execute(terminal::Clear(ClearType::All))?;
        {
            let game = game.lock().unwrap();
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            game.display();

            if game.state == GameState::GameOver {
                println!("Game Over! Score final: {}", game.score);
                break;
            }

            match game.state {
                GameState::Running => {
                    println!("\nCommandes: (z)haut (s)bas (q)gauche (d)droite (i)inventaire (h)aide (x)quitter");
                },
                GameState::Combat => {
                    println!("\nCommandes de combat: (a)ttaque (s)péciale (p)otion");
                },
                GameState::GameOver => break,
            }
        }
       
        if let Event::Key(key_event) = event::read()? {
            let mut game = game.lock().unwrap();
            match game.state {
                GameState::Running => {
                                match key_event.code {
                                    KeyCode::Up | KeyCode::Char('z') => game.move_player(0, -1),
                                    KeyCode::Down | KeyCode::Char('s') => game.move_player(0, 1),
                                    KeyCode::Left | KeyCode::Char('q') => game.move_player(-1, 0),
                                    KeyCode::Right | KeyCode::Char('d') => game.move_player(1, 0),
                                    KeyCode::Char('i') => {
                                        game.show_inventory();
                                        std::thread::sleep(std::time::Duration::from_secs(2)); // Ajoutez un délai
                                    },
                                    KeyCode::Char('h') => {
                                        game.show_help();
                                        std::thread::sleep(std::time::Duration::from_secs(2)); // Ajoutez un délai
                                    },
                                    KeyCode::Char('x') => break,
                                    _ => {}
                    }
                },
                GameState::Combat => {
                    match key_event.code {
                        KeyCode::Char('a') => game.combat_turn(PlayerCombatAction::Attack),
                        KeyCode::Char('s') => game.combat_turn(PlayerCombatAction::SpecialAttack),
                        KeyCode::Char('p') => game.combat_turn(PlayerCombatAction::Drink),
                        _ => {}
                    }
                },
                GameState::GameOver => break,
            }
        }
    }

    // Désactivation du mode brut après le jeu
    terminal::disable_raw_mode()?;
    Ok(())
}