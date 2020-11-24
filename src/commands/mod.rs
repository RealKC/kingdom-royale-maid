mod delete_category;
pub mod game;
pub mod help;
pub mod meta;
pub mod random;
pub mod stats;
mod test_cmds;
mod tos;

use crate::commands::meta::*;
use delete_category::*;
use game::*;
use random::*;
use stats::*;
use test_cmds::*;
use tos::*;

use serenity::{framework::standard::macros::group, prelude::*};
use std::collections::HashMap;

#[group]
#[only_in(guilds)]
#[commands(about, tos)]
pub struct Meta;

#[group]
#[commands(say, commands, delete_category)]
pub struct Random;

#[group]
#[only_in(guilds)]
#[sub_groups(GameManagement, ItemInteractions, PlayerInteractions, GameInformation)]
pub struct Game;

#[group("Game Management")]
#[only_in(guilds)]
#[commands(
    join,
    leave,
    new_game,
    end_game,
    start,
    start_gathering,
    end_gathering,
    next_block
)]
pub struct GameManagement;

#[group("Item Interactions")]
#[only_in(guilds)]
#[commands(give_item, inventory, notes, write_note, show_note, rip_note)]
pub struct ItemInteractions;

#[group("Player Interactions")]
#[only_in(guilds)]
#[commands(substitute, stab)]
pub struct PlayerInteractions;

#[group("Game Information")]
#[only_in(guilds)]
#[commands(game_info, roles, role_info, rules, info)]
pub struct GameInformation;

#[group]
#[commands(king_test, rev_test, confirm_murder, forceadd)]
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
