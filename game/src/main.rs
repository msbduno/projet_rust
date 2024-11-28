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

fn select_character() -> (String, Espece) {
   
    let mut input = String::new();

    // Get player name
    println!("Bienvenue dans le Mini-RPG!");
    println!("Entrez votre nom:");
    std::io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    // Clear terminal for character selection
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    // Display character selection menu
    // add smybol for each character




    println!("Choisissez votre perso:");
    println!("1. Homme 🧑");
    println!("   Force brute et résistance exceptionnelle");
    println!("   PV: 120, Attaque: 15, Défense: 10");
    println!();
    
    println!("2. Sorcière 🧙");
    println!("   Maîtrise de la magie et capacité de soin");
    println!("   PV: 80, Attaque: 12, Défense: 5");
    println!();
    
    println!("3. Elfe 🧚");
    println!("   Agilité et puissance d'attaque supérieure");
    println!("   PV: 100, Attaque: 18, Défense: 7");
    println!();
    
    println!("4. Chevalier 🧝");
    println!("   Equilibre entre attaque et défense");
    println!("   PV: 150, Attaque: 16, Défense: 12");
    
    // Get character choice
    loop {
        input.clear();
        println!("\nEntrez votre choix (1-4):");
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => return (name, Espece::Homme),
            "2" => return (name, Espece::Sorciere),
            "3" => return (name, Espece::Elfe),
            "4" => return (name, Espece::Chevalier),
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
}

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

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    // Get player name and character choice
    let (name, espece) = select_character();

    {
        let mut game = game.lock().unwrap();
        game.initialize_player(&name, espece);
    }

    // Activation du mode brut pour les déplacements
    terminal::enable_raw_mode()?;
    loop {
        let mut stdout = std::io::stdout();
        stdout.execute(terminal::Clear(ClearType::All))?;
        {
            let game = game.lock().unwrap();
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            game.display();   
            
            
            if game.state == GameState::Win {
                //clear screen
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!();
                println!("🏆 FÉLICITATIONS ! Vous avez atteint le niveau 5 et remporté le jeu avce le score {}!", game.score);
                
            }

            if game.state == GameState::GameOver {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!();
                println!("Game Over! Score final: {}, Niveau atteint : {}", game.score , game.player.level);
                break;
            }

         

            match game.state {
                GameState::Running => {
                    println!("\nCommandes: (z)haut (s)bas (q)gauche (d)droite (i)inventaire (h)aide (x)quitter");
                },
                GameState::Combat => {
                    println!("\nCommandes de combat: (a)ttaque (s)péciale (p)otion (x)quitter");
                },
                GameState::GameOver => break,
                GameState::Win => break,
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
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        },
                        KeyCode::Char('h') => {
                            game.show_help();
                            std::thread::sleep(std::time::Duration::from_secs(2));
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
                        KeyCode::Char('h') => {
                            game.show_help();
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        },
                        KeyCode::Char('x') => break,
                        _ => {}
                    }
                },


                GameState::GameOver => break,
                GameState::Win => break,
            }
        }
    }

    // Désactivation du mode brut après le jeu
    terminal::disable_raw_mode()?;
    Ok(())
}