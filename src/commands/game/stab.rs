use crate::game::DeathCause;

use super::prelude::*;
use rand::{self, distributions::Distribution};
use serenity::model::id::UserId;

#[command]
#[only_in(guilds)]
#[description("Stab another player")]
pub async fn stab(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if game.is_none() {
        msg.reply(
            ctx,
            ", you can't stab someone when there isn't a game running!",
        )
        .await?;
        return Ok(());
    }
    let game = game.unwrap();
    let mut game = game.write().await;
    if !game.players().contains_key(&msg.author.id) {
        msg.reply(ctx, ", you can't stab someone when you're not in the game!")
            .await?;
        return Ok(());
    }

    let target = args.single::<UserId>();
    if target.is_err() {
        msg.reply(ctx, ", I couldn't get a user ID from your message!")
            .await?;
        return Err(target.unwrap_err().into());
    }
    let target = target.unwrap();
    if game.players().contains_key(&target) {
        msg.reply(ctx, ", you can't stab someone not in the game!")
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
        msg.reply(ctx, ", you can't kill a player that's not in this room! ... you sure are blood thirsty though...").await?;
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