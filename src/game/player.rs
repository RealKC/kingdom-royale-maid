use crate::game::role::Role;
use crate::game::Game;
use serenity::model::id::UserId;

pub struct Player {
    id: UserId,
    role: Box<dyn Role>,
    alive: bool,
}

impl Player {
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn can_act(&self, game: &Game) -> bool {
        self.role.can_act(game)
    }

    fn act(&self, game: &mut Game) {
        self.role.act(game)
    }
}
