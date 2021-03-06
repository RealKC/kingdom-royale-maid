use super::prelude::*;

use tracing::info;

#[command]
#[only_in(guilds)]
#[bucket = "join_leave_ratelimit_bucket"]
#[description("Allows you to leave a game")]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let game = data.get_mut::<GameContainer>();

    if game.is_some() {
        let mut game = game.unwrap().write().await;
        if game.is_started() {
            msg.reply(ctx, "You can't leave a game that has started!")
                .await?;
            info!("User tried leaving running game");
            return Ok(());
        } else {
            let result = game.leave(msg.author.id);
            if result.is_ok() {
                info!("A user successfully left a game");
                msg.reply(ctx, "You've successfully left the game :c")
                    .await?;
            } else {
                info!("User couldn't leave, error is {:?}", result.unwrap_err());
                msg.reply(ctx, format!("{}", result.unwrap_err())).await?;
            }
        }
    } else {
        msg.reply(ctx, "You can't leave a game if you aren't in one!")
            .await?;
    }
    Ok(())
}
