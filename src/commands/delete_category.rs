use serenity::model::id::ChannelId;
use tracing::info;

use super::prelude::*;
use crate::helpers::{react::react_with, serenity_ext::MaidReply};

#[command("delcat")]
#[only_in(guilds)]
#[description("Deletes a category and all channels under it")]
pub async fn delete_category(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let author = guild.member(ctx, msg.author.id).await?;

    if !author.permissions(ctx).await?.manage_channels() {
        msg.reply_err(
            ctx,
            "you can't delete a category if you don't have the \"Manage Channels\" permission"
                .into(),
        )
        .await?;
        return Ok(());
    }

    if !guild
        .member(ctx, ctx.cache.current_user_id().await)
        .await?
        .permissions(ctx)
        .await?
        .manage_channels()
    {
        msg.reply_err(
            ctx,
            "I can't delete channels when I lack the \"Manage Channels\" permission".into(),
        )
        .await?;
        return Ok(());
    }

    let category = args.single::<ChannelId>();
    if category.is_err() {
        let err = category.err();
        msg.reply_err(
            ctx,
            format!(
                "I couldn't get a channel ID from your message. I got this error: {:?}",
                err
            ),
        )
        .await?;
        info!("Got error {:?} while trying to parse a channel ID", err);
        return Ok(());
    }
    let category = category.unwrap();

    let category = category.to_channel(ctx).await?;
    let category = category.category();

    if category.is_none() {
        msg.reply_err(
            ctx,
            "you didn't actually pass me a category. I can only delete categories".into(),
        )
        .await?;
        return Ok(());
    }
    let category = category.unwrap();

    let mut channels_to_delete = vec![];

    for channel in msg.guild(ctx).await.unwrap().channels(ctx).await? {
        if channel.1.category_id == Some(category.id) {
            channels_to_delete.push(channel.0);
        }
    }

    let mut message_string = "You will delete the following channels: ".to_string();

    for channel in &channels_to_delete {
        message_string.push_str(&format!("• {}\n", channel.mention()));
    }

    message_string
        .push_str("\nAre you sure you want to do this? React with 🇾 if so, and with 🇳 if not");

    let sent_msg = msg.channel_id.say(ctx, message_string).await?;

    static REACTIONS: [&str; 2] = ["🇾", "🇳"];
    react_with(ctx, &sent_msg, &REACTIONS).await?;

    if let Some(reaction) = msg
        .await_reaction(&ctx)
        .filter(|r| REACTIONS.contains(&r.emoji.to_string().as_str()))
        .guild_id(msg.guild_id.unwrap())
        .author_id(msg.author.id)
        .await
    {
        let emoji = reaction.as_inner_ref().emoji.to_string();
        if emoji.as_str() == REACTIONS[0] {
            for channel in channels_to_delete {
                channel.delete(ctx).await?;
            }
        }
        msg.reply(ctx, "Those channels have been successfully deleted")
            .await?;
    }

    Ok(())
}
