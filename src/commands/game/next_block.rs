use super::prelude::*;

#[command("nextblock")]
#[only_in(guilds)]
#[description("Forcefully go to the next time block")]
#[checks(StandardGameCheck)]
pub async fn next_block(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let mut game = expect_game_mut!(data);

    if msg.author.id != game.host() {
        msg.reply_err(
            ctx,
            "you can't go to the next time block if you're not the host.".into(),
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
