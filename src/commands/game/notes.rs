use super::prelude::*;
use crate::{
    data::Prefix,
    game::item::{MemoBook, Note},
    helpers::react::react_with,
};

use futures::StreamExt;
use serenity::{builder::CreateEmbed, model::id::UserId};
use std::time::Duration;

#[command]
#[only_in(guilds)]
#[description("Allows you to browse your memo book")]
#[aliases("memobook")]
#[checks(GameCheckAllowGameEnded)]
pub async fn notes(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let game = expect_game!(data);

    let player = game.players().get(&msg.author.id);
    if player.is_none() {
        msg.reply(
            ctx,
            "You can't take a look at your note when you're not part of the game",
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    let channel = player.room();
    let memo_book = player.items().memo_book();

    fn make_note_embed(memo: &MemoBook, start: usize, end: usize) -> Option<CreateEmbed> {
        if end >= memo.number_of_written_notes() {
            return None;
        }

        let mut embed = CreateEmbed::default();

        for i in start..end {
            let note = memo.get_note(i);
            if note.is_none() {
                break;
            }
            let note = note.unwrap();

            embed.field(note.when.clone(), format! {"{}. {}", i, note.text}, false);
        }

        Some(embed)
    }

    let mut sent_msg = channel
        .send_message(ctx, |m| {
            if let Some(embed) = make_note_embed(memo_book, 0, 3) {
                m.set_embed(embed);
            } else {
                m.content("You haven't written any notes yet");
            }
            m
        })
        .await?;

    if memo_book.number_of_written_notes() == 0 {
        return Ok(());
    }

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
            sent_msg
                .edit(ctx, |em| {
                    if let Some(embed) =
                        make_note_embed(memo_book, current_start_note, current_start_note + 3)
                    {
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
            sent_msg
                .edit(ctx, |em| {
                    if let Some(embed) =
                        make_note_embed(memo_book, current_start_note - 3, current_start_note)
                    {
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
#[checks(StandardGameCheck)]
pub async fn write_note(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let note = args.rest();

    let data = ctx.data.read().await;
    let mut game = expect_game_mut!(data);

    let game_state = game.state();
    let player = game.players_mut().get_mut(&msg.author.id);

    if player.is_none() {
        msg.reply(
            ctx,
            "You can't write a note to your memo book when you're not in the game",
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    let res = player
        .items_mut()
        .memo_book_mut()
        .add_note(note.into(), game_state.to_time_range().unwrap());

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
#[checks(GameCheckAllowGameEnded)]
pub async fn show_note(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let page = args.parse::<usize>();

    let data = ctx.data.read().await;
    let game = expect_game!(data);

    let player = game.players().get(&msg.author.id);

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

    let note = player.items().memo_book().get_note(page);

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
#[checks(StandardGameCheck)]
pub async fn rip_note(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let page = args.single::<usize>();
    let target = args.single::<UserId>();

    let data = ctx.data.read().await;
    let mut game = expect_game_mut!(data);

    let page = page.unwrap();

    if target.is_err() {
        msg.reply(
            ctx,
            format!(
                "I couldn't get a user from your message. Try {}help ripnote",
                data.get::<Prefix>().unwrap()
            ),
        )
        .await?;
        return Ok(());
    }
    let target = target.unwrap();

    let target_is_in_game = {
        let them = game.players().get(&target);
        them.is_some()
    };

    let game_state = game.state();

    let note = {
        let myself = game.players_mut().get_mut(&msg.author.id);
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

        myself.unwrap().items_mut().memo_book_mut().rip_note(page)
    };

    {
        let them = game.players_mut().get_mut(&target).unwrap();
        let note = note.unwrap_or(Note {
            text: format!("*<An empty page from {}>*", msg.author.mention()),
            when: game_state.to_time_range().unwrap(),
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

        them.items_mut().memo_book_mut().add_ripped_note(note);
    }

    Ok(())
}
