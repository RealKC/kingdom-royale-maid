use super::prelude::*;
use crate::game::fsm::TimeBlock;

#[derive(Clone)]
pub struct Prince;

impl Role for Prince {
    fn name(&self) -> RoleName {
        RoleName::Prince
    }

    fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        !block.is_king_alive() && !block.is_the_double_alive() && !block.is_revolutionary_alive()
    }

    fn wrap(self) -> RoleHolder {
        RoleHolder::Prince(self)
    }
}
