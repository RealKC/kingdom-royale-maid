use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use std::{collections::HashMap, fmt::Write};

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[command]
#[owners_only]
pub async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data
        .get::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name = k, amount = v);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &contents).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
