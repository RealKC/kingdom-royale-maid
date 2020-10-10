use super::prelude::*;

#[command]
#[only_in(guilds)]
#[description("Starts a game if it has 6 players in it")]
pub async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let game = ctx.data.write().await;
    let game = game.get::<GameContainer>();

    match game {
        Some(game) => {
            let mut game = game.write().await;
            if msg.author.id != game.host() {
                msg.reply(ctx, ", you can't start a game that you aren't the host of.")
                    .await
                    .map(|_| ())?;
            } else {
                if game.can_start() {
                    msg.channel_id
                        .say(ctx, "Starting the game...")
                        .await
                        .map(|_| ())?;

                    game.start(ctx).await?;
                } else {
                    msg.channel_id
                        .say(ctx, "You can't start a game if there's less than 6 players")
                        .await?;
                }
            }
        }
        None => msg
            .reply(ctx, ", you can't start a game if there isn't one running!")
            .await
            .map(|_| ())?,
    }

    Ok(())
}
