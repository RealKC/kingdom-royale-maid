mod game_info;
mod join;
mod leave;
mod new_game;
mod roles;
mod rules;
mod start;

pub use game_info::*;
pub use join::*;
pub use leave::*;
pub use new_game::*;
pub use roles::*;
pub use rules::*;
pub use start::*;

mod prelude {
    pub use crate::commands::prelude::*;
    pub use crate::game::Game;

    pub struct GameContainer;

    impl TypeMapKey for GameContainer {
        type Value = Arc<RwLock<Game>>;
    }
}
