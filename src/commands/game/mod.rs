mod end_game;
mod game_info;
mod give_item;
mod join;
mod leave;
mod new_game;
mod roles;
mod rules;
mod stab;
mod start;
mod start_gathering;

pub use end_game::*;
pub use game_info::*;
pub use give_item::*;
pub use join::*;
pub use leave::*;
pub use new_game::*;
pub use roles::*;
pub use rules::*;
pub use stab::*;
pub use start::*;
pub use start_gathering::*;
pub struct GameContainer;

use super::prelude::*;
pub use crate::game::Game;

impl TypeMapKey for GameContainer {
    type Value = Arc<RwLock<Game>>;
}

mod prelude {
    pub use super::GameContainer;
    pub use crate::commands::prelude::*;
    pub use crate::game::Game;
}
