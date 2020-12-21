use super::prelude::*;
use crate::game::DeathCause;

use rand::{self, distributions::Distribution};
use serenity::model::id::UserId;

#[command]
#[only_in(guilds)]
#[description(
    r#"
Stab another player

(Usage and Sample usage do not include the prefix, but it still must be used)
"#
)]
#[usage("<target user mention>")]
#[example("@KC#7788")]
#[checks(StandardGameCheck, UserIsPlaying)]
pub async fn stab(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let mut game = game_guard.write().await;

    let target = match args.single::<UserId>() {
        Ok(target) => target,
        Err(err) => {
            msg.reply(ctx, "I couldn't get a user ID from your message!")
                .await?;
            return Err(err.into());
        }
    };
    if target == ctx.cache.current_user_id().await {
        msg.reply(ctx, "Ara ara~, you can't stab me~").await?;
        return Ok(());
    }

    if target == game.host() {
        msg.reply(
            ctx,
            "You can't stab the host! That's rather rude towards them, isn't it?",
        )
        .await?;
        return Ok(());
    }

    let channel = msg.channel(ctx).await;
    if channel.is_none() {
        return Err("Got an invalid channel somehow".into());
    }
    let channel = channel.unwrap().guild();
    if channel.is_none() {
        return Err("For some reason this channel didn't have an attached guild".into());
    }
    let channel = channel.unwrap();

    let target_perms = channel.permissions_for_user(ctx, target).await?;
    if !target_perms.read_messages()
        && !target_perms.read_message_history()
        && !target_perms.send_messages()
    {
        msg.reply(ctx, "You can't kill a player that's not in this room! ... you sure are blood thirsty though...").await?;
        return Ok(());
    }

    let (attacker_roll, target_roll) = {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new(1, 21);

        (dist.sample(&mut rng), dist.sample(&mut rng))
    };

    if attacker_roll > target_roll {
        let target = match game.player_mut(target) {
            Ok(target) => target,
            Err(err) => {
                msg.reply(ctx, "You can't stab someone not in the game!")
                    .await?;
                return Err(err.into());
            }
        };

        target
            .set_dead(DeathCause::Stab(msg.author.id), &ctx, channel.id)
            .await?;

        let new_target_perms = crate::helpers::perms::make_denied_override_for_user(target.id());
        if channel.id != game.meeting_room() {
            channel.create_permission(ctx, &new_target_perms).await?;
            return Ok(());
        }

        game.meeting_room()
            .create_permission(ctx, &new_target_perms)
            .await?;
    }

    Ok(())
}
