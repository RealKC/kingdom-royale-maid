use crate::game::roles::{Role, RoleName};
use crate::game::Game;
use serenity::model::id::UserId;

pub struct Player {
    id: UserId,
    role: Box<dyn Role>,
    alive: bool,
}

impl Player {
    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_dead(&mut self) {
        self.alive = false;

        // TODO: Discord related stuffs
    }

    pub fn can_do_special_action(&self, game: &Game) -> bool {
        self.role.can_do_special_action(game)
    }

    pub fn act(&self, target: &mut Player, game: &mut Game) {
        self.role.act(target, game)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }
}
