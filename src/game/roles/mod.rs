mod role;

mod king;
mod prince;
mod double;
mod sorcerer;
mod knight;
mod revolutionary;

pub use role::{Role, RoleName};

pub use king::King;
pub use prince::Prince;
pub use double::TheDouble;
pub use sorcerer::Sorcerer;
pub use knight::Knight;
pub use revolutionary::Revolutionary;

mod prelude {
    pub use super::{Role, RoleName};
    pub use crate::game::{Game, GameState};
}
