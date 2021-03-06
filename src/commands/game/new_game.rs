use super::prelude::*;
use crate::data::Prefix;

use serenity::model::id::{ChannelId, RoleId};

#[command("newgame")]
#[only_in(guilds)]
#[description("Creates a new game")]
pub async fn new_game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // So, in order to avoid a game being created during our argument parsing[0], we
    // hold a writer lock during our argument parsing to avoid such a race condition.
    //
    // [0] We want to avoid a game being created during argument parsing, so the earliest
    //     person wanting to make a new game, will actually be the one creating a game.
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

    let delete_rooms_category_on_game_end = args.single::<bool>().unwrap_or(true);

    if data.get::<GameContainer>().is_some() {
        msg.reply(ctx, "You cannot start a game if one is already running")
            .await?;
    } else {
        data.insert::<GameContainer>(Arc::new(RwLock::new(Game::new(
            msg.guild_id.unwrap(),
            msg.author.id,
            meeting_room_id,
            announcement_channel_id,
            player_role_id,
            delete_rooms_category_on_game_end,
        ))));
        msg.channel_id
            .say(
                ctx,
                format!(
                    "{} has started a new game. You can join it by typing {}join",
                    msg.author.name,
                    data.get::<Prefix>().unwrap()
                ),
            )
            .await?;
    }
    Ok(())
}
