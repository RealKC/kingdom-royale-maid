use super::prelude::*;
use tracing::{error, info, instrument};

#[command]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let mut game = data.get_mut::<GameContainer>();

    if game.is_some() {
        let mut game = game.unwrap().write().await;
        let result = game.join(msg.author.id);
        if result.is_ok() {
            info!("Successfully added a new user to the game");
            msg.reply(
                ctx,
                format!(
                    ", you've joined {}'s Kingdom Royale game.",
                    game.host().to_user(ctx).await?
                ),
            )
            .await?;
        } else {
            info!("Couldn't add new user, error is {:?}", result.unwrap_err());
            msg.reply(ctx, format!("{}", result.unwrap_err())).await?;
        }
    } else {
        info!("User tried joining nonexistent user");
        msg.reply(
            ctx,
            ", you can't join a game if there aren't any in progress",
        )
        .await?;
    }
    Ok(())
}
