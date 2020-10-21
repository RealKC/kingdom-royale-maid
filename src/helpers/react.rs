use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, ReactionType};
use serenity::prelude::*;

pub async fn react_with(ctx: &Context, msg: &Message, unicodes: &[&str]) -> CommandResult {
    for emoji in unicodes {
        msg.react(ctx, ReactionType::Unicode(emoji.to_string()))
            .await?;
    }

    Ok(())
}
