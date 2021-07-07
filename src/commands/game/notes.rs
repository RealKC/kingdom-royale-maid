use super::prelude::*;
use crate::{data::Prefix, game::item::Note, helpers::react::react_with};

use futures::StreamExt;
use serenity::model::id::UserId;
use std::time::Duration;

#[command]
#[only_in(guilds)]
#[description("Allows you to browse your memo book")]
#[aliases("memobook")]
#[checks(GameCheckAllowGameEnded, UserIsPlaying)]
pub async fn notes(ctx: &Context, msg: &Message) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let game = game_guard.read().await;

    let pool = ctx
        .data
        .read()
        .await
        .get::<Db>()
        .cloned()
        .expect("Have a pool in ctx.data");

    let player = game
        .player(msg.author.id)
        .expect("notes: UserIsPlaying broke its contract");
    let channel = player.room();

    let embed = player.get_notes_between_as_embed(0, 3, &pool).await?;

    let mut sent_msg = channel
        .send_message(ctx, |m| {
            if let Some(embed) = embed {
                m.set_embed(embed)
            } else {
                m.content("You haven't written any notes yet")
            }
        })
        .await?;

    static REACTIONS: [&str; 2] = ["⏮️", "⏭️"];
    react_with(ctx, &sent_msg, &REACTIONS).await?;

    let mut reactions_collector = sent_msg
        .await_reactions(ctx)
        .filter(|r| REACTIONS.contains(&r.emoji.to_string().as_str()))
        .timeout(Duration::from_secs(120))
        .channel_id(sent_msg.channel_id)
        .author_id(player.id())
        .await;

    let mut current_start_note = 3;
    while let Some(reaction) = reactions_collector.next().await {
        if reaction.as_inner_ref().emoji.to_string() == REACTIONS[0] {
            let embed = player
                .get_notes_between_as_embed(current_start_note, current_start_note + 3, &pool)
                .await?;

            sent_msg
                .edit(ctx, |em| {
                    if let Some(embed) = embed {
                        em.embed(|e| {
                            *e = embed;
                            e
                        });
                        current_start_note += 3;
                    }
                    em
                })
                .await?;
        } else if reaction.as_inner_ref().emoji.to_string() == REACTIONS[1] {
            let embed = player
                .get_notes_between_as_embed(current_start_note - 3, current_start_note, &pool)
                .await?;

            sent_msg
                .edit(ctx, |em| {
                    if let Some(embed) = embed {
                        em.embed(|e| {
                            *e = embed;
                            e
                        });
                        current_start_note -= 3;
                    }
                    em
                })
                .await?;
        } else {
            // Do nothing if we, for some reason, get a wrong reaction
        }
    }

    Ok(())
}

#[command("writenote")]
#[aliases("wnote", "wn")]
#[description(r#"Allows you to write a note in your book, note that this consumes a page in it, and you will not be able to write in that page anymore. You may write at most 128 notes, that may not be longer than 512 characters.

(Usage and Sample usage do not include the prefix, but it still must be used)
"#)]
#[usage("your note here, can't be too long")]
#[checks(StandardGameCheck, UserIsPlaying)]
pub async fn write_note(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let note = args.rest();

    let game_guard = get_game_guard(ctx).await?;
    let mut game = game_guard.write().await;

    let pool = ctx
        .data
        .read()
        .await
        .get::<Db>()
        .cloned()
        .expect("Have a pool in ctx.data");

    let time_range = game
        .time_range()
        .expect("write_note: StandardGameCheck broke its contract");
    let player = game.player_mut(msg.author.id);

    if player.is_none() {
        msg.reply(
            ctx,
            "You can't write a note to your memo book when you're not in the game",
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    let res = player.add_note(note, time_range, &pool).await;

    match res {
        Ok(_) => (),
        Err(err) => msg.reply(ctx, err).await.map(|_| ())?,
    }

    Ok(())
}

#[command("shownote")]
#[aliases("shn")]
#[description(
    r#"
Shows a note at "page" N in the current channel.

(Usage and Sample usage do not include the prefix, but it still must be used)"#
)]
#[usage("N")]
#[checks(GameCheckAllowGameEnded, UserIsPlaying)]
pub async fn show_note(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let page = args.parse::<usize>();

    let game_guard = get_game_guard(ctx).await?;
    let game = game_guard.write().await;

    let pool = ctx
        .data
        .read()
        .await
        .get::<Db>()
        .cloned()
        .expect("Have a pool in ctx.data");

    let player = game.player(msg.author.id);

    if player.is_none() {
        msg.reply(
            ctx,
            "You can't show a note from your memo book when you're not in the game",
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    if page.is_err() {
        msg.reply(ctx, "I couldn't get a number from your message!")
            .await?;
        return Ok(());
    }
    let page = page.unwrap();

    let note = player.get_note(page, &pool).await?;

    if note.is_none() {
        msg.channel_id
            .say(
                ctx,
                format!("*{} shows an empty page*", msg.author.mention()),
            )
            .await?;
    } else {
        msg.channel_id.say(ctx, &note.unwrap().text).await?;
    }

    Ok(())
}

#[command("ripnote")]
#[description(r#"
Allows you to rip a note out of your memobook and give it to someone. Note that ripping a note will **permanently** decrease the amount of notes you can write.

(Usage and Sample usage do not include the prefix, but it still must be used)
"#)]
#[usage("<page> <user mention>")]
#[example("5 @KC#7788")]
#[checks(StandardGameCheck, UserIsPlaying)]
pub async fn rip_note(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let page = args.single::<usize>();
    let target = args.single::<UserId>();

    let game_guard = get_game_guard(ctx).await?;
    let mut game = game_guard.write().await;

    let pool = ctx
        .data
        .read()
        .await
        .get::<Db>()
        .cloned()
        .expect("Have a pool in ctx.data");

    let page = page.unwrap();

    let prefix = ctx
        .data
        .read()
        .await
        .get::<Prefix>()
        .expect("Prefix should always be in ctx.data")
        .clone();
    if target.is_err() {
        msg.reply(
            ctx,
            format!(
                "I couldn't get a user from your message. Try {}help ripnote",
                prefix
            ),
        )
        .await?;
        return Ok(());
    }
    let target = target.unwrap();

    let target_is_in_game = {
        let them = game.player(target);
        them.is_some()
    };

    let note = {
        let myself = game.player_mut(msg.author.id);
        if myself.is_none() {
            msg.reply(ctx, "You can't give a note when you're not in the game")
                .await?;
            return Ok(());
        }

        if !target_is_in_game {
            msg.reply(
                ctx,
                "You can't give a note to someone who's not in the game",
            )
            .await?;
            return Ok(());
        }

        myself.unwrap().rip_note(page, &pool).await?
    };

    let time_range = game
        .time_range()
        .expect("rip_note: StandardGameCheck broke its contract")
        .to_string();
    {
        let them = game.player_mut(target).unwrap();
        let note = note.unwrap_or(Note {
            text: format!("*<An empty page from {}>*", msg.author.mention()),
            when: time_range,
            ripped: true,
        });

        them.room()
            .say(
                ctx,
                format!(
                    "You've received a note from {}. Its contents are {}",
                    msg.author.mention(),
                    &note.text
                ),
            )
            .await?;

        them.add_ripped_note(note, &pool).await?;
    }

    Ok(())
}
