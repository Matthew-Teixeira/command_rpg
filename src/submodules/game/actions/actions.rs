#[derive(Debug)]
pub enum Actions {
    Attack,
    Defend,
    UsePotion,
    ViewInvintory,
    Skip,
    ViewStats,
    ViewEnemyStats,
}

impl Actions {
    pub fn variants() -> &'static [Actions] {
        static VARIANTS: &[Actions] = &[
            Actions::Attack,
            Actions::Defend,
            Actions::UsePotion,
            Actions::ViewInvintory,
            Actions::Skip,
            Actions::ViewStats,
            Actions::ViewEnemyStats,
        ];
        VARIANTS
    }

    pub fn from_index(index: usize) -> Option<Actions> {
        match index {
            1 => Some(Actions::Attack),
            2 => Some(Actions::Defend),
            3 => Some(Actions::UsePotion),
            4 => Some(Actions::ViewInvintory),
            5 => Some(Actions::Skip),
            6 => Some(Actions::ViewStats),
            7 => Some(Actions::ViewEnemyStats),
            _ => None,
        }
    }

    pub fn view_actions() {
        for (index, action) in Actions::variants().iter().enumerate() {
            println!("{}: {:?}", index + 1, action);
        }
    }
}
