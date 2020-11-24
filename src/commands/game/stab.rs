use super::prelude::*;
use crate::game::{DeathCause, GameState};

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
pub async fn stab(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if game.is_none() {
        msg.reply_err(
            ctx,
            "you can't stab someone when there isn't a game running!".into(),
        )
        .await?;
        return Ok(());
    }
    let game = game.unwrap();
    let mut game = game.write().await;
    if game.state() == GameState::NotStarted {
        if !game.joined_users().contains(&msg.author.id) {
            msg.reply_err(
                ctx,
                "you can't stab someone when you're not in the game!".into(),
            )
            .await?;
            return Ok(());
        }
    } else if game.state() == GameState::GameEnded {
        msg.reply_err(
            ctx,
            "you can't stab someone when the game just ended".into(),
        )
        .await?;
        return Ok(());
    } else if !game.players().contains_key(&msg.author.id) {
        msg.reply_err(
            ctx,
            "you can't stab someone when you're not in the game!".into(),
        )
        .await?;
        return Ok(());
    }

    let target = args.single::<UserId>();
    if target.is_err() {
        msg.reply_err(ctx, "I couldn't get a user ID from your message!".into())
            .await?;
        return Err(target.unwrap_err().into());
    }
    let target = target.unwrap();

    if target == game.host() {
        msg.reply_err(
            ctx,
            "you can't stab the host! That's rather rude towards them, isn't it?".into(),
        )
        .await?;
        return Ok(());
    }

    if target == ctx.cache.current_user_id().await {
        msg.reply_err(ctx, "ara ara~, you can't stab me~".into())
            .await?;
        return Ok(());
    }

    if game.players().contains_key(&target) {
        msg.reply_err(ctx, "you can't stab someone not in the game!".into())
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
        msg.reply_err(ctx, "you can't kill a player that's not in this room! ... you sure are blood thirsty though...".into()).await?;
        return Ok(());
    }

    let (attacker_roll, target_roll) = {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new(1, 21);

        (dist.sample(&mut rng), dist.sample(&mut rng))
    };

    if attacker_roll > target_roll {
        let target = game.players_mut().get_mut(&target).unwrap();
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
