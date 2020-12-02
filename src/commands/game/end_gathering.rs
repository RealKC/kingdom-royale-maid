use super::prelude::*;
use crate::game::GameState;

#[command("endgathering")]
#[only_in(guilds)]
#[description("Forcefully end a meeting")]
#[checks(StandardGameCheck)]
pub async fn end_gathering(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let mut game = expect_game_mut!(data);

    if msg.author.id != game.host() {
        msg.reply(
            ctx,
            "You can't end a gathering in the meeting room if you're not the host.",
        )
        .await?;
        return Ok(());
    }

    if ![GameState::BBlock, GameState::DBlock].contains(&game.state()) {
        msg.reply(
            ctx,
            "You can't end a gathering in the big room if the current block isn't either the A block or the C block"
        )
        .await?;
        return Ok(());
    }

    game.transition_to_next_state(ctx).await?;
    Ok(())
}
