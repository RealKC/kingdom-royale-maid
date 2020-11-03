mod end_game;
mod end_gathering;
mod game_info;
mod give_item;
mod info;
mod inventory;
mod join;
mod leave;
mod new_game;
mod next_block;
mod roles;
mod stab;
mod start;
mod start_gathering;
mod substitute;

pub use end_game::*;
pub use end_gathering::*;
pub use game_info::*;
pub use give_item::*;
pub use info::*;
pub use inventory::*;
pub use join::*;
pub use leave::*;
pub use new_game::*;
pub use next_block::*;
pub use roles::*;
pub use stab::*;
pub use start::*;
pub use start_gathering::*;
pub use substitute::*;
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
    pub use crate::helpers::serenity_ext::MaidReply;
}
