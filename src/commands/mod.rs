pub mod game;
pub mod help;
pub mod meta;
pub mod random;
pub mod stats;
mod test_cmds;
mod tos;

use crate::commands::meta::*;
use game::*;
use random::*;
use stats::*;
use test_cmds::*;
use tos::*;

use serenity::framework::standard::macros::group;
use serenity::prelude::*;
use std::collections::HashMap;

#[group]
#[only_in(guilds)]
#[commands(about, tos)]
pub struct Meta;

#[group]
#[commands(say, commands)]
pub struct Random;

#[group]
#[only_in(guilds)]
#[commands(
    new_game,
    end_game,
    join,
    leave,
    game_info,
    start,
    start_gathering,
    end_gathering,
    next_block,
    give_item,
    substitute,
    stab,
    roles,
    role_info,
    rules,
    info,
    inventory
)]
pub struct Game;

#[group]
#[commands(king_test, rev_test, confirm_murder)]
pub struct Tests;

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

mod prelude {
    pub use serenity::framework::standard::{macros::command, Args, CommandResult};
    pub use serenity::model::channel::Message;
    pub use serenity::prelude::*;

    pub use std::sync::Arc;
    pub use tokio::sync::RwLock;
}
