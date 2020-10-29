use super::prelude::*;

pub struct Revolutionary;

impl Role for Revolutionary {
    fn can_do_special_action(&self, game: &Game) -> bool {
        game.state() == GameState::EBlock
    }

    fn name(&self) -> RoleName {
        RoleName::Revolutionary
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_king_alive() && !game.is_the_double_alive() && !game.is_prince_alive()
    }
}
