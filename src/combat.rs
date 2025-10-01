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
            let attack_check :f64= attack_check_rng.random();

            if attack_check <= 0.5
            {
                let attack: i32 = player.stats.attack - enemy.stats.defense;
                enemy.stats.health -= attack;

                let attack_message = encounters::get_random_attack_message_player(&enemy.name);
                println!("{}", attack_message);
                println!("You dealt {} damage", attack);
                println!("Enemy has {} health left", enemy.stats.health);
                thread::sleep(Duration::from_secs(1));
            }
            else
            {
                println!("Attack failed try again!");
                thread::sleep(Duration::from_secs(1));
                //probably going to make a random failed attack message in the future
            }
        }
        else if player_action_input == "2"
        {
            //rest and heal up
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
        let enemy_attack_check :f64= enemy_attack_check_rng.random();

        if enemy_attack_check < 0.5
        {
            let enemy_attack: i32 = enemy.stats.attack - player.stats.defense;
            player.stats.health -= enemy_attack;

            println!("{}'s attack dealt {} damage",enemy.name, enemy_attack);
            println!("You have {} health left", player.stats.health);
            thread::sleep(Duration::from_secs(1));
        }
        else 
        {
            println!("{}'s attack failed", enemy.name);
            thread::sleep(Duration::from_secs(1));
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