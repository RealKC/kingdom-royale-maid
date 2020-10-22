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
