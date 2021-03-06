use super::prelude::*;
use crate::game::fsm::TimeBlock;

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
        matches!(
            self,
            RoleName::King | RoleName::TheDouble | RoleName::Prince
        )
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
    fn name(&self) -> RoleName;
    fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool;
    fn wrap(self) -> RoleHolder;
}

impl Into<DeathCause> for RoleName {
    fn into(self) -> DeathCause {
        match self {
            RoleName::Sorcerer => DeathCause::Sorcery,
            RoleName::Knight => DeathCause::Beheading,
            RoleName::Revolutionary => DeathCause::Assassination,
            _ => panic!(
                "RoleName.into::<KilledBy> should never be called on {:?}",
                self
            ),
        }
    }
}
