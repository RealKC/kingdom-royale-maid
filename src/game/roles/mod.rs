mod role;

mod double;
mod king;
mod knight;
mod prince;
mod revolutionary;
mod sorcerer;

pub use role::{Role, RoleName};

pub use double::TheDouble;
pub use king::King;
pub use knight::Knight;
pub use prince::Prince;
pub use revolutionary::Revolutionary;
pub use sorcerer::Sorcerer;

mod prelude {
    pub use super::{Role, RoleName};
    pub use crate::game::{Game, GameState, KilledBy, Player};
}

mod common_actions {
    use super::prelude::*;
    use super::Role;

    // Used by King, TheDouble, and Prince
    pub fn king_like_action(this: &dyn Role, target: &mut Player, game: &mut Game) {
        assert!(this.can_do_special_action(game));
        assert!([RoleName::King, RoleName::TheDouble, RoleName::Prince].contains(&this.name()));

        game.set_king_murder_target(target);
    }

    // Used by Sorcerer, and Knight
    pub fn sorcerer_like_action(this: &dyn Role, target: &mut Player, game: &mut Game) {
        assert!(this.can_do_special_action(game));
        assert!([RoleName::Sorcerer, RoleName::Knight].contains(&this.name()));
        assert!(target.id() == game.king_murder_target_mut().id());

        game.kill(target.id(), this.name().into());
    }
}
