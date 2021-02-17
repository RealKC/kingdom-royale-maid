use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::{ChannelId, UserId},
    },
};
use sqlx::PgPool;
use tracing::{error, info, warn};

use crate::game::fsm::{macros::tasks::expect_game, reactions::*};

pub async fn handle_secret_meeting_selection(
    ctx: Context,
    msg: Message,
    user_and_room: (UserId, ChannelId),
    pool: PgPool,
) {
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
            let game = game.write().await;

            // Panic safety: The only GameState that's not a TimeBlock is NotStarted, and this can never wake up then
            let id = game
                .players()
                .expect("handle_secret_meeting_selection should only be called by a TimeBlock")
                .keys()
                .nth(idx)
                .copied();
            match id {
                Some(id) => {
                    // Panic safety: The only GameState that's not a TimeBlock is NotStarted, and this can never wake up then
                    let res = game
                        .add_secret_meeting(user_and_room.0, id, user_and_room.1, &pool)
                        .await;

                    info!("{:?}", res);
                }
                None => {
                    error!("Got a wrong reaction somehow");
                }
            }
        }
    }
}

pub async fn handle_king_choosing_target(
    ctx: Context,
    msg: Message,
    king_id: UserId,
    room_id: ChannelId,
) {
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

            // Panic safety: The only GameState that's not a TimeBlock is NotStarted, and this can never wake up then
            let id = game
                .players_mut()
                .expect("handle_king_choosing_target should only be called in a TimeBlock")
                .keys()
                .nth(idx)
                .copied();
            match id {
                Some(id) => {
                    game.set_king_murder_target(id);
                }
                None => {
                    error!("Got a wrong reaction somehow");
                }
            }
        }
    }
}

pub async fn handle_assistant_choice(
    ctx: Context,
    msg: Message,
    assistant_id: UserId,
    room_id: ChannelId,
) {
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

            let target_id = if let Some(id) = game.king_murder_target() {
                id
            } else {
                warn!("handle_assistance_choice woke up in the wrong block");
                return;
            };

            let meeting_room = game.meeting_room();
            let target = if let Some(players) = game.players_mut() {
                players.get_mut(&target_id).unwrap()
            } else {
                warn!("handle_assistance_choice woke up in the wrong block");
                return;
            };
            let _ = target
                .set_dead(target.role_name().into(), &ctx, meeting_room)
                .await
                .map_err(|e| {
                    warn!("{}", e);
                });
        }
    }
}
