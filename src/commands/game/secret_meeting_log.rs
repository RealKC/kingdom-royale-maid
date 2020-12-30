use super::prelude::*;
use crate::{
    data::Prefix,
    game::{Player, SecretMeeting},
    helpers::react::react_with,
};

use futures::StreamExt;
use serenity::{
    builder::CreateEmbed,
    collector::ReactionAction,
    model::id::{ChannelId, UserId},
};
use std::time::Duration;
use tracing::{info, warn};

#[command("showlogs")]
#[description(
    r#"
Allows you to show the chat logs in a secret meeting between you and another player.

You can navigate log history using ⏮️ and ⏭️, and anyone can use these reactions in order to navigate the history. The bot will listen for new reactions for a total of five(5) minutes.

*Attachments are intentionally excluded.*"#
)]
#[usage("<day> <target user mention> [optionally, which meeting(as you can meet with a user 2 times a day, use 1 or 2)]")]
#[checks(GameCheckAllowGameEnded, UserIsPlaying)]
#[only_in(guilds)]
pub async fn show_meeting_log(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let game = game_guard.read().await;

    let prefix = ctx
        .data
        .read()
        .await
        .get::<Prefix>()
        .expect("Prefix should always be in ctx.data")
        .clone();

    let day = match args.single::<u8>() {
        Ok(day) => day,
        Err(err) => {
            msg.reply(
            ctx,
            format!("I couldn't get a day out of your message! Please use {}help showlogs to see proper usage.", prefix)
            ).await?;
            info!("Failed parsing day: {:?}", err);
            return Ok(());
        }
    };
    let partner_id = match args.single::<UserId>() {
        Ok(user) => user,
        Err(err) => {
            msg.reply(
            ctx,
            format!("I couldn't get a user from your message! Please use {}help showlogs to see proper usage.", prefix)
            ).await?;
            info!("Failed parsing user id: {:?}", err);
            return Ok(());
        }
    };

    let partner = match game.player(partner_id) {
        Some(player) => player,
        None => {
            msg.reply(
                ctx,
                "You can't show your secret meeting logs with someone who's not in the game!",
            )
            .await?;

            return Err("err".into());
        }
    };

    let player = game.player(msg.author.id).expect("");

    if day > game.day().expect("")
        || (day == game.day().expect("") && !game.secret_meetings_took_place())
    {
        msg.reply(ctx, "You can't show secret meeting logs from the future!")
            .await?;
        return Ok(());
    } else if day == game.day().expect("") && game.secret_meetings_are_happening() {
        msg.reply(ctx, "Time is a fickle thing, and your tablet seems to show that you didn't participate in that meeting from earlier. Did you? Either way, it's not allowing you to show logs you swore existed").await?;
        return Ok(());
    }

    // PANIC SAFETY: Unwrapping here should be safe as we check above that the `day` is within bounds
    let secret_meetings = player.get_secret_meetings_for_day(day).unwrap();
    let which_meeting = args.single::<u8>().ok();

    let (partner, room) = match choose_secret_meeting(
        ctx,
        msg,
        partner,
        day,
        which_meeting,
        secret_meetings,
    )
    .await?
    {
        Some((partner, room)) => (partner, room),
        None => {
            let partner_name = partner_id.to_user(ctx).await?.name;
            warn!(
                "No secret meeting between {author} and {partner} on day={day}",
                author = msg.author.name,
                partner = partner_name,
                day = day
            );
            msg.reply(ctx, "There wasn't actually a secret meeting! Huh...")
                .await?;
            // Not actually an "Ok" state I believe, but I do my own logging and I don't want serenity to log here
            return Ok(());
        }
    };

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

async fn choose_secret_meeting(
    ctx: &Context,
    msg: &Message,
    player: &Player,
    day: u8,
    which_meeting: Option<u8>,
    secret_meetings: &(SecretMeeting, SecretMeeting),
) -> Result<SecretMeeting, Box<dyn std::error::Error + Send + Sync>> {
    // I allow that warning here because I find the code easier to read this way
    #[allow(clippy::collapsible_if)]
    if secret_meetings.0.unwrap().0 == secret_meetings.1.unwrap().0 {
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
            return Err("Player didn't specify the meeting".into());
        };

        if ![1, 2].contains(&which_meeting) {
            msg.reply(
                ctx,
                "You should specify either 1 or 2 for the secret meeting choice",
            )
            .await?;
            return Err("Player didn't do args right".into());
        }

        match which_meeting {
            1 => Ok(secret_meetings.0),
            2 => Ok(secret_meetings.1),
            _ => unreachable!("rustc, cpus, or the universe has failed, for the set {1, 2} now contains more than two elements. woe is upon us"),
        }
    } else {
        // Otherwise we try finding the correct meeting manually
        if secret_meetings.0.unwrap().0 == player.id() {
            Ok(secret_meetings.0)
        } else if secret_meetings.1.unwrap().0 == player.id() {
            Ok(secret_meetings.1)
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
            Err("Players did not meet".into())
        }
    }
}

/// Deals with the high level parts of pagination.
async fn pagination(
    ctx: Context,
    msg: Message,
    channel: ChannelId,
    unicodes: &'static [&'static str],
) {
    let collector = msg
        .await_reactions(&ctx)
        .filter(move |r| unicodes.contains(&r.emoji.to_string().as_str()))
        .channel_id(channel)
        .timeout(Duration::from_secs(300))
        .await;

    collector
        .for_each(|reaction| switch_page(ctx.clone(), msg.clone(), unicodes, reaction))
        .await;
}

/// Fetches messages and builds an embed for a new page
async fn switch_page(
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

    // We don't edit the message if we didn't get any messages that way.
    if fields.is_empty() {
        return;
    }

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
