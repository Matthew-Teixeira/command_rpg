use crate::submodules::game::game_weapons::weapon_types;
use crate::submodules::game::damage_types::damage;

#[derive(Debug)]
pub enum UserClass {
    Mage,
    Warrior,
    Archer,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub class: UserClass,
    pub equ_inventory: i32,
    pub potion_inventory: i32,
    pub health: i32,
    pub str: i32,
    pub dex: i32,
    pub stam: i32,
    pub int: i32,
    pub main_weap: weapon_types::WeaponType,
    pub off_weap: weapon_types::WeaponType,
}

impl Player {
    pub fn new(class_type: UserClass, player_name: &str) -> Self {
        match class_type {
            UserClass::Mage => {
                Self {
                    name: String::from(player_name),
                    class: class_type,
                    equ_inventory: 10,
                    potion_inventory: 15,
                    health: 20,
                    str: 6,
                    dex: 12,
                    stam: 6,
                    int: 16,
                    main_weap: weapon_types::WeaponType::Staff(weapon_types::Staff {
                        name: String::from("Broken Staff"),
                        lv: 1,
                        atk: 8,
                        def: 2,
                        atk_type: damage::AttackTypes::Magic,
                        ele_dam: 0,
                        ele_dam_type: damage::ElementTypes::NoElement,
                        one_handed: false,
                    }),
                    off_weap: weapon_types::WeaponType::NoWeapon,
                }
            }
            UserClass::Warrior => {
                Self {
                    name: String::from(player_name),
                    class: class_type,
                    equ_inventory: 15,
                    potion_inventory: 10,
                    health: 20,
                    str: 16,
                    dex: 10,
                    stam: 10,
                    int: 4,
                    main_weap: weapon_types::WeaponType::Sword2H(weapon_types::Sword2H {
                        name: String::from("Broken Claymore"),
                        lv: 1,
                        atk: 10,
                        def: 0,
                        atk_type: damage::AttackTypes::Physical,
                        ele_dam: 0,
                        ele_dam_type: damage::ElementTypes::NoElement,
                        one_handed: false,
                    }),
                    off_weap: weapon_types::WeaponType::NoWeapon,
                }
            }
            UserClass::Archer => {
                Self {
                    name: String::from(player_name),
                    class: class_type,
                    equ_inventory: 13,
                    potion_inventory: 12,
                    health: 20,
                    str: 12,
                    dex: 12,
                    stam: 12,
                    int: 4,
                    main_weap: weapon_types::WeaponType::Bow(weapon_types::Bow {
                        name: String::from("Broken Bow"),
                        lv: 1,
                        atk: 10,
                        def: 0,
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

impl UserClass {
    pub fn variants() -> &'static [UserClass] {
        static VARIANTS: &[UserClass] = &[UserClass::Mage, UserClass::Warrior, UserClass::Archer];
        VARIANTS
    }

    pub fn from_index(index: usize) -> Option<UserClass> {
        match index {
            1 => Some(UserClass::Mage),
            2 => Some(UserClass::Warrior),
            3 => Some(UserClass::Archer),
            _ => None,
        }
    }
}

/* 

self.name = String::from("Apollo");
self.class = Mage;
self.equ_inventory = 10;
self.potion_inventory = 12;
self.str = 8;
self.dex = 8;
self.stam = 8;
self.int = 16;
*/
