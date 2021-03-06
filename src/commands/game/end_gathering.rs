use super::prelude::*;

#[command("endgathering")]
#[only_in(guilds)]
#[description("Forcefully end a meeting")]
#[checks(StandardGameCheck)]
pub async fn end_gathering(ctx: &Context, msg: &Message) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let mut game = game_guard.write().await;

    if msg.author.id != game.host() {
        msg.reply(
            ctx,
            "You can't end a gathering in the meeting room if you're not the host.",
        )
        .await?;
        return Ok(());
    }

    if !game.can_end_gathering() {
        msg.reply(
            ctx,
            "You can't end a gathering in the big room if the current block isn't either the A block or the C block"
        )
        .await?;
        return Ok(());
    }

    *game = game.clone().transition_to_next_state(ctx).await;
    Ok(())
}
