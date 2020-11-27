use super::prelude::*;
use crate::game::GameState;

#[command("startgathering")]
#[only_in(guilds)]
#[description("Forcefully start a meeting")]
#[checks(StandardGameCheck)]
pub async fn start_gathering(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let mut game = expect_game_mut!(data);
    if msg.author.id != game.host() {
        msg.reply_err(
            ctx,
            "you can't start a gathering in the meeting room if you're not the host.".into(),
        )
        .await?;
        return Ok(());
    }

    if ![GameState::ABlock, GameState::CBlock].contains(&game.state()) {
        msg.reply_err(
                    ctx,
                    "you can't start a gathering in the big room if the current block isn't either the A block or the C block".into()
                ).await?;
        return Ok(());
    }

    game.transition_to_next_state(ctx).await?;

    Ok(())
}
