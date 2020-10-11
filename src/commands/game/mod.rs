mod end_game;
mod game_info;
mod join;
mod leave;
mod new_game;
mod roles;
mod rules;
mod start;
mod start_gathering;

pub use end_game::*;
pub use game_info::*;
pub use join::*;
pub use leave::*;
pub use new_game::*;
pub use roles::*;
pub use rules::*;
pub use start::*;
pub use start_gathering::*;

mod prelude {
    pub use crate::commands::prelude::*;
    pub use crate::game::Game;

    pub struct GameContainer;

    impl TypeMapKey for GameContainer {
        type Value = Arc<RwLock<Game>>;
    }
}
