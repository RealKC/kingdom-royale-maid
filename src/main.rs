use data::Prefix;
use serenity::{
    async_trait, client::bridge::gateway::GatewayIntents, client::bridge::gateway::ShardManager,
    framework::standard::CommandResult, framework::standard::StandardFramework, http::Http,
    model::gateway::Ready,
};
use std::{
    collections::{HashMap, HashSet},
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
mod helpers;

use crate::data::{Cdn, Reqwest, ReqwestClient};

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

    let (token, prefix, cdn_channel_id) = get_env_config();

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

    let mut framework = StandardFramework::new()
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
        // Ratelimit !join and !leave to 2 invocations, every 45 secs, with a 5 second delay between the two
        .bucket("join_leave_ratelimit_bucket", |b| {
            b.delay(5).time_span(45).limit(2)
        })
        .await
        .help(&MY_HELP)
        .group(&META_GROUP)
        .group(&RANDOM_GROUP)
        .group(&TESTS_GROUP);

    for group in GAME_GROUP.options.sub_groups {
        framework = framework.group(group);
    }

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .intents(
            GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::GUILDS,
        )
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
        data.insert::<Prefix>(prefix);
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}

fn get_env_config() -> (String, String, String) {
    dotenv::dotenv().expect("Encountered an error that didn't allow parsing the .env file");

    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let prefix = dotenv::var("MAID_PREFIX").unwrap_or("!".into());
    let cdn_channel_id = dotenv::var("MAID_CDN_CHANNEL_ID").expect("Give me my discord cdn pl0x");

    (token, prefix, cdn_channel_id)
}
