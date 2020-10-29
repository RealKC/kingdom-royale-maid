use super::prelude::*;

pub struct TheDouble;

impl Role for TheDouble {
    fn name(&self) -> RoleName {
        RoleName::TheDouble
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_prince_alive() && !game.is_revolutionary_alive()
    }
}
