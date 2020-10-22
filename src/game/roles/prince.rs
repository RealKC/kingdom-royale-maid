use super::prelude::*;

pub struct Prince;

impl Role for Prince {
    fn can_do_special_action(&self, game: &Game) -> bool {
        !game.is_king_alive() && !game.is_the_double_alive()
    }

    fn act(&self, _target: &mut Player) {
        unreachable!(
            "This should never be called, instead handled by Game::make_king_choose_target"
        );
    }

    fn name(&self) -> RoleName {
        RoleName::Prince
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_king_alive() && !game.is_the_double_alive() && !game.is_revolutionary_alive()
    }
}
