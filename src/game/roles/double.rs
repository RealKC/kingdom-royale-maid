use super::prelude::*;

pub struct TheDouble;

impl Role for TheDouble {
    fn can_do_special_action(&self, game: &Game) -> bool {
        !game.is_king_alive() && game.state() == GameState::CBlock
    }

    fn name(&self) -> RoleName {
        RoleName::TheDouble
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_prince_alive() && !game.is_revolutionary_alive()
    }
}
