use crate::submodules::game::damage_types::damage;

#[derive(Debug)]
pub enum WeaponType {
    Staff(Staff),
    Sword1H(Sword1H),
    Sword2H(Sword2H),
    // Shield(Shield),
    //Staff1H(Staff1H),
    //Staff2H(Staff2H),
    Bow(Bow),
    NoWeapon(Fist),
}

#[derive(Debug)]
pub struct Fist {
    pub name: String,
    pub lv: usize,
    pub atk: i32,
    pub def: i32,
    pub atk_type: damage::AttackTypes,
    pub ele_dam: i32,
    pub ele_dam_type: damage::ElementTypes,
    pub one_handed: bool,
}

#[derive(Debug)]
pub struct Sword1H {
    pub name: String,
    pub lv: usize,
    pub atk: i32,
    pub def: i32,
    pub atk_type: damage::AttackTypes,
    pub ele_dam: i32,
    pub ele_dam_type: damage::ElementTypes,
    pub one_handed: bool,
}

#[derive(Debug)]
pub struct Sword2H {
    pub name: String,
    pub lv: usize,
    pub atk: i32,
    pub def: i32,
    pub atk_type: damage::AttackTypes,
    pub ele_dam: i32,
    pub ele_dam_type: damage::ElementTypes,
    pub one_handed: bool,
}

#[derive(Debug)]
pub struct Staff {
    pub name: String,
    pub lv: usize,
    pub atk: i32,
    pub def: i32,
    pub atk_type: damage::AttackTypes,
    pub ele_dam: i32,
    pub ele_dam_type: damage::ElementTypes,
    pub one_handed: bool,
}

#[derive(Debug)]
pub struct Bow {
    pub name: String,
    pub lv: usize,
    pub atk: i32,
    pub def: i32,
    pub atk_type: damage::AttackTypes,
    pub ele_dam: i32,
    pub ele_dam_type: damage::ElementTypes,
    pub one_handed: bool,
}
