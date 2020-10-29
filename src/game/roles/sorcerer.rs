use super::prelude::*;

pub struct Sorcerer;

impl Role for Sorcerer {
    fn name(&self) -> RoleName {
        RoleName::Sorcerer
    }

    fn win_condition_achieved(&self, _: &Game) -> bool {
        true // As long as he's alive at the end of the game, he wins
    }
}
