use super::prelude::*;
use crate::game::fsm::TimeBlock;

#[derive(Clone)]
pub struct Revolutionary;

impl Role for Revolutionary {
    fn name(&self) -> RoleName {
        RoleName::Revolutionary
    }

    fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        !block.is_king_alive() && !block.is_the_double_alive() && !block.is_prince_alive()
    }

    fn wrap(self) -> RoleHolder {
        RoleHolder::Revolutionary(self)
    }
}
