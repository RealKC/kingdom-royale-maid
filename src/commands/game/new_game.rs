use crate::data::Prefix;

use super::prelude::*;
use serenity::model::id::{ChannelId, RoleId};

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
            .unwrap()
            .id,
    );

    let announcement_channel = args.single::<ChannelId>();
    let announcement_channel_id = announcement_channel.unwrap_or(msg.channel_id);

    if data.get::<GameContainer>().is_some() {
        msg.reply_err(
            ctx,
            ", you cannot start a game if one is already running".into(),
        )
        .await?;
    } else {
        data.insert::<GameContainer>(Arc::new(RwLock::new(Game::new(
            msg.guild_id.unwrap(),
            msg.author.id,
            meeting_room_id,
            announcement_channel_id,
            player_role_id,
        ))));
        msg.reply(
            ctx,
            format!(
                "has started a new game. You can join it by typing {}join",
                data.get::<Prefix>().unwrap()
            ),
        )
        .await?;
    }
    Ok(())
}
