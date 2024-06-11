use crate::submodules::game::combatants::enemies;
use crate::submodules::game::game_weapons::weapon_types;
use crate::submodules::game::user_classes::user_classes;
use crate::submodules::game::utils::crit_damage;
use std::thread::sleep;
use std::time::Duration;

pub fn enemy_attack(user: &mut user_classes::Player, bad_guy: &mut enemies::Enemy) {
    let bad_guy_weapon = &bad_guy.main_weap;

    let crit_value = crit_damage::crit_mod_value(&bad_guy.name);

    let damage: f64 = match bad_guy_weapon {
        weapon_types::WeaponType::Staff(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((bad_guy.int as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::Sword2H(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((bad_guy.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::Sword1H(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((bad_guy.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::Bow(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((bad_guy.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::NoWeapon(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((bad_guy.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
    };

    user.health = user.health - damage;
    println!("\nEnemy Attacking {}...", user.name);
    sleep(Duration::from_millis(1500));
    println!("\nEnemy delt {} to {}", damage, user.name);
    sleep(Duration::from_millis(1500));
}
