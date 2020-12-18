use std::time::Duration;

use futures::StreamExt;
use serenity::{
    builder::CreateEmbed,
    collector::ReactionAction,
    model::id::{ChannelId, UserId},
};
use tracing::{info, log::warn};

use crate::{data::Prefix, game::GameState, helpers::react::react_with};

use super::prelude::*;

#[command("showlogs")]
#[description(
    r#"
Allows you to show the chat logs in a secret meeting between you and another player.

You can navigate log history using ⏮️ and ⏭️, and anyone can use these reactions in order to navigate the history. The bot will listen for new reactions for a total of five(5) minutes.

*Attachments are intentionally excluded.*"#
)]
#[usage("<day> <target user mention> [optionally, which meeting(as you can meet with a user 2 times a day, use 1 or 2)]")]
#[checks(GameCheckAllowGameEnded)]
pub async fn show_meeting_log(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let game = expect_game!(data);

    let game_state = game.state();

    let day = args.single::<u8>();
    let user = args.single::<UserId>();

    {
        let prefix = data.get::<Prefix>().unwrap();
        if day.is_err() {
            msg.reply(
                ctx,
                format!("I couldn't get a day out of your message! Please use {}help showlogs to see proper usage.",
                prefix))
                .await?;
            return Ok(());
        }

        if user.is_err() {
            msg.reply(
                    ctx,
                    format!("I couldn't get a user from your message! Please use {}help showlogs to see proper usage.",
                    prefix))
                    .await?;
            return Ok(());
        }
    }

    let day = day.unwrap();
    let user = user.unwrap();

    let player = game.players().get(&user);
    if player.is_none() {
        msg.reply(
            ctx,
            "You can't show secret meeting logs when you're not in a game!",
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    if day > game.day()
        || (day == game.day()
            && [GameState::ABlock, GameState::BBlock, GameState::CBlock].contains(&game_state))
    {
        msg.reply(ctx, "You can't show secret meeting logs from the future!")
            .await?;
        return Ok(());
    } else if day == game.day() && game.state() == GameState::DBlock {
        msg.reply(ctx, "Time is a fickle thing, and your tablet seems to show that you didn't participate in that meeting from earlier. Did you? Either way, it's not allowing you to show logs you swore existed").await?;
        return Ok(());
    }

    let secret_meetings = player.get_secret_meetings_for_day(game.day()).unwrap();

    let which_meeting = args.single::<u8>().ok();

    #[allow(clippy::collapsible_if)]
    let secret_meeting = if secret_meetings.0.unwrap().0 == secret_meetings.1.unwrap().0 {
        // if player had two secret meetings with same dood
        let which_meeting: u8 = if let Some(wm) = which_meeting {
            wm
        } else {
            msg.reply(
                ctx,
                format!(
                    "You've had two meetings with {} on day {}. Please specify which one to choose",
                    secret_meetings.0.unwrap().0.mention(),
                    day
                ),
            )
            .await?;
            return Ok(());
        };

        if ![1, 2].contains(&which_meeting) {
            msg.reply(
                ctx,
                "You should specify either 1 or 2 for the secret meeting choice",
            )
            .await?;
            return Ok(());
        }

        match which_meeting {
            1 => secret_meetings.0,
            2 => secret_meetings.1,
            _ => unreachable!("rustc, cpus, or the universe has failed, for the set {1, 2} now contains more than two elements. woe is upon us"),
        }
    } else {
        if secret_meetings.0.unwrap().0 == player.id() {
            secret_meetings.0
        } else if secret_meetings.1.unwrap().0 == player.id() {
            secret_meetings.1
        } else {
            msg.reply(
                ctx,
                format!(
                    "You haven't met with {} on day {}",
                    player.id().mention(),
                    day
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let (partner, room) = secret_meeting.unwrap();

    let mut messages = room.messages_iter(&ctx).boxed();
    let mut message_fields: Vec<(String, String, bool)> = vec![];
    while let Some(message) = messages.next().await {
        if let Ok(msg) = message {
            if !msg.content.is_empty() {
                message_fields.push((format!("{} said:", msg.author), msg.content, false));
            }
            if message_fields.len() >= 10 {
                break;
            }
        }
    }

    let mut embed = CreateEmbed::default();

    embed
        .title(format!("Secret logs between you and {}", partner.mention()))
        .fields(message_fields);

    let sent_msg = msg
        .channel_id
        .send_message(ctx, |m| m.set_embed(embed))
        .await?;

    static REACTIONS: [&str; 2] = ["⏮️", "⏭️"];

    react_with(ctx, &sent_msg, &REACTIONS).await?;

    let channel = sent_msg.channel_id;

    tokio::task::spawn(pagination(ctx.clone(), sent_msg, channel, &REACTIONS));

    Ok(())
}

/// Creates a task that paginates the !showlogs output, and runs it
async fn pagination(
    ctx: Context,
    msg: Message,
    channel: ChannelId,
    unicodes: &'static [&'static str],
) -> CommandResult {
    let collector = msg
        .await_reactions(&ctx)
        .filter(move |r| unicodes.contains(&r.emoji.to_string().as_str()))
        .channel_id(channel)
        .timeout(Duration::from_secs(300))
        .await;

    collector
        .for_each(|reaction| paginate(ctx.clone(), msg.clone(), unicodes, reaction))
        .await;

    warn!("Reacing this is probably bad????");

    Ok(())
}

async fn paginate(
    ctx: Context,
    mut msg: Message,
    unicodes: &'static [&'static str],
    reaction: Arc<ReactionAction>,
) {
    let message_id = msg.id;

    // [0] is "⏮️" and [1] is "⏭️". [0] Moves backwards, [1] forwards
    let fields = if reaction.as_inner_ref().emoji.unicode_eq(unicodes[0]) {
        msg.channel_id.messages(&ctx, |b| b.after(message_id)).await
    } else {
        msg.channel_id
            .messages(&ctx, |b| b.before(message_id))
            .await
    }
    .map(|mut v| {
        v.truncate(10);
        v
    })
    .or_else(|err| -> Result<Vec<Message>, ()> {
        info!("{:?}", err);
        Ok(vec![])
    })
    .unwrap()
    .iter()
    .map(|m| (format!("{} said:", m.author), m.content.clone(), false))
    .collect::<Vec<_>>();

    let mut embed = CreateEmbed::default();
    embed.fields(fields);

    let edited = msg
        .edit(ctx, |em| {
            em.embed(|e| {
                *e = embed;
                e
            })
        })
        .await;

    info!("{:?}", edited);
}
