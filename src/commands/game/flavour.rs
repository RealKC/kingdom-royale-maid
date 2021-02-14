use crate::{data::Db, game::item::Item};

use super::prelude::*;

#[command]
#[aliases("lookat")]
#[description("This command allows you to look at items and other objects in order to get information about them.")]
#[checks(StandardGameCheck, UserIsPlaying)]
pub async fn inspect(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let item = args.rest();

    match item.to_lowercase().as_ref() {
        "tv" => {
            msg.reply(
                ctx,
                "You look at the TV on the wall, or at least you think it's a TV, since that's what it looks like. It seems to be flush against the wall."
            )
            .await?;
        }

        "door" => {
            msg.reply(
                ctx,
                "You look at the door of your room. It's just a normal door.",
            )
            .await?;
        }

        "bed" => {
            msg.reply(ctx, "You look at the bed in your room. It looks depressing")
                .await?;
        }

        "watch" => {
            let game_guard = get_game_guard(ctx).await?;
            let game = game_guard.read().await;
            let player = game.player(msg.author.id).expect("Have a player here");

            let pool = ctx
                .data
                .read()
                .await
                .get::<Db>()
                .cloned()
                .expect("Have a pool in ctx.data");

            let watch = player.get_item("watch", &pool).await?;

            match watch {
                Some(watch) => {
                    msg.reply(
                        ctx,
                        format!(
                            "You look at your watch. It's just a normal {watch}.",
                            watch = watch.1.name
                        ),
                    )
                    .await?;
                }
                None => {
                    msg.reply(ctx, "Your watch is gone :c").await?;
                }
            }
        }

        "food" | "food bar" | "food ration" | "food item" | "snack" => {
            let game_guard = get_game_guard(ctx).await?;
            let game = game_guard.read().await;
            let player = game
                .player(msg.author.id)
                .expect("inspect: need a player here");

            let pool = ctx
                .data
                .read()
                .await
                .get::<Db>()
                .cloned()
                .expect("Have a pool in ctx.data");

            let item = player.get_item(Item::FOOD_NAME, &pool).await?;

            match item {
                Some(food) => {
                    msg.reply(
                        ctx,
                        format!("You see {count} food bars in your bag.", count = food.0),
                    )
                    .await?;
                }
                None => {
                    msg.reply(ctx, "You see no food in your bag. Will you starve tonight?")
                        .await?;
                }
            }
        }

        "tablet" | "digital tablet" => {
            let game_guard = get_game_guard(ctx).await?;
            let game = game_guard.read().await;
            let day = game.day().expect("inspect: should have a game running");

            if day == 1 {
                if game
                    .time_range()
                    .expect("inspect: should have a game running")
                    == "~12"
                {
                    msg.reply(ctx, "You look at the tablet. It currently is off.")
                        .await?;
                } else if game
                    .time_range()
                    .expect("inspect: should have a game running")
                    == "12~14"
                {
                    msg.reply(
                        ctx,
                        r#"You look at the tablet. It says "Logs" on it, but it seems to be empty"#,
                    )
                    .await?;
                }
            } else {
                msg.reply(ctx, "You look at the tablet. It stores logs when you talk with someone else at a secret meeting. You can show them to other people...").await?;
            }
        }

        "ballpoint pen" | "pen" => {
            msg.reply(ctx, "You look at the pen. It's accompanied by a memo book")
                .await?;
        }

        "memo book" | "notebook" => {
            msg.reply(ctx, "You look at the memo book. It's accompanied by a ballpoint pen. You wonder if writing in it would get recorded in the tablet...").await?;
        }

        "bag" => {
            msg.reply(
                ctx,
                "You look at the bag on your table, it's got some stuff in it. Food rations, a watch, a digital tablet, a ball-point pen, a memo book, ... a knife?"
            )
            .await?;
        }

        "table" => {
            let game_guard = get_game_guard(ctx).await?;
            let game = game_guard.read().await;
            let player = game.player(msg.author.id).expect("yes");

            if msg.channel_id == player.room() {
                msg.reply(
                    ctx,
                    "In the middle of your room is a table. It has a bag on it.",
                )
                .await?;
            } else if msg.channel_id == game.meeting_room() {
                msg.reply(ctx, "In the middle of the room is a big table, with 6 TV hanging from the ceiling over it.").await?;
            } else {
                // In a secret meeting
                msg.reply(ctx, "In the middle of your meeting partner's room is a table. It doesn't strike you as any different than yours").await?;
            }
        }

        _ => (),
    };

    msg.reply(ctx, "I couldn't get an item or object from your message!")
        .await?;

    Ok(())
}

#[command("lookaround")]
#[description("This command allows you to look around in order to get a general description of the room you're in.")]
#[checks(StandardGameCheck, UserIsPlaying)]
pub async fn look_around(ctx: &Context, msg: &Message) -> CommandResult {
    let game_guard = get_game_guard(ctx).await?;
    let game = game_guard.read().await;

    let player = game.player(msg.author.id);

    if let Some(player) = player {
        if msg.channel_id == player.room() {
            msg.reply(ctx, "You look around your room. There is a TV flush with the wall on the North side, a bed right of the TV, a table with a bag on it in the middle, a place that reminds you of a bathroom on the East side, and a door on the South side.").await?;
        } else if msg.channel_id == game.meeting_room() {
            msg.reply(ctx, "You look around the meeting room. It is a dark place illuminated only by the TVs above the table in the middle. You see a single way out of it.").await?;
        } else {
            // In a secret meeting
            msg.reply(ctx, "Your partner's room doesn't look any different than yours, though you don't pry in their belongings so they might be hiding some things, or not.").await?;
        }
    }

    Ok(())
}
