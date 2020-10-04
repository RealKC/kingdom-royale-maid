use super::common_actions::sorcerer_like_action;
use super::prelude::*;
pub struct Sorcerer;

impl Role for Sorcerer {
    fn can_do_special_action(&self, game: &Game) -> bool {
        game.state() == GameState::CBlock
    }

    fn act(&self, target: &mut Player, game: &mut Game) {
        sorcerer_like_action(self, target, game);
    }

    fn name(&self) -> RoleName {
        RoleName::Sorcerer
    }

    fn win_condition_achieved(&self, _: &Game) -> bool {
        true // As long as he's alive at the end of the game, he wins
    }
}
