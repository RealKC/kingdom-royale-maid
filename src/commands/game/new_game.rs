use super::prelude::*;
use serenity::model::id::{ChannelId, RoleId};
use std::env;

#[command("newgame")]
#[only_in(guilds)]
#[description("Creates a new game")]
pub async fn new_game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;

    let meeting_room = args.single::<ChannelId>();
    let meeting_room_id = meeting_room.unwrap_or(msg.channel_id);

    let player_role = args.single::<RoleId>();
    let player_role_id = player_role.unwrap_or(
        msg.guild(ctx)
            .await
            .unwrap()
            .role_by_name("The Guests")
            .unwrap_or(
                msg.guild(ctx)
                    .await
                    .unwrap()
                    .role_by_name("Players")
                    .unwrap(),
            )
            .id,
    );

    if data.get::<GameContainer>().is_some() {
        msg.reply(ctx, ": Cannot start a game if one is already running")
            .await?;
    } else {
        data.insert::<GameContainer>(Arc::new(RwLock::new(Game::new(
            msg.guild_id.unwrap(),
            msg.author.id,
            meeting_room_id,
            player_role_id,
        ))));
        msg.reply(
            ctx,
            format!(
                "has started a new game. You can join it by typing {}join",
                env::var("MAID_PREFIX").unwrap_or("!".to_owned())
            ),
        )
        .await?;
    }
    Ok(())
}
