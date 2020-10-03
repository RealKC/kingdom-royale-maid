use crate::game::roles::prelude::*;

pub struct Knight;

impl Role for Knight {
    fn name(&self) -> RoleName {
        RoleName::Knight
    }

    fn can_do_special_action(&self, game: &Game) -> bool {
        let right_block = match game.state() {
            GameState::CBlock => true,
            _ => false
        };

        right_block && !game.is_sorcerer_alive()
    }

    fn act(&self, game: &mut crate::game::Game) {
        todo!()
    }
}