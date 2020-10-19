//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
use serenity::{
    async_trait, client::bridge::gateway::ShardManager, framework::standard::CommandResult,
    framework::standard::StandardFramework, http::Http, model::gateway::Ready,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

use serenity::prelude::*;
use tokio::sync::Mutex;
use tracing::{error, info, instrument};

mod commands;
mod hooks;
use commands::{help::*, *};
use hooks::*;

mod data;
mod game;

use crate::data::{Cdn, Reqwest, ReqwestClient};

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
#[instrument]
async fn main() -> CommandResult {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token & prefix in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let prefix = env::var("MAID_PREFIX").unwrap_or("!".into());
    let cdn_channel_id = env::var("MAID_CDN_CHANNEL_ID").expect("Give me my discord cdn pl0x");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix(&prefix)
                .delimiters(vec![" "])
                // Sets the bot's owners. These will be used for commands that
                // are owners only.
                .owners(owners)
        })
        .before(before)
        .after(after)
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&META_GROUP)
        .group(&RANDOM_GROUP)
        .group(&GAME_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    let reqwest_client = Reqwest::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:81.0) Gecko/20100101 Firefox/81.0")
        .build()?;

    {
        use serenity::model::id::ChannelId;

        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestClient>(reqwest_client);
        data.insert::<Cdn>(ChannelId(str::parse::<u64>(&cdn_channel_id)?));
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
