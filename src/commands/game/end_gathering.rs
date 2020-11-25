use super::prelude::*;
use crate::game::GameState;

#[command("endgathering")]
#[only_in(guilds)]
#[description("Forcefully end a meeting")]
pub async fn end_gathering(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let game = data.get::<GameContainer>();
    if game.is_none() {
        msg.reply_err(
            ctx,
            "you can't end a gathering if there's no game running!".into(),
        )
        .await?;
        return Ok(());
    }
    let game = game.unwrap();
    let mut game = game.write().await;

    if msg.author.id != game.host() {
        msg.reply_err(
            ctx,
            "you can't end a gathering in the meeting room if you're not the host.".into(),
        )
        .await?;
        return Ok(());
    }

    if game.state() == GameState::NotStarted {
        msg.reply_err(
            ctx,
            "you can't end a meeting in the big room if the game hasn't started yet".into(),
        )
        .await?;
        return Ok(());
    }

    if ![GameState::BBlock, GameState::DBlock].contains(&game.state()) {
        msg.reply_err(
            ctx,
              "you can't end a gathering in the big room if the current block isn't either the A block or the C block".into()
        )
        .await?;
        return Ok(());
    }

    game.transition_to_next_state(ctx).await?;
    Ok(())
}
