use super::prelude::*;

#[command("nextblock")]
#[only_in(guilds)]
#[description("Forcefully go to the next time block")]
#[checks(StandardGameCheck)]
pub async fn next_block(ctx: &Context, msg: &Message) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let mut game = game_guard.write().await;

    if msg.author.id != game.host() {
        msg.reply(
            ctx,
            "You can't go to the next time block if you're not the host.",
        )
        .await?;
        return Ok(());
    }

    game.transition_to_next_state(ctx).await?;
    msg.channel_id
        .say(ctx, format!("☑️ New time block is {}", game.state()))
        .await?;

    Ok(())
}
