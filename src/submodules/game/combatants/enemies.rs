use crate::submodules::game::game_weapons::weapon_types;
use crate::submodules::game::damage_types::damage;

#[derive(Debug)]
pub enum EnemyClasses {
    AcolyteMage,
    ApostleWarrior,
    AdeptArcher,
}

#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub class: EnemyClasses,
    pub health: i32,
    pub str: i32,
    pub dex: i32,
    pub stam: i32,
    pub int: i32,
    pub main_weap: weapon_types::WeaponType,
    pub off_weap: weapon_types::WeaponType,
}

impl Enemy {
    pub fn new(class_type: EnemyClasses, enemy_name: String) -> Self {
        match class_type {
            EnemyClasses::AcolyteMage => {
                Self {
                    name: enemy_name,
                    class: class_type,
                    health: 10,
                    str: 2,
                    dex: 5,
                    stam: 2,
                    int: 8,
                    main_weap: weapon_types::WeaponType::Staff(weapon_types::Staff {
                        name: String::from("Magic Wood"),
                        lv: 1,
                        atk: 4,
                        def: 0,
                        atk_type: damage::AttackTypes::Magic,
                        ele_dam: 0,
                        ele_dam_type: damage::ElementTypes::NoElement,
                        one_handed: false,
                    }),
                    off_weap: weapon_types::WeaponType::NoWeapon,
                }
            }
            EnemyClasses::ApostleWarrior => {
                Self {
                    name: enemy_name,
                    class: class_type,
                    health: 10,
                    str: 5,
                    dex: 3,
                    stam: 4,
                    int: 1,
                    main_weap: weapon_types::WeaponType::Sword2H(weapon_types::Sword2H {
                        name: String::from("Wood Stick"),
                        lv: 1,
                        atk: 3,
                        def: 1,
                        atk_type: damage::AttackTypes::Physical,
                        ele_dam: 0,
                        ele_dam_type: damage::ElementTypes::NoElement,
                        one_handed: false,
                    }),
                    off_weap: weapon_types::WeaponType::NoWeapon,
                }
            }
            EnemyClasses::AdeptArcher => {
                Self {
                    name: enemy_name,
                    class: class_type,
                    health: 8,
                    str: 4,
                    dex: 5,
                    stam: 4,
                    int: 2,
                    main_weap: weapon_types::WeaponType::Bow(weapon_types::Bow {
                        name: String::from("Stringed Branch"),
                        lv: 1,
                        atk: 3,
                        def: 1,
                        atk_type: damage::AttackTypes::Physical,
                        ele_dam: 0,
                        ele_dam_type: damage::ElementTypes::NoElement,
                        one_handed: false,
                    }),
                    off_weap: weapon_types::WeaponType::NoWeapon,
                }
            }
        }
    }

    pub fn get_stats(&self) {
        println!("{:#?}", self);
    }
}
