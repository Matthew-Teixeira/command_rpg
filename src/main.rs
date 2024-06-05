use std::io;
use std::cmp::Ordering;
use rand::Rng;
mod submodules;
use submodules::game::start::player_setup;
use submodules::game::actions::actions;

fn main() {
    let player = player_setup();
    //println!("{:#?}", player);

    player.get_stats();
    loop {
        println!("\nChoose your action.");
        let mut action_no = String::new();

        // List actions available
        actions::view_actions();

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

        println!("Action Number: {}", action_no);
        println!("Action: {:?}", action);
    }
}
