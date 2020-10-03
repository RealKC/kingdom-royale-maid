use crate::game::Game;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoleName {
    King,
    Prince,
    TheDouble,
    Sorcerer,
    Knight,
    Revolutionary
}

pub trait Role {
    fn can_do_special_action(&self, game: &Game) -> bool;
    fn act(&self, game: &mut Game);
    fn name(&self) -> RoleName;
}
