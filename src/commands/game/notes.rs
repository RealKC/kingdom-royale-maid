use std::time::Duration;

use futures::StreamExt;
use serenity::builder::CreateEmbed;

use super::prelude::*;
use crate::{
    game::{item::MemoBook, GameState},
    helpers::react::react_with,
};

#[command]
#[description("Allows you to browse your memo book")]
#[aliases("memobook")]
pub async fn notes(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let game = data.get::<GameContainer>();
    if game.is_none() {
        msg.reply_err(
            ctx,
            "you can't take a look into your memo book when there isn't a game running on".into(),
        )
        .await?;
        return Ok(());
    }
    let game = game.unwrap().read().await;

    if game.state() == GameState::NotStarted {
        msg.reply_err(
            ctx,
            "you can't take a look into your memo book before the game starts".into(),
        )
        .await?;
        return Ok(());
    }

    let player = game.players().get(&msg.author.id);
    if player.is_none() {
        msg.reply_err(
            ctx,
            "you can't take a look at your note when you're not part of the game".into(),
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
