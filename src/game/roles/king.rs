use super::prelude::*;

pub struct King;

impl Role for King {
    fn can_do_special_action(&self, game: &Game) -> bool {
        match game.state() {
            GameState::CBlock => true,
            _ => false
        }
    }

    fn act(&self, game: &mut Game) {
        todo!()
    }

    fn name(&self) -> RoleName {
        RoleName::King
    }
}