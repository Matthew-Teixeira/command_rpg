use std::io;
use std::cmp::Ordering;
use rand::Rng;
mod submodules;
use submodules::game::start::player_setup;
use submodules::game::actions::actions;
use submodules::game::actions::attack;
use submodules::game::combatants::enemy_attack;
use submodules::game::combatants::enemies;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut player = player_setup();
    //println!("{:#?}", player);

    println!("\nPlayer Character Has Been Created.");

    // Create Enemy
    let mut enemy = enemies::Enemy::new(
        enemies::EnemyClasses::AcolyteMage,
        String::from("Randome Mage")
    );

    sleep(Duration::from_millis(1500));
    println!("Walking...");
    sleep(Duration::from_millis(1500));
    println!("Reaching top of hill");
    sleep(Duration::from_millis(1500));
    println!("What is that?");
    sleep(Duration::from_millis(1500));

    println!("\n ENEMY HAS APPEARED");
    sleep(Duration::from_millis(1000));

    sleep(Duration::from_millis(3000));

    loop {
        println!("\nWhat will you do?\n");
        let mut action_no = String::new();

        // List actions available
        actions::Actions::view_actions();

        // Get player input on action chossen as a number
        io::stdin().read_line(&mut action_no).expect("Failed to read line");

        // Get the action number
        let action_no: usize = match action_no.trim().parse() {
            Ok(num) => { num }
            Err(_) => {
                println!("Please Select A Number");
                continue;
            }
        };

        // Get the corresponding action
        let action = actions::Actions::from_index(action_no);

        let action = match action {
            Some(act) => act,
            None => actions::Actions::Skip,
        };

        println!("\nYou have choosen {:?}", action);
        sleep(Duration::from_millis(1500));

        match action {
            actions::Actions::Attack => {
                attack::attack_enemy(&mut player, &mut enemy);
            }
            actions::Actions::Defend => {}
            actions::Actions::ViewInvintory => {}
            actions::Actions::UsePotion => {}
            actions::Actions::Skip => {}
            actions::Actions::ViewStats => {
                player.get_stats();
                continue;
            }
            actions::Actions::ViewEnemyStats => {
                enemy.get_stats();
                continue;
            }
            other => println!("Not an action"),
        }

        sleep(Duration::from_millis(1000));
        println!("\n{}'s Health: {}", player.name, player.health);
        println!("{}'s Health: {}\n", enemy.name, enemy.health);
        sleep(Duration::from_millis(3000));

        if enemy.health <= 0.0 {
            println!("You have defeted {}", enemy.name);
            break;
        } else if player.health <= 0.0 {
            println!("{}, you have been defeted by {}", player.name, enemy.name);
            break;
        }

        //enemy attack
        enemy_attack::enemy_attack(&mut player, &mut enemy);

        if enemy.health <= 0.0 {
            println!("You have defeted {}", enemy.name);
            break;
        } else if player.health <= 0.0 {
            println!("{}, you have been defeted by {}", player.name, enemy.name);
            break;
        }
    }
}
