use super::prelude::*;

pub struct King;

impl Role for King {
    fn name(&self) -> RoleName {
        RoleName::King
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_prince_alive() && !game.is_revolutionary_alive()
    }
}
