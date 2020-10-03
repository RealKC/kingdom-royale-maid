use super::prelude::*;

pub struct Sorcerer;

impl Role for Sorcerer {
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