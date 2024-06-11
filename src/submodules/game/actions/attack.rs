use crate::submodules::game::combatants::enemies;
use crate::submodules::game::game_weapons::weapon_types;
use crate::submodules::game::user_classes::user_classes;
use crate::submodules::game::utils::crit_damage;
use std::thread::sleep;
use std::time::Duration;

pub fn attack_enemy(user: &mut user_classes::Player, bad_guy: &mut enemies::Enemy) {
    // Damage=(Base Attack+Weapon Power)×Level Modifier−Defense
    // Damage_With_Element =((Base Attack + Weapon Power) × Level Modifier × Random Factor) − Defense
    let user_weapon = &user.main_weap;
    let bad_guy_weapon = &bad_guy.main_weap;

    /* if let weapon_types::WeaponType::Sword2H(_) = user_weapon {
        println!("USER WEAPON IS A 2 HANDED SWORD");
    } */

    let crit_value = crit_damage::crit_mod_value(&user.name);

    let damage = match user_weapon {
        weapon_types::WeaponType::Sword2H(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((user.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::Sword1H(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((user.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::Bow(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((user.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::Staff(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((user.int as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
        weapon_types::WeaponType::NoWeapon(weap) => {
            let mut def: f64 = (100.0 * (user.level as f64) - (bad_guy.def as f64)) / 100.0;
            let d: f64 = ((user.str as f64) + (weap.atk as f64)) * def;
            d * crit_value
        }
    };

    bad_guy.health = bad_guy.health - damage;
    println!("\nAttacking {}...", bad_guy.name);
    sleep(Duration::from_millis(1500));
    println!("\nYou delt {} to {}", damage, bad_guy.name);
    sleep(Duration::from_millis(1500));
}

/* 
-- Base Attack: The intrinsic attack strength of the player, which could include the player's strength or any other stat that contributes to attack power.
-- Weapon Power: Additional attack power contributed by the weapon.
-- Level Modifier: A multiplier based on the attacker's level to scale the attack appropriately.
-- Defense: The defense rating of the enemy which reduces the incoming damage.
-- Random Factor: A random multiplier between, say, 0.85 and 1.15, to add unpredictability to the attack results.

Player {
    name: "Big Dog",
    class: Warrior,
    equ_inventory: 15,
    potion_inventory: 10,
    health: 14,
    str: 16,
    dex: 10,
    stam: 10,
    int: 4,
    main_weap: Sword2H(
        Sword2H {
            name: "Broken Claymore",
            lv: 1,
            atk: 10,
            def: 0,
            atk_type: Physical,
            ele_dam: 0,
            ele_dam_type: NoElement,
            one_handed: false,
        },
    ),
    off_weap: NoWeapon,
}

    Enemy {
    name: "Randome Mage",
    class: AcolyteMage,
    health: 10,
    str: 2,
    dex: 5,
    stam: 2,
    int: 8,
    main_weap: Staff(
        Staff {
            name: "Magic Wood",
            lv: 1,
            atk: 4,
            def: 0,
            atk_type: Magic,
            ele_dam: 0,
            ele_dam_type: NoElement,
            one_handed: false,
        },
    ),
    off_weap: NoWeapon,
}
*/
