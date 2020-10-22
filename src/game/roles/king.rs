use super::prelude::*;

pub struct King;

impl Role for King {
    fn can_do_special_action(&self, game: &Game) -> bool {
        game.state() == GameState::CBlock
    }

    fn act(&self, _target: &mut Player) {
        unreachable!(
            "This should never be called, instead handled by Game::make_king_choose_target"
        );
    }

    fn name(&self) -> RoleName {
        RoleName::King
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_prince_alive() && !game.is_revolutionary_alive()
    }
}
