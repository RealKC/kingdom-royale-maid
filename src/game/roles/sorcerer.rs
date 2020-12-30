use super::prelude::*;
use crate::game::fsm::TimeBlock;

#[derive(Clone)]
pub struct Sorcerer;

impl Role for Sorcerer {
    fn name(&self) -> RoleName {
        RoleName::Sorcerer
    }

    fn win_condition_achieved(&self, _: &dyn TimeBlock) -> bool {
        true // As long as he's alive at the end of the game, he wins
    }

    fn wrap(self) -> RoleHolder {
        RoleHolder::Sorcerer(self)
    }
}
