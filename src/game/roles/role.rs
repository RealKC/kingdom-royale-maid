use super::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoleName {
    King,
    Prince,
    TheDouble,
    Sorcerer,
    Knight,
    Revolutionary,
}

impl RoleName {
    pub fn is_king_like(&self) -> bool {
        match self {
            RoleName::King | RoleName::TheDouble | RoleName::Prince => true,
            _ => false,
        }
    }
}

impl ToString for RoleName {
    fn to_string(&self) -> String {
        match self {
            RoleName::King => "King".to_string(),
            RoleName::Prince => "Prince".to_string(),
            RoleName::TheDouble => "The Double".to_string(),
            RoleName::Sorcerer => "Sorcerer".to_string(),
            RoleName::Knight => "Knight".to_string(),
            RoleName::Revolutionary => "Revolutionary".to_string(),
        }
    }
}

pub trait Role {
    fn can_do_special_action(&self, game: &Game) -> bool;
    fn act(&self, target: &mut Player);
    fn name(&self) -> RoleName;
    fn win_condition_achieved(&self, game: &Game) -> bool;
}

impl Into<KilledBy> for RoleName {
    fn into(self) -> KilledBy {
        match self {
            RoleName::Sorcerer => KilledBy::Sorcerer,
            RoleName::Knight => KilledBy::Knight,
            RoleName::Revolutionary => KilledBy::Revolutionary,
            _ => panic!(
                "RoleName.into::<KilledBy> should never be called on {:?}",
                self
            ),
        }
    }
}
