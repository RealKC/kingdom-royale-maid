use super::prelude::*;

use tracing::info;

#[command]
#[only_in(guilds)]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let mut game = data.get_mut::<GameContainer>();

    if game.is_some() {
        let mut game = game.unwrap().write().await;
        let result = game.leave(msg.author.id);
        if result.is_ok() {
            info!("A user succesfully left a game");
            msg.reply(ctx, ", you've successfully left the game :c")
                .await?;
        } else {
            info!("User couldn't leave, error is {:?}", result.unwrap_err());
            msg.reply(ctx, format!("{}", result.unwrap_err())).await?;
        }
    } else {
        msg.reply(ctx, ", you can't leave a game if you aren't in one!")
            .await?;
    }
    Ok(())
}
