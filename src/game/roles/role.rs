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

pub trait Role {
    fn can_do_special_action(&self, game: &Game) -> bool;
    fn act(&self, target: &mut Player, game: &mut Game);
    fn name(&self) -> RoleName;
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
