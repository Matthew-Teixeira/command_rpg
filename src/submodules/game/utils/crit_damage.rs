use rand::Rng;

pub fn is_crit_hit() -> bool {
    let mut rng = rand::thread_rng(); // Get a random number generator
    let num: i32 = rng.gen_range(1..6); // Generate a random number between 1 and 5
    println!("Random number: {}", num);
    if num == 5 {
        true
    } else {
        false
    }
}

pub fn crit_mod_value(name: &str) -> f64 {
    let is_crit = is_crit_hit();
    if is_crit {
        let mut rng = rand::thread_rng(); // Get a random number generator
        let modifier: i32 = rng.gen_range(2..10); // Generate a random number between 2 and 9
        let crit_multiplyer = 1.0 + (modifier as f64) / 10.0;
        println!("CRIT MOD NUMBER: {}", crit_multiplyer);
        println!("{name} landed a critical hit!!");
        crit_multiplyer
    } else {
        1.0
    }
}
