use super::common_actions::king_like_action;
use super::prelude::*;

pub struct King;

impl Role for King {
    fn can_do_special_action(&self, game: &Game) -> bool {
        game.state() == GameState::CBlock
    }

    fn act(&self, target: &mut Player, game: &mut Game) {
        king_like_action(self, target, game);
    }

    fn name(&self) -> RoleName {
        RoleName::King
    }
}
