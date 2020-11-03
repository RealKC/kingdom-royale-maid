use crate::game::GameState;

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
        if game.state() != GameState::NotStarted || game.state() != GameState::GameEnded {
            msg.reply_err(ctx, "you can't leave a game that has started!".into())
                .await?;
            info!("User tried leaving running game");
            return Ok(());
        } else {
            let result = game.leave(msg.author.id);
            if result.is_ok() {
                info!("A user succesfully left a game");
                msg.reply_err(ctx, "you've successfully left the game :c".into())
                    .await?;
            } else {
                info!("User couldn't leave, error is {:?}", result.unwrap_err());
                msg.reply(ctx, format!("{}", result.unwrap_err())).await?;
            }
        }
    } else {
        msg.reply_err(ctx, "you can't leave a game if you aren't in one!".into())
            .await?;
    }
    Ok(())
}
