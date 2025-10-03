use std::io;
use std::thread;
use std::time::Duration;
use crate::game_data::{Player, Enemy};
use rand::Rng;
use crate::encounters;


pub fn combat_loop(player: &mut Player, enemy: &mut Enemy) -> bool
{
    println!("You encounter a {} it spots you and decides to attack", enemy.name);

    enemy.stats.health = enemy.stats.max_health;

    loop 
    {
        println!();
        println!("Your Turn to attack!");

        let mut player_action_input: String = String::new();

        println!("What do you do?\n\
        1.Attack!\n\
        2.Rest\n\
        3.Flee");

        io::stdin()
        .read_line(&mut player_action_input)
        .expect("Failed to read line");

        let player_action_input = player_action_input.trim();

        if player_action_input == "1"
        {
            let mut attack_check_rng = rand::rng();
            let attack_check :i32 = attack_check_rng.random_range(1..=4);

            println!("You rolled a {}", attack_check);

            match attack_check 
            {
                1 => 
                {
                    println!("Attack failed try again!");

                    thread::sleep(Duration::from_secs(1));
                },

                2 => 
                {
                    let attack_damage: i32 = player.stats.attack / 2;
                    let mut attack: i32 = attack_damage - enemy.stats.defense;

                    if attack < 0 
                    {
                        attack = 0;
                    }

                    enemy.stats.health -= attack;

                    if enemy.stats.health < 0
                    {
                        enemy.stats.health = 0;
                    }

                    let attack_message: String = encounters::get_random_attack_message_player(&enemy.name);

                    println!("{}", attack_message);
                    println!("You dealt {} damage", attack);
                    println!("Enemy has {} health left", enemy.stats.health);
                    
                    thread::sleep(Duration::from_secs(1));
                },

                3 =>
                {
                    let mut attack: i32 = player.stats.attack - enemy.stats.defense;

                    if attack < 0
                    {
                        attack = 0;
                    }

                    enemy.stats.health -= attack;

                    if enemy.stats.health < 0
                    {
                        enemy.stats.health = 0;
                    }

                    let attack_message: String = encounters::get_random_attack_message_player(&enemy.name);

                    println!("{}", attack_message);
                    println!("You dealt {} damage", attack);
                    println!("Enemy has {} health left", enemy.stats.health);
                },

                4 =>
                {
                    let attack_damage: i32 = player.stats.attack * 2;
                    let mut attack: i32 = attack_damage - enemy.stats.defense;

                    if attack < 0
                    {
                        attack = 0;
                    }

                    enemy.stats.health -= attack;

                    if enemy.stats.health < 0
                    {
                        enemy.stats.health = 0;
                    }

                    let attack_message: String = encounters::get_random_attack_message_player(&enemy.name);

                    println!("CRITICAL HIT!");
                    println!("{}", attack_message);
                    println!("You dealt {} damage", attack);
                    println!("Enemy has {} health left", enemy.stats.health);
                },

                _ => 
                {
                    println!("Code error dice roll not in roll range");
                }
            }
        }
        else if player_action_input == "2"
        {
            let heal_amount = 10;

            player.stats.health += heal_amount;

            if player.stats.health > player.stats.max_health
            {
                player.stats.health = player.stats.max_health;
            }

            println!("You healed {}", heal_amount);
            println!("Your health is now {}", player.stats.health);
        }
        else if player_action_input == "3"
        {
            let mut flee_check_rng = rand::rng();
            let flee_check :f64= flee_check_rng.random();

            if flee_check <= 0.3
            {
                println!("You were able to escape the {} you coward", enemy.name);
                thread::sleep(Duration::from_secs(1));
                return true;
            }
            else
            {
                println!("You really thought you could escape? its pretty dificult, I don't recommend you try again");
                println!("But yet again who am I to tell you what to do, coward");
                thread::sleep(Duration::from_secs(1));
            }
        }
        else
        {
            println!("Oops invalid input we will take that as giving up your turn.\n\
            You should actually try to play the game next time");
            thread::sleep(Duration::from_secs(1));
        }

        // Enemy attack
        println!();
        println!("{}'s turn", enemy.name);

        thread::sleep(Duration::from_secs(1));

        let mut enemy_attack_check_rng = rand::rng();
        let enemy_attack_check :i32= enemy_attack_check_rng.random_range(1..=4);

        match enemy_attack_check 
        {
            1 =>
            {
                println!("Enemy attack fialed");

                thread::sleep(Duration::from_secs(1));
            },
            2 =>
            {
                let enemy_attack_damage: i32 = enemy.stats.attack / 2;
                let mut enemy_attack: i32 = enemy_attack_damage - player.stats.defense;

                if enemy_attack < 0 
                {
                    enemy_attack = 0;
                }

                player.stats.health -= enemy_attack;

                if player.stats.health < 0
                {
                    player.stats.health = 0;
                }

                println!("{}'s attack dealt {} damage",enemy.name, enemy_attack);
                println!("You have {} health left", player.stats.health);

                thread::sleep(Duration::from_secs(1));
            },
            3 =>
            {
                let mut enemy_attack: i32 = enemy.stats.attack - player.stats.defense;

                if enemy_attack < 0 
                {
                    enemy_attack = 0;
                }

                player.stats.health -= enemy_attack;

                if player.stats.health < 0
                {
                    player.stats.health = 0;
                }

                println!("{}'s attack dealt {} damage",enemy.name, enemy_attack);
                println!("You have {} health left", player.stats.health);
            },
            4 => 
            {
                let enemy_attack_damage: i32 = enemy.stats.attack * 2;
                let mut enemy_attack: i32 = enemy_attack_damage - player.stats.defense;

                if enemy_attack < 0 
                {
                    enemy_attack = 0;
                }

                player.stats.health -= enemy_attack;

                if player.stats.health < 0
                {
                    player.stats.health = 0;
                }

                println!("CRITICAL HIT!");
                println!("{}'s attack dealt {} damage",enemy.name, enemy_attack);
                println!("You have {} health left", player.stats.health);

                thread::sleep(Duration::from_secs(1));
            },
            _ =>
            {

            }
        }

        if player.stats.health <= 0
        {
            player.stats.health = 0;
            println!("You were defeated by the {}", enemy.name);
            thread::sleep(Duration::from_secs(1));
            return false;
        }
        else if enemy.stats.health <= 0
        {
            println!("You defeated the {}", enemy.name);
            thread::sleep(Duration::from_secs(1));
            return true;
        }
    }
}