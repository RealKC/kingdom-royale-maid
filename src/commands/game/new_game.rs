use super::prelude::*;

use std::env;

#[command("newgame")]
pub async fn new_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;

    if data.get::<GameContainer>().is_some() {
        msg.reply(ctx, ": Cannot start a game if one is already running")
            .await?;
    } else {
        data.insert::<GameContainer>(Arc::new(RwLock::new(Game::new(
            msg.guild_id.unwrap(),
            msg.author.id,
        ))));
        msg.reply(
            ctx,
            format!(
                "has started a new game. You can join it by typing {}join",
                env::var("MAID_PREFIX").unwrap_or("!".to_owned())
            ),
        )
        .await?;
    }
    Ok(())
}
