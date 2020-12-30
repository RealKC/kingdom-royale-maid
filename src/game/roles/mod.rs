mod role;

mod double;
mod king;
mod knight;
mod prince;
mod revolutionary;
mod sorcerer;

pub use role::{Role, RoleName};

pub use double::TheDouble;
pub use king::King;
pub use knight::Knight;
pub use prince::Prince;
pub use revolutionary::Revolutionary;
pub use sorcerer::Sorcerer;

#[derive(Clone)]
pub enum RoleHolder {
    King(King),
    Double(TheDouble),
    Prince(Prince),
    Sorcerer(Sorcerer),
    Knight(Knight),
    Revolutionary(Revolutionary),
}

impl RoleHolder {
    pub fn name(&self) -> RoleName {
        match &self {
            RoleHolder::King(r) => r.name(),
            RoleHolder::Double(r) => r.name(),
            RoleHolder::Prince(r) => r.name(),
            RoleHolder::Sorcerer(r) => r.name(),
            RoleHolder::Knight(r) => r.name(),
            RoleHolder::Revolutionary(r) => r.name(),
        }
    }

    pub fn win_condition_achieved(&self, block: &dyn crate::game::fsm::TimeBlock) -> bool {
        match &self {
            RoleHolder::King(r) => r.win_condition_achieved(block),
            RoleHolder::Double(r) => r.win_condition_achieved(block),
            RoleHolder::Prince(r) => r.win_condition_achieved(block),
            RoleHolder::Sorcerer(r) => r.win_condition_achieved(block),
            RoleHolder::Knight(r) => r.win_condition_achieved(block),
            RoleHolder::Revolutionary(r) => r.win_condition_achieved(block),
        }
    }
}

mod prelude {
    pub use super::{Role, RoleHolder, RoleName};
    pub use crate::game::{DeathCause, Game, Player};
}
