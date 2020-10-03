use super::common_actions::sorcerer_like_action;
use crate::game::roles::prelude::*;

pub struct Knight;

impl Role for Knight {
    fn name(&self) -> RoleName {
        RoleName::Knight
    }

    fn can_do_special_action(&self, game: &Game) -> bool {
        !game.is_sorcerer_alive() && game.state() == GameState::CBlock
    }

    fn act(&self, target: &mut Player, game: &mut Game) {
        sorcerer_like_action(self, target, game);
    }
}
