use super::prelude::*;
use crate::game::GameState;

#[command("nextblock")]
#[description("Forcefully go to the next time block")]
pub async fn next_block(ctx: &Context, msg: &Message) -> CommandResult {
    let game = ctx.data.read().await;
    let game = game.get::<GameContainer>();
    match game {
        Some(game) => {
            let mut game = game.write().await;
            if msg.author.id != game.host() {
                msg.reply(
                    ctx,
                    ", you can't go to the next time block if you're not the host.",
                )
                .await?;
                return Ok(());
            }

            if game.state() == GameState::NotStarted {
                msg.reply(
                    ctx,
                    ", you can't go to the next time block if the game hasn't started yet",
                )
                .await?;
                return Ok(());
            }

            game.transition_to_next_state(ctx).await?;
        }
        None => {
            msg.reply(
                ctx,
                ", you can't go to the next time block if there's no game running!",
            )
            .await?;
        }
    };
    Ok(())
}
