use super::prelude::*;
use crate::game::fsm::TimeBlock;

#[derive(Clone)]
pub struct TheDouble;

impl Role for TheDouble {
    fn name(&self) -> RoleName {
        RoleName::TheDouble
    }

    fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        !block.is_prince_alive() && !block.is_revolutionary_alive()
    }

    fn wrap(self) -> RoleHolder {
        RoleHolder::Double(self)
    }
}
