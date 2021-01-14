use bot::Bot;
use serenity::{
    client::bridge::gateway::ShardManager, framework::standard::CommandResult, model::id::ChannelId,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, instrument};

mod bot;
mod commands;
mod data;
mod game;
mod helpers;
mod hooks;
mod resources;
#[cfg(not(feature = "deterministic"))]
mod version_data;

#[tokio::main]
#[instrument]
async fn main() -> CommandResult {
    tracing_subscriber::fmt::init();

    let startup_time = std::time::Instant::now();

    let (token, prefix, cdn_channel_id) = get_env_config();

    let mut bot = Bot::new(
        token,
        prefix,
        ChannelId(cdn_channel_id.parse::<u64>()?),
        startup_time,
    )
    .await;

    setup_signals(bot.shard_manager()).await;

    if let Err(why) = bot.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}

fn get_env_config() -> (String, String, String) {
    dotenv::dotenv().expect("Encountered an error that didn't allow parsing the .env file");

    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let prefix = dotenv::var("MAID_PREFIX").unwrap_or_else(|_| "!".into());
    let cdn_channel_id = dotenv::var("MAID_CDN_CHANNEL_ID").expect("Give me my discord cdn pl0x");

    (token, prefix, cdn_channel_id)
}

async fn setup_signals(shard_manager: Arc<Mutex<ShardManager>>) {
    // Listen to interrupts
    // Thanks Prof Bloodstone from the serenity discord
    #[cfg(not(windows))]
    {
        use tokio::{signal::unix::signal, signal::unix::SignalKind};

        let signals_to_handle = vec![
            SignalKind::hangup(),
            SignalKind::interrupt(),
            SignalKind::terminate(),
        ];
        for kind in signals_to_handle {
            let mut stream = signal(kind).unwrap();
            let shard_manager = shard_manager.clone();
            tokio::spawn(async move {
                stream.recv().await;
                info!("Signal received - shutting down!");
                shard_manager.lock().await.shutdown_all().await;
            });
        }
    }

    #[cfg(windows)]
    {
        use tokio::signal::windows::{ctrl_break, ctrl_c};

        let shard_manager_clone = Arc::clone(shard_manager);

        tokio::spawn(async move {
            ctrl_break().unwrap().recv().await;
            info!("Ctrl Break received - shutting down!");
            shard_manager_clone.lock().await.shutdown_all().await;
        });

        tokio::spawn(async move {
            ctrl_c().unwrap().recv().await;
            info!("Ctrl C received - shutting down!");
            shard_manager.lock().await.shutdown_all().await;
        });
    }
}
