use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::{
        channel::Message,
        id::{ChannelId, UserId},
    },
};
use tracing::{error, warn};

use super::{
    data::{NUMBER_EMOJIS_ONE_TO_SIX, YES_NO_EMOJIS},
    DeathCause, RoleName,
};
use crate::commands::game::GameContainer;

macro_rules! expect_game {
    ($ctx:ident, $func: literal) => {{
        let data = $ctx.data.read().await;
        let game = data.get::<GameContainer>().cloned();

        if game.is_none() {
            warn!(
                "{} woke up but no game is running. (Game likely ended)",
                $func
            );

            return Ok(());
        }

        game.unwrap()
    }};
}

pub async fn handle_secret_meeting_selection(
    ctx: Context,
    msg: Message,
    user_and_room: (UserId, ChannelId),
) -> CommandResult {
    if let Some(reaction) = msg
        .await_reaction(&ctx)
        .author_id(user_and_room.0)
        .channel_id(user_and_room.1)
        .filter(|r| NUMBER_EMOJIS_ONE_TO_SIX.contains(&r.emoji.to_string().as_str()))
        .await
    {
        let emoji = reaction.as_inner_ref().emoji.to_string();
        if let Ok(idx) = NUMBER_EMOJIS_ONE_TO_SIX.binary_search(&emoji.as_str()) {
            let game = expect_game!(ctx, "handle_secret_meeting_selection");
            let mut game = game.write().await;

            let id = game.players().keys().nth(idx).copied();
            match id {
                Some(id) => {
                    game.players_mut()
                        .get_mut(&user_and_room.0)
                        .unwrap()
                        .set_secret_meeting_partner(id);
                }
                None => {
                    error!("Got a wrong reaction somehow");
                    panic!();
                }
            }
        }
    }
    Ok(())
}

pub async fn handle_king_choosing_target(
    ctx: Context,
    msg: Message,
    king_id: UserId,
    room_id: ChannelId,
) -> CommandResult {
    if let Some(reaction) = msg
        .await_reaction(&ctx)
        .author_id(king_id)
        .channel_id(room_id)
        .filter(|r| NUMBER_EMOJIS_ONE_TO_SIX.contains(&r.emoji.to_string().as_str()))
        .await
    {
        let emoji = reaction.as_inner_ref().emoji.to_string();
        if let Ok(idx) = NUMBER_EMOJIS_ONE_TO_SIX.binary_search(&emoji.as_str()) {
            let game = expect_game!(ctx, "handle_king_choosing_target");
            let mut game = game.write().await;

            let id = game.players_mut().keys().nth(idx).copied();
            match id {
                Some(id) => {
                    game.set_king_murder_target(id);
                }
                None => {
                    error!("Got a wrong reaction somehow");
                    panic!();
                }
            }
        }

        return Ok(());
    }
    Ok(())
}

pub async fn handle_assistant_choice(
    ctx: Context,
    msg: Message,
    assistant_id: UserId,
    room_id: ChannelId,
) -> CommandResult {
    if let Some(reaction) = msg
        .await_reaction(&ctx)
        .filter(|r| YES_NO_EMOJIS.contains(&r.emoji.to_string().as_str()))
        .author_id(assistant_id)
        .channel_id(room_id)
        .await
    {
        if reaction.as_inner_ref().emoji.unicode_eq(YES_NO_EMOJIS[0]) {
            let game = expect_game!(ctx, "handle_assistant_choice");
            let mut game = game.write().await;

            let target_id = game.king_murder_target().id();
            let meeting_room = game.meeting_room();
            let target = game.players_mut().get_mut(&target_id).unwrap();
            target
                .set_dead(target.role_name().into(), &ctx, meeting_room)
                .await?;
        }
        return Ok(());
    }

    Ok(())
}

pub async fn handle_assassination(
    ctx: Context,
    msg: Message,
    revolutionary_id: UserId,
    room_id: ChannelId,
) -> CommandResult {
    if let Some(reaction) = msg
        .await_reaction(&ctx)
        .author_id(revolutionary_id)
        .channel_id(room_id)
        .filter(|r| NUMBER_EMOJIS_ONE_TO_SIX.contains(&r.emoji.to_string().as_str()))
        .await
    {
        let game = expect_game!(ctx, "handle_assassination");
        let mut game = game.write().await;

        let meeting_room = game.meeting_room();

        let emoji = reaction.as_inner_ref().emoji.to_string();
        if let Ok(idx) = NUMBER_EMOJIS_ONE_TO_SIX.binary_search(&emoji.as_str()) {
            let id = game.players().keys().nth(idx).copied();
            match id {
                Some(id) => {
                    let hit_king = game.players().get(&id).unwrap().role_name() == RoleName::King;
                    if hit_king {
                        if game.king_has_substituted() {
                            let double = {
                                let mut res = None;
                                for player in game.players_mut() {
                                    if player.1.role_name() == RoleName::TheDouble {
                                        res = Some(player);
                                        break;
                                    }
                                }
                                res.unwrap()
                            };

                            double
                                .1
                                .set_dead(DeathCause::Assassination, &ctx, meeting_room)
                                .await?;
                        } else {
                            let player = game.players_mut().get_mut(&id);
                            player
                                .unwrap()
                                .set_dead(DeathCause::Assassination, &ctx, meeting_room)
                                .await?;
                        }
                    }
                }
                None => {
                    error!("Got a wrong reaction somehow");
                    panic!();
                }
            }
        }

        return Ok(());
    }
    Ok(())
}
