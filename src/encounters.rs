use rand::Rng;

pub fn get_random_entrance_message() -> String 
{
    let entrance = vec![
        "You exit the bunker after a gas attack the air reeks of death.\n\
        You know that your greates priority is to get out of this trench no matter what.\n\
        You had heard rumors of what that gas did to people you just hoped that they weren't real \n\
        that is until you heard outside what happened to those who weren't able to make it to the bunker on time,\n\
        And how they turned on those who had gasmasks.\n\
        Weapon in hand you begin your attempt of escaping this place",
        "You awoke panicking flashes of the horrific event that happened right infront of your eyes.\n\
        It was a gas attack like any other, you were able to get your mask on but many didn't have one,\n\
        or weren't able to make it to the bunkers in time. What should have been another mass loss turned into\n\
        a horrifying event as the gas twisted and corrupted those who were exposed into horrifying creatures.\n\
        Flesh and bone contorted in unimaginable ways that shouldn't be possible into monstrous beings.\n\
        The last thing you remembered whas being chucked into a pile of wooden crates by one of those creatures.\n\
        You couldn't even fathom how you were still alive, grabing your trusty weapon you decide that you are\n\
        going to make it out if it is the last thing you do so you begin your dangerous task.",
        "You were on the toilet when it all happened taking a poo, luckily you always bring your gasmask\n\
        with you to the toilet because of misfortunes you had heard of terrible mishaps while on the John.\n\
        You also brought your rifle in with you because you never know what could happen\n\
        You had heard the commotion outside but were not finished wiping so you couldn't go out and help.\n\
        Once you finished doing your buissiness you walk out of the outhouse rifle in hand ready to face the world,\n\
        while toilet paper stuck to your foot trailed behind you."
    ];

    let mut rng = rand::rng();
    let index = rng.random_range(0..entrance.len());
    entrance[index].to_string()
}

pub fn get_random_encounter () -> String
{
    let random_encounter = vec![
        "Nothing",
        "Enemy",
        "Rest Zone",
        "Supplies"
    ];

    let mut rng = rand::rng();
    let index = rng.random_range(0..random_encounter.len());
    random_encounter[index].to_string()
}

pub fn get_random_attack_message_player(enemy_name: &str) -> String 
{
    let random_attack_message_player = vec![
        "You shoot the {} in the stomach blood spraying the area behind it as the bullet goes through",
        "You stab the {} in the chest with your bayonet twisting it to maximize damage",
        "You clock the {} in the head with your rifle butt",
        "You club the {} with your mace",
        "You chopt at the {} with your trench shovel",
        "You throw the contents of a latrine at the face of the {}"
    ];
    let mut rng = rand::rng();
    let index = rng.random_range(0..random_attack_message_player.len());
    random_attack_message_player[index].to_string().replace("{}", enemy_name)
}

pub fn get_death_message_per_stage(stage:u32) -> String
{
    match stage 
    {
        1..=2 => 
        {
            "You were so confident but you weren't even able to make it that far before you got ended.\n\
            The trench reclaims what you have lost and you are yet another warning for those who are to come.\n\
            Your corpse nothing but a maker on the path."
            .to_string()
        }
        3..=5 => 
        {
            "You learned how to fight, but the mutated horrors proved too numerous.\n\
            Your last thought is of the life you could have lived outside this muddy hell."
            .to_string()
        }
        6..=8 => {
            "So close! You could smell the fresh air, but your wounds were too deep.\n\
            You died a hero's death, but the trench keeps its trophies close."
            .to_string()
        }
        9 => {
            "The finish line was in sight, but a final, powerful enemy ended your journey.\n\
            The escape was an illusion; the trench never intended to let you go."
            .to_string()
        }
        // Catch-all for any other stage, including a new game (stage 0 or 1)
        _ => 
        {
            "You were defeated. The trench reclaims what little hope you had left."
            .to_string()
        }
    }
}