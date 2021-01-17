use crate::{
    commands::{help::*, *},
    data::{stats, Cdn, Prefix, Reqwest, ReqwestClient},
    hooks::*,
};
use serenity::{
    async_trait,
    client::{
        bridge::gateway::{GatewayIntents, ShardManager},
        ClientBuilder,
    },
    framework::standard::StandardFramework,
    http::Http,
    model::{
        gateway::Ready,
        id::{ChannelId, UserId},
    },
    prelude::*,
};
use std::{collections::HashSet, fs::File, io::Read, sync::Arc, time};
use tokio::sync::Mutex;
use tracing::info;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Bot {
    client: Client,
}

impl Bot {
    pub async fn new(
        token: String,
        prefix: String,
        cdn_channel_id: ChannelId,
        startup_time: time::Instant,
    ) -> Self {
        let http = Http::new_with_token(&token);
        let (owners, bot_id) = Self::application_info(&http).await;
        let framework = Self::new_framework(&prefix, bot_id, owners).await;
        let client = Self::new_client(&token, http, framework).await;

        let mut bot = Self { client };
        bot.initialise_data(cdn_channel_id, prefix, startup_time)
            .await;

        bot
    }

    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client.start().await
    }

    pub fn shard_manager(&self) -> Arc<Mutex<ShardManager>> {
        Arc::clone(&self.client.shard_manager)
    }

    async fn new_framework(
        prefix: &str,
        bot_id: UserId,
        owners: HashSet<UserId>,
    ) -> StandardFramework {
        StandardFramework::new()
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
            .group(&TESTS_GROUP)
            .group(&RANDOM_GROUP)
            .group(&META_GROUP)
            .group(&GAMEMANAGEMENT_GROUP)
            .group(&ITEMINTERACTIONS_GROUP)
            .group(&PLAYERINTERACTIONS_GROUP)
            .group(&GAMEINFORMATION_GROUP)
    }

    async fn new_client(token: &str, http: Http, framework: StandardFramework) -> Client {
        ClientBuilder::new_with_http(http)
            .token(token)
            .event_handler(Handler)
            .intents(
                GatewayIntents::GUILD_MEMBERS
                    | GatewayIntents::GUILD_MESSAGES
                    | GatewayIntents::GUILD_MESSAGE_REACTIONS
                    | GatewayIntents::GUILDS,
            )
            .framework(framework)
            .await
            .expect("Err creating client")
    }

    async fn application_info(http: &Http) -> (HashSet<UserId>, UserId) {
        match http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();
                if let Some(team) = info.team {
                    owners.insert(team.owner_user_id);
                    for member in team.members {
                        owners.insert(member.user.id);
                    }
                } else {
                    owners.insert(info.owner.id);
                }

                (owners, info.id)
            }
            Err(why) => panic!("Could not access application info: {:?}", why),
        }
    }

    async fn initialise_data(
        &mut self,
        cdn_channel_id: ChannelId,
        prefix: String,
        startup_time: time::Instant,
    ) {
        let reqwest_client = Reqwest::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:81.0) Gecko/20100101 Firefox/81.0")
            .build()
            .expect(
                "Could not create a Reqwest client, which is necessary for the bot to function.",
            );

        let mut data = self.client.data.write().await;

        #[cfg(target_os = "linux")]
        if let Ok(version) = read_system_version() {
            data.insert::<stats::SystemVersion>(version);
        }

        data.insert::<stats::CommandStatisticsContainer>(Default::default());
        data.insert::<stats::StartupTime>(startup_time);
        data.insert::<ShardManagerContainer>(Arc::clone(&self.client.shard_manager));
        data.insert::<ReqwestClient>(reqwest_client);
        data.insert::<Cdn>(cdn_channel_id);
        data.insert::<Prefix>(prefix);
    }
}

#[cfg(target_os = "linux")]
fn read_system_version() -> Result<String, std::io::Error> {
    let mut f = File::open("/proc/version")?;
    let mut version = String::new();

    f.read_to_string(&mut version)?;

    Ok(version)
}
