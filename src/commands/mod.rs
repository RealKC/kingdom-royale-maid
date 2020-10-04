pub mod game;
pub mod help;
pub mod meta;
pub mod random;
pub mod stats;

mod prelude {
    pub use serenity::framework::standard::{macros::command, Args, CommandResult};
    pub use serenity::model::channel::Message;
    pub use serenity::prelude::*;

    pub use std::sync::Arc;
    pub use tokio::sync::RwLock;
}
