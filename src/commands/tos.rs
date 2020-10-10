use super::prelude::*;

#[command]
pub async fn tos(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, r#"
By using this bot you agree for your Discord user ID to be stored in the bot's RAM for the duration of a game.

Messages are never stored on disk.
    "#).await
}
