use rand::distr::weighted::WeightedIndex;
use rand::Rng;
use rand::distr::Distribution;

pub fn get_random_entrance_message() -> String 
{
    let entrance = vec![
        "You exit the bunker after a gas attack the air reeks of death. \n\
        You know that your greates priority is to get out of this trench no matter what. \n\
        You had heard rumors of what that gas did to people you just hoped that they weren't real \n\
        that is until you heard outside what happened to those who weren't able to make it to the bunker on time, \n\
        And how they turned on those who had gasmasks. \n\
        Weapon in hand you begin your attempt of escaping this place",
        "You awoke panicking flashes of the horrific event that happened right infront of your eyes. \n\
        It was a gas attack like any other, you were able to get your mask on but many didn't have one, \n\
        or weren't able to make it to the bunkers in time. What should have been another mass loss turned into \n\
        a horrifying event as the gas twisted and corrupted those who were exposed into horrifying creatures. \n\
        Flesh and bone contorted in unimaginable ways that shouldn't be possible into monstrous beings. \n\
        The last thing you remembered whas being chucked into a pile of wooden crates by one of those creatures. \n\
        You couldn't even fathom how you were still alive, grabing your trusty weapon you decide that you are \n\
        going to make it out if it is the last thing you do so you begin your dangerous task.",
        "You were on the toilet when it all happened taking a poo, luckily you always bring your gasmask \n\
        with you to the toilet because of misfortunes you had heard of terrible mishaps while on the John. \n\
        You also brought your rifle in with you because you never know what could happen \n\
        You had heard the commotion outside but were not finished wiping so you couldn't go out and help. \n\
        Once you finished doing your buissiness you walk out of the outhouse rifle in hand ready to face the world, \n\
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

    let weights = [1, 4, 2, 3];

    let dist = WeightedIndex::new(&weights).unwrap();

    

    let mut rng = rand::rng();
    let index = dist.sample(&mut rng);
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
            "You were so confident but you weren't even able to make it that far before you got ended. \n\
            You were horrified at the sound of your own bones being crushed the pain become an unbearable \n\
            indescribeable feeling, you wanted it to be over so bad but the creature that your fate lied in the hands \n\
            wasn't so merciful. The last thing you saw was another soldier nearby petrified with fear unable to move \n\
            while witnessing your fate. You couldn't balme him you would be the same in his position."
            .to_string()
        }
        3..=5 => 
        {
            "In the moments before you were ripped in half you thought back to why you were even here. \n\
            Why were you even fighting in this horrid war. But in those moments you realized, \n\
            It was all your fault, all those actions you chose all those things you did. \n\
            The only one you could blame was yourself, no matter how far back in your memory you searched, \n\
            All you could see was it was your fault. You did what you did and you enlisted to escape \n\
            the consequenses but it seems that they catched up to you no matter how far or how fast you could run. \n\
            In the end it will catch you."
            .to_string()
        }
        6..=8 => {
            "You could see it in the distance the entry ramp, the only safe way in or out of the trench. \n\
            You could see it from your vantage point, you could only wish that you could feel joy at it's sight. \n\
            But the only reason you could see it was because you were held up high by the creature that shoved an arm \n\
            through your chest. You could feel the warmth in your chest giveway to cold as you slowly bled out onto the creature. \n\
            It held you higher almost as if mocking you with the view of what could have been your escape. You felt bitter \n\
            knowing that you would never make it out, that this blasted trench was going to be your final resting place. \n\
            In your last thoughts you wished you could have seen her face one last time"
            .to_string()
        }
        9 => {
            "You were just about to make it, you tried crawling up the entrance ramp but then it grabbed your leg \n\
            Violently dragging you back. It seems the creature that had once been your compatriot had decided that your fight \n\
            wasn't over. You tried to escape as it held you up by your leg. That seemed to anger it since it decided that it would \n\
            tear you limb by limb. First it was your arms so because you tried punching it. Then it went for your legs. \n\
            You were now but a bloody stump laying on the floor in your own blood. In your last moments as the creature reached for \n\
            your head all you could think about was 3 things. Why me?, her face, and the thing you regreted the most. The creature \n\
            wrapping its claws around your head then applied force and that was it for you."
            .to_string()
        }
        // Catch-all for any other stage, including a new game (stage 0 or 1)
        _ => 
        {
            "Uh pretend you read a horrifying death message bc you shouldn't be able to see this yk like this means there has been an error \n\
            Were you tinkering with the code? Welp idk you got hit by a poop and died."
            .to_string()
        }
    }
}

pub fn get_win_message() -> String 
{
    let random_win_message = vec![
        "You crawled out, your clothes coverd in mud and blood, you rejoiced happy that you finally made it out of that place, you were crying tears of \n\
        joy. You started thinking about all the things you would do now, all the people that you would talk to. You were so happy that you decided \n\
        to start running making sure to keep as much distance as you and that trench as possible. You ran and ran feeling energetic even though you \n\
        had just gone through something horrible. You were thought that you had forgoten something something important but you didin't know what that was. \n\
        You then felt a sharp pain in your abdomen and then you heard it a gunshot. Off in the distance you saw some soldiers in redish brown uniform. \n\
        They were aiming rifles at you. Then you realized what you had forgotten. The war. You held your hand against the wound, warm blood trickling \n\
        down your fingers. Your happines drained like the warmth as you slowly started to lose consciousness."

    ];
    let mut rng = rand::rng();
    let index = rng.random_range(0..random_win_message.len());
    random_win_message[index].to_string()
}