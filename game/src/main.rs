use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};

mod game;
mod map;
mod player;
mod monster;
mod combat;

use game::{Game, GameState};

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

    // Saisie classique pour le prénom
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

            println!("\nCommandes: (z)haut (s)bas (q)gauche (d)droite (i)inventaire (h)aide (x)quitter");
        }

       
            if let Event::Key(key_event) = event::read()? {
                let mut game = game.lock().unwrap();
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('z') => game.move_player(0, -1),
                    KeyCode::Down | KeyCode::Char('s') => game.move_player(0, 1),
                    KeyCode::Left | KeyCode::Char('q') => game.move_player(-1, 0),
                    KeyCode::Right | KeyCode::Char('d') => game.move_player(1, 0),
                    KeyCode::Char('i') => game.show_inventory(),
                    KeyCode::Char('h') => game.show_help(),
                    KeyCode::Char('x') => break,
                    _ => {}
                }
            }
        
    }

    // Désactivation du mode brut après le jeu
    terminal::disable_raw_mode()?;
    Ok(())
}
