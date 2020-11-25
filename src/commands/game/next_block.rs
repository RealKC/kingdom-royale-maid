use super::prelude::*;
use crate::game::GameState;

#[command("nextblock")]
#[only_in(guilds)]
#[description("Forcefully go to the next time block")]
pub async fn next_block(ctx: &Context, msg: &Message) -> CommandResult {
    let game = ctx.data.read().await;
    let game = game.get::<GameContainer>();
    match game {
        Some(game) => {
            let mut game = game.write().await;
            if msg.author.id != game.host() {
                msg.reply_err(
                    ctx,
                    "you can't go to the next time block if you're not the host.".into(),
                )
                .await?;
                return Ok(());
            }

            if game.state() == GameState::NotStarted {
                msg.reply_err(
                    ctx,
                    "you can't go to the next time block if the game hasn't started yet.".into(),
                )
                .await?;
                return Ok(());
            }

            game.transition_to_next_state(ctx).await?;
            msg.channel_id
                .say(ctx, format!("☑️ New time block is {}", game.state()))
                .await?;
        }
        None => {
            msg.reply_err(
                ctx,
                "you can't go to the next time block if there's no game running!".into(),
            )
            .await?;
        }
    };
    Ok(())
}
