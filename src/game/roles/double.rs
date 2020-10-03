use super::prelude::*;

pub struct TheDouble;

impl Role for TheDouble {
    fn can_do_special_action(&self, game: &Game) -> bool {
        todo!()
    }

    fn act(&self, game: &mut Game) {
        todo!()
    }

    fn name(&self) -> RoleName {
        todo!()
    }
}