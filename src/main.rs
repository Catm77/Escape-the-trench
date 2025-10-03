use std::io;
use std::thread;
use std::time::Duration;
use std::fs;
use rand::Rng;
use serde_json;

use crate::encounters::get_death_message_per_stage;

mod encounters;
mod game_data;
mod combat;

//Constant data that I don't want changed
const WIN_STAGE:u32 = 10;
const SAVE_FILE_PATH: &str = "data/stats.json";
const ENEMY_SCALING_FACTOR: f32 = 1.15;
//Saving system and stuff uh is pain


fn load_game_data() -> Result<game_data::GameData, io::Error> 
{
    match fs::read_to_string(SAVE_FILE_PATH)
    {
        Ok(json_data) =>
        {
            let game_data: game_data::GameData = serde_json::from_str(&json_data)?;
            Ok(game_data)
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => 
        {
            Err(io::Error::new(io::ErrorKind::NotFound, "Save file not found. Please start a new game to create one."))
        },
        Err(e) => Err(e),
    }
}

fn save_game_data(game_data: &game_data::GameData) -> Result<(), io::Error> 
{
    let json_data = serde_json::to_string_pretty(game_data)?;
    fs::write(SAVE_FILE_PATH, json_data)?;
    Ok(())
}

fn scale_enemy_stats(mut enemy: game_data::Enemy, stage: i32) -> game_data::Enemy 
{
    if stage <= 1 
    {
        return enemy;
    }

    
    let multiplier: f32 = ENEMY_SCALING_FACTOR.powi(stage - 1);

    
    enemy.stats.max_health = ((enemy.stats.max_health as f32) * multiplier).round() as i32;
    enemy.stats.health = enemy.stats.max_health; 

    enemy.stats.attack = ((enemy.stats.attack as f32) * multiplier).round() as i32;
    enemy.stats.defense = ((enemy.stats.defense as f32) * multiplier).round() as i32;

    enemy
}

fn display_ending(player: &game_data::Player)
{
    if player.stats.health <= 0 
    {
        println!();
        let death_art = r#"
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░▓████████████████████████▒░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░▓█████▓▒░░░░░░░░░░░░░░░▒██████▒░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░████▒░░░░░░░░░░░░░░░░░░░░░░░░░▓███▒░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░███░░░░░░░░░░░░░░░
░░░░░░░░░░░░░▒██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒██░░░░░░░░░░░░░░
░░░░░░░░░░░░▒██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░░░░░░░░░░░░
░░░░░░░░░░░░██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░░░░░░░░░░░
░░░░░░░░░░░██▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒░░██░░░░░░░░░░░░
░░░░░░░░░░░██░░██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░██░░░░░░░░░░░
░░░░░░░░░░░██░░██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░██░░░░░░░░░░░
░░░░░░░░░░░██░░██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░██░░░░░░░░░░░
░░░░░░░░░░░██▒░██▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██▓░▒██░░░░░░░░░░░
░░░░░░░░░░░░██░░██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██░░██░░░░░░░░░░░░
░░░░░░░░░░░░██▒░██░░░░░▒▒▓███▒░░░░░░░▒███▓▒▒░░░░░██░▓██░░░░░░░░░░░░
░░░░░░░░░░░░░██░██░░██████████▒░░░░░▓██████████░░██▒██░░░░░░░░░░░░░
░░░░░░░░░░░░░░████░████████████░░░░░████████████░████░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░███░▒██████████░░░░░░░██████████▒░██▒░░░░░░░░░▒░░░░░
░░░▒████░░░░░░░▓█▒░░█████████░░░░░░░░░█████████░░▒█▓░░░░░░▓████░░░░
░░░██░▒██▒░░░░░██░░░░██████▓░░░░█░█░░░░███████░░░░██░░░░░███░░██░░░
░░░██░░░██▓░░░░██░░░░░░▒▓▓░░░░▒██░██░░░░░▓▓▒░░░░░▒██░░░░███░░░██░░░
░▓██▒░░░░████▓░░██░░░░░░░░░░░░███░███░░░░░░░░░░░░██░░█████░░░░▓██▒░
██▓░░░░░░░░▒████████▓░░░░░░░░████░███▓░░░░░░░▒▓████████░░░░░░░░░███
██▓▒▓███▓░░░░░░▓████████▓░░░░████░███▓░░░░▓████████▓░░░░░░████▓▓███
░███████████▒░░░░░░███████░░░░██░░░██░░░░██████▓░░░░░░▓███████████░
░░░░░░░░░░▓█████░░░░██▓▓░██░░░░░░░░░░░░░██░█▒██░░░▒█████▓░░░░░░░░░░
░░░░░░░░░░░░░▒█████▒▒█▓█░███▓▓▒▒▒▓▒▒▓▓▓███▒███░▓█████░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░▒████▒▓█▒▒█░█▒█░█░█▓█▒█▓░█░█████▒░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░░░░██░░██▓█▓█▓█▒█▒█▓█▓████░▓█▓░░░░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░░░░░▓████▓░▓█▓█░█▒█░█░█▒█▒███▒░██████░░░░░░░░░░░░░░░░░
░░░░░░░░░░░░░▓█████░░██░░░▒█████▓█▓█████▒░░░██░▒█████▓░░░░░░░░░░░░░
░░░░▒██████████▓░░░░░███░░░░░░░░░░░░░░░░░░░██▒░░░░░▓██████████▒░░░░
░░░░██░░░▓▓▓░░░░░░▒██████▓░░░░░░░░░░░░░░░███████▒░░░░░░▓▓▒░░▒██░░░░
░░░░▓██░░░░░░░░▓████▓░░░█████▒░░░░░░▒▓█████░░░▓████▓░░░░░░░▒██▓░░░░
░░░░░░███░░░░████▒░░░░░░░░▓█████████████▒░░░░░░░░▒████░░░░███░░░░░░
░░░░░░░██░░░██▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓██░░░██░░░░░░░
░░░░░░░██▒▓██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒██▒▓██░░░░░░░
░░░░░░░░████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████░░░░░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░"#;
        for line in death_art.lines()
        {
            println!("{}", line);
            thread::sleep(Duration::from_millis(250));
        }
        let death_ending: String = get_death_message_per_stage(player.stage);
        println!();

        println!("{}", death_ending);

        thread::sleep(Duration::from_secs(10));
    }
    if player.stage == WIN_STAGE
    {
        println!();

        let win_message = encounters::get_win_message();

        println!("{}", win_message);
    }
}

fn game_loop(mut game_data: game_data::GameData)
{
    let mut player_is_alive = game_data.player.stats.health > 0;

    let mut heal_times = 3;

    if game_data.player.stage == 1 && game_data.player.stats.health == 100 
    {
        let encounter_message: String = encounters::get_random_entrance_message();
        for line in encounter_message.lines()
        {
            print!("{}", line);

            thread::sleep(Duration::from_millis(255));
        }
        thread::sleep(Duration::from_secs(5));
    }
    else
    {
        println!("[Loaded] Continuing from stage {}", game_data.player.stage);
    }

    loop 
    {
        if !player_is_alive || game_data.player.stage >= WIN_STAGE
        {
            display_ending(&game_data.player);

            thread::sleep(Duration::from_secs(5));

            break;
        }

        println!();

        println!("You decide to check on yourself.\n\
        [Status]\n\
        Stage: {}\n\
        Health: {}|{}\n\
        Attack: {}\n\
        Defense: {}",
         game_data.player.stage,
         game_data.player.stats.health, 
         game_data.player.stats.max_health,
         game_data.player.stats.attack,
         game_data.player.stats.defense);
        
        let art = r#"⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠿⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠀⠀⢀⣴⣿⣿⣿⣷⡄⠀⠀⣠⣾⣿⣿⣿⣦⠀⠀⠀⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠀⠀⢸⣿⣿⣿⣿⣿⡷⠀⠀⣿⣿⣿⣿⣿⣿⡇⠀⠀⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠀⠀⠈⠻⣿⣿⣿⡿⠃⠀⠀⠘⢿⣿⣿⣿⠟⠀⠀⠀⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣄⣀⠀⠀⠀⠉⠀⠀⣀⣤⣤⣀⠀⠈⠉⠀⠀⠀⣀⣠⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⣰⡟⠉⣤⣤⠉⢻⣆⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⣿⠀⣴⣮⣵⣦⠀⣿⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣍⣁⣈⣩⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠷⠿⠿⠿⠿⠾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠷⠶⠶⠶⠶⠾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣶⣶⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
"#;

        for line in art.lines() 
        {
            println!("{}", line);
            thread::sleep(Duration::from_millis(250));
        }

        let mut choice_input: String = String::new();

        println!("What do you want to do?\n\
        1.Advance\n\
        2.Rest\n\
        3.exit");

        io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read line");
                
        let choice_input = choice_input.trim();

        if choice_input == "1" 
        {
            let encounter = encounters::get_random_encounter();
            let mut rng = rand::rng();

            if encounter == "Enemy"
            {
                let enemy_index = rng.random_range(0..game_data.enemies.len());

                let base_enemy = game_data.enemies[enemy_index].clone();

                let mut scaled_enemy = scale_enemy_stats(base_enemy, game_data.player.stage as i32);

                player_is_alive = combat::combat_loop(&mut game_data.player, &mut scaled_enemy);
                        
                if player_is_alive 
                {
                    game_data.player.stage += 1;

                    heal_times = 3;

                    println!("You have advanced to stage {}. Keep it up!", game_data.player.stage);
                }
            }
            else if encounter == "Nothing"
            {
                println!("You encountered nothing worth noting");

                thread::sleep(Duration::from_secs(1));

                game_data.player.stage += 1;

                heal_times = 3;

                println!("You have advanced to stage {}. Keep it up!", game_data.player.stage);
            }
            else if encounter =="Rest Zone"
            {
                println!("You stumble upon a sheltered area, perfect for a quick rest.");

                game_data.player.stats.health += 25;

                if game_data.player.stats.health > game_data.player.stats.max_health 
                {
                    game_data.player.stats.health = game_data.player.stats.max_health;
                }

                println!("You feel refreshed. Your health is now {}", game_data.player.stats.health);

                thread::sleep(Duration::from_secs(1));

                game_data.player.stage += 1;

                heal_times = 3;

                println!("You have advanced to stage {}. Keep it up!", game_data.player.stage);

            }
            else if encounter == "Supplies"
            {
                println!("You find an abandoned supply crate. Inside is a new weapon!");

                game_data.player.stats.attack += 5; 

                println!("Your attack power has increased to {}", game_data.player.stats.attack);

                thread::sleep(Duration::from_secs(1));

                game_data.player.stage += 1;

                heal_times = 3;

                println!("You have advanced to stage {}. Keep it up!", game_data.player.stage);
            }
            else
            {
                println!("Error event generator failed. Try again");
            }

            if let Err(e) = save_game_data(&game_data) 
            {
                eprintln!("Failed to save game data: {}", e);
            } 
            else 
            {
                println!("Game saved!");
            }
        }
        else if choice_input == "2" 
        {
            let heal_amount = 10;

            if heal_times > 0
            {
                game_data.player.stats.health += heal_amount;

                if game_data.player.stats.health > game_data.player.stats.max_health
                {
                    game_data.player.stats.health = game_data.player.stats.max_health;
                }

                heal_times -= 1;

                println!("Times you can rest left: {}", heal_times);

                println!("You rest for a moment, gaining {} health. Health: {}", heal_amount, game_data.player.stats.health);
 
                if let Err(e) = save_game_data(&game_data) 
                {
                    eprintln!("Failed to save game data: {}", e);
                }

                thread::sleep(Duration::from_secs(1));
            }
            else if heal_times == 0 
            {
                println!("You can't rest right now get moving!");
            }
            else 
            {
                println!("Something happened this is an error idk what happened");
            }
        }

        else if choice_input == "3"
        {
            println!("Attempting to save game...");

            if let Err(e) = save_game_data(&game_data) 
            {
                eprintln!("Failed to save game data: {}", e);
            } 
            else
            {
                println!("Game saved! You will have to return eventually.");
            }

            thread::sleep(Duration::from_secs(1));
            break;
        }
        else
        {
            println!("Invalid input please try again");
        }
    }
}

fn main() 
{
    println!("==========================================");
    println!("Welcome to entrenched nightmare");
    println!("==========================================");

    let mut choice: String = String::new();

    let loaded_data = load_game_data();

    loop
    {
        choice.clear();

        println!("Choose an Option");

        match &loaded_data 
        {
            Ok(data) =>
            {
                println!("1.Continue Game (Stage {})", data.player.stage);

                println!("2.New Game");
            },

            Err(_) =>
            {
                println!("1.Start New Game");
            }
        }
        
        println!("3.Exit");

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        match choice.trim()
        {
            "1" => 
            {
                match loaded_data 
                {
                    Ok(data) => 
                    {
                        game_loop(data);
                        break;
                    },
                    Err(_) =>
                    {
                        match fs::read_to_string(SAVE_FILE_PATH) 
                        {
                            Ok(default_json) => 
                            {
                                match serde_json::from_str::<game_data::GameData>(&default_json) 
                                {
                                    Ok(mut default_data) => 
                                    {
                                        default_data.player.stats.health = default_data.player.stats.max_health;

                                        default_data.player.stats.attack = default_data.player.stats.default_attack;

                                        default_data.player.stats.defense = default_data.player.stats.default_defense;

                                        default_data.player.stage = 1;

                                        game_loop(default_data);

                                        break;
                                    },
                                    Err(e) => 
                                    {
                                        eprintln!("Failed to parse default stats.json: {}", e);

                                        return;
                                    }
                                }
                            },
                            Err(e) => 
                            {
                                eprintln!("Cannot find or read initial stats.json for New Game: {}", e);

                                return;
                            }
                        }
                    }
                }
            },

            "2" =>
            {
                println!("Starting New Game");

                match fs::read_to_string(SAVE_FILE_PATH)
                {
                    Ok(default_json) =>
                    {
                        match serde_json::from_str::<game_data::GameData>(&default_json)
                        {
                            Ok(mut default_data) =>
                            {
                                default_data.player.stats.health = default_data.player.stats.max_health;

                                default_data.player.stats.attack = default_data.player.stats.default_attack;

                                default_data.player.stats.defense = default_data.player.stats.default_defense;

                                default_data.player.stage = 1;

                                game_loop(default_data);

                                break;
                            },

                            Err(e) =>
                            {
                                eprintln!("Failed to parse default stats.json: {}", e);

                                return;
                            }
                        }
                    },

                    Err(e) =>
                    {
                        eprintln!("Cannot find or read initial stats.json for New Game: {}", e);

                        return;
                    }
                }
            },

            "3" =>
            {
                println!("I guess you don't want to get out, Oh well the trench always awaits.");

                thread::sleep(Duration::from_secs(1));

                break;
            },
            _ => 
            {
                println!("Invalid input. Please try again.");
            }
        }
    }
}