use super::prelude::*;
use crate::game::fsm::TimeBlock;

#[derive(Clone)]
pub struct Knight;

impl Role for Knight {
    fn name(&self) -> RoleName {
        RoleName::Knight
    }

    fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        !block.is_king_alive() && !block.is_the_double_alive()
    }

    fn wrap(self) -> RoleHolder {
        RoleHolder::Knight(self)
    }
}
