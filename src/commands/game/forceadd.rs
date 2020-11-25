use super::prelude::*;

use serenity::model::id::UserId;
use tracing::info;

#[command]
#[owners_only]
#[only_in(guilds)]
pub async fn forceadd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("{:?}", msg);

    let data = ctx.data.write().await;
    let game = {
        let game = data.get::<GameContainer>().unwrap();

        &mut game.write().await
    };

    for user in args.iter::<UserId>() {
        game.join(user?)?;
    }

    Ok(())
}
