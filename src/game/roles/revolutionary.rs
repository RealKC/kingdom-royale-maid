use super::prelude::*;

pub struct Revolutionary;

impl Role for Revolutionary {
    fn can_do_special_action(&self, game: &Game) -> bool {
        game.state() == GameState::EBlock
    }

    fn act(&self, target: &mut Player, game: &mut Game) {
        assert!(self.can_do_special_action(game));

        game.kill(target.id(), self.name().into());
    }

    fn name(&self) -> RoleName {
        RoleName::Revolutionary
    }
}
