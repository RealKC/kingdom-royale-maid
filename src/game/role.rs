use super::game::Game;

pub trait Role {
    fn can_act(&self, game: &Game) -> bool;
    fn act(&self, game: &mut Game);
}
