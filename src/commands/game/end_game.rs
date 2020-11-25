use super::prelude::*;

use typemap_rev::Entry;

#[command("endgame")]
#[only_in(guilds)]
#[description("Forcefully end a game")]
pub async fn end_game(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;

    {
        let game = data.get::<GameContainer>();
        match game {
            Some(game) => {
                let mut game = game.write().await;
                game.end(ctx).await?;
            }
            None => {
                msg.reply_err(
                    ctx,
                    "you can't end a game if there isn't one running".into(),
                )
                .await?;
                return Ok(());
            }
        }
    }

    let game_entry = data.entry::<GameContainer>();
    match game_entry {
        Entry::Occupied(game_container) => {
            game_container.remove();
        }
        Entry::Vacant(_) => {
            // Don't think this should ever happen, but to be safe
            return Ok(());
        }
    }

    Ok(())
}
