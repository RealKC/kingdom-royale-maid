// Command modules
mod end_game;
mod end_gathering;
mod flavour;
mod forceadd;
mod game_info;
mod give_item;
mod info;
mod inventory;
mod join;
mod leave;
mod new_game;
mod next_block;
mod notes;
mod roles;
mod secret_meeting_log;
mod stab;
mod start;
mod start_gathering;
mod substitute;

pub use end_game::*;
pub use end_gathering::*;
pub use flavour::*;
pub use forceadd::*;
pub use game_info::*;
pub use give_item::*;
pub use info::*;
pub use inventory::*;
pub use join::*;
pub use leave::*;
pub use new_game::*;
pub use next_block::*;
pub use notes::*;
pub use roles::*;
pub use secret_meeting_log::*;
pub use stab::*;
pub use start::*;
pub use start_gathering::*;
pub use substitute::*;

mod checks;
#[macro_use]
mod macros;

use super::prelude::*;
pub use crate::game::Game;

pub struct GameContainer;

impl TypeMapKey for GameContainer {
    type Value = Arc<RwLock<Game>>;
}

mod prelude {
    pub use super::{checks::*, GameContainer};
    pub use crate::{
        commands::prelude::*, expect_game, expect_game_mut, expect_player, game::Game,
    };
}
