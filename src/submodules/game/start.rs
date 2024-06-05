use std::io;
use std::cmp::Ordering;
use rand::Rng;
use crate::submodules::game::user_classes::user_classes;
use std::thread::sleep;
use std::time::Duration;

pub fn player_setup() -> user_classes::Player {
    println!("\n\n\nHello and welcome to the GAME!");
    sleep(Duration::from_millis(1500));
    println!("\nPlease Enter your name.");

    let mut player_name: String = String::new();
    io::stdin().read_line(&mut player_name).expect("Failed to read line");
    let player_name = player_name.trim();

    sleep(Duration::from_millis(700));
    println!("\nHello {}", player_name);
    sleep(Duration::from_millis(1000));
    println!("\nPlease select your class number!\n");

    let class_type = select_class();

    let new_player = user_classes::Player::new(class_type, player_name);

    sleep(Duration::from_millis(500));
    new_player
}

fn select_class() -> user_classes::UserClass {
    loop {
        let mut class_no: String = String::new();
        for (index, class) in user_classes::UserClass::variants().iter().enumerate() {
            println!("{}: {:?}", index + 1, class);
        }

        io::stdin().read_line(&mut class_no).expect("Failed to read line");

        let class_no: usize = match class_no.trim().parse() {
            Ok(num) => { num }
            Err(_) => {
                println!("Please Select A Number");
                continue;
            }
        };

        let class = match user_classes::UserClass::from_index(class_no) {
            Some(c) => c,
            None => {
                println!("Invalid class number.");
                continue;
            }
        };

        return class;
    }
}

/* 
let secret_number = rand::thread_rng().gen_range(1..=100);
 io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
*/
