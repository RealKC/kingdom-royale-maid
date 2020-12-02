use super::prelude::*;
use crate::helpers::react::react_with;

use serenity::model::id::ChannelId;
use tracing::info;

#[command("delcat")]
#[only_in(guilds)]
#[description("Deletes a category and all channels under it")]
pub async fn delete_category(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let author = guild.member(ctx, msg.author.id).await?;

    if !author.permissions(ctx).await?.manage_channels() {
        msg.reply(
            ctx,
            "You can't delete a category if you don't have the \"Manage Channels\" permission",
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
        msg.reply(
            ctx,
            "I can't delete channels when I lack the \"Manage Channels\" permission",
        )
        .await?;
        return Ok(());
    }

    let category = args.single::<ChannelId>();
    if category.is_err() {
        let err = category.err();
        msg.reply(
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

    let category = category.to_channel(&ctx.http).await?;
    let category = category.category();

    if category.is_none() {
        msg.reply(
            ctx,
            "You didn't actually pass me a category. I can only delete categories",
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

    let mut message_string = "You will delete the following channels:\n".to_string();

    for channel in &channels_to_delete {
        message_string.push_str(&format!("• {}\n", channel.mention()));
    }

    message_string.push_str(&format!(
        "and the category {}({})",
        category.mention(),
        category.id
    ));

    message_string.push_str(
        r#"
Are you sure you want to do this? React with 🇾 if so, and with 🇳 if not.
⚠️ **This action cannot be reversed**"#,
    );

    let mut sent_msg = msg.channel_id.say(ctx, message_string).await?;

    static REACTIONS: [&str; 2] = ["🇾", "🇳"];
    react_with(ctx, &sent_msg, &REACTIONS).await?;

    if let Some(reaction) = sent_msg
        .await_reaction(&ctx)
        .filter(|r| REACTIONS.contains(&r.emoji.to_string().as_str()))
        .guild_id(msg.guild_id.unwrap())
        .author_id(msg.author.id)
        .await
    {
        info!("We got woke up");
        if reaction.as_inner_ref().emoji.unicode_eq(REACTIONS[0]) {
            for channel in channels_to_delete {
                channel.delete(ctx).await?;
            }
        }

        category.delete(&ctx).await?;

        sent_msg
            .edit(ctx, |m| {
                m.content("{}, successfully deleted those channels!")
            })
            .await?
    }

    Ok(())
}
