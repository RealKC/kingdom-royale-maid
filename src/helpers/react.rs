use serenity::{
    framework::standard::CommandResult,
    model::channel::{Message, ReactionType},
    prelude::*,
};

pub async fn react_with(ctx: &Context, msg: &Message, unicodes: &[&str]) -> CommandResult {
    for emoji in unicodes {
        msg.react(ctx, ReactionType::Unicode(emoji.to_string()))
            .await?;
    }

    Ok(())
}
