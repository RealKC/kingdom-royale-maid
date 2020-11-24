use super::prelude::*;

pub struct Knight;

impl Role for Knight {
    fn name(&self) -> RoleName {
        RoleName::Knight
    }

    fn win_condition_achieved(&self, game: &Game) -> bool {
        !game.is_king_alive() && !game.is_the_double_alive()
    }
}
