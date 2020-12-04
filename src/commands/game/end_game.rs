use super::prelude::*;

use typemap_rev::Entry;

#[command("endgame")]
#[only_in(guilds)]
#[description("Forcefully end a game")]
pub async fn end_game(ctx: &Context, msg: &Message) -> CommandResult {
    end_game_impl(ctx, msg, false).await
}

pub async fn end_game_impl(ctx: &Context, msg: &Message, in_shutdown: bool) -> CommandResult {
    let mut data = ctx.data.write().await;

    {
        let game = data.get::<GameContainer>();
        match game {
            Some(game) => {
                let mut game = game.write().await;
                game.end(ctx).await?;
            }
            None => {
                if !in_shutdown {
                    msg.reply(ctx, "You can't end a game if there isn't one running")
                        .await?;
                }
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
            // This can happen when in_shutdown == true
            return Ok(());
        }
    }

    Ok(())
}
