use super::prelude::*;
use crate::game::fsm::TimeBlock;

#[derive(Clone)]
pub struct King;

impl Role for King {
    fn name(&self) -> RoleName {
        RoleName::King
    }

    fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        !block.is_prince_alive() && !block.is_revolutionary_alive()
    }

    fn wrap(self) -> RoleHolder {
        RoleHolder::King(self)
    }
}
