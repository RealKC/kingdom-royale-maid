use serenity::model::id::UserId;

use crate::{data::Prefix, game::GameState};

use super::prelude::*;

#[command("showlogs")]
#[description(
    r#"
Allows you to show the chat logs in a secret meeting between you and another player.

*Attachments are intentionally excluded.*"#
)]
#[usage("<day> <target user mention> [optionally, which meeting(as you can meet with a user 2 times a day, use 1 or 2)]")]
pub async fn show_meeting_log(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();

    if game.is_none() {
        msg.reply_err(
            ctx,
            "you can't show secret meeting logs when there's no game running".into(),
        )
        .await?;
        return Ok(());
    }
    let game = game.unwrap().read().await;

    let game_state = game.state();

    if game_state == GameState::NotStarted {
        msg.reply_err(
            ctx,
            "you can't show secret logs before a game has started".into(),
        )
        .await?;
        return Ok(());
    }

    let day = args.single::<u8>();
    let user = args.single::<UserId>();

    {
        let prefix = data.get::<Prefix>().unwrap();
        if day.is_err() {
            msg.reply_err(
                ctx,
                format!("I couldn't get a day out of your message! Please use {}help showlogs to see proper usage.",
                prefix))
                .await?;
            return Ok(());
        }

        if user.is_err() {
            msg.reply_err(
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
        msg.reply_err(
            ctx,
            "you can't show secret meeting logs when you're not in a game!".into(),
        )
        .await?;
        return Ok(());
    }
    let player = player.unwrap();

    if day > game.day()
        || (day == game.day()
            && [GameState::ABlock, GameState::BBlock, GameState::CBlock].contains(&game_state))
    {
        msg.reply_err(
            ctx,
            "you can't show secret meeting logs from the future!".into(),
        )
        .await?;
        return Ok(());
    }

    let secret_meetings = player.get_secret_meetings_for_day(game.day()).unwrap();

    let which_meeting = args.single::<u8>().ok();

    let secret_meeting = if secret_meetings.1.is_some()
        && secret_meetings.0.unwrap().0 == secret_meetings.1.unwrap().0
    {
        // if player had two secret meetings with same dood
        if which_meeting.is_none() {
            msg.reply_err(
                ctx,
                format!(
                    "you've had two meetings with {} on {}. Please specify which one to choose",
                    secret_meetings.0.unwrap()
                ),
            )
            .await?;
            return Ok(());
        }
        let which_meeting = which_meeting.unwrap();

        if ![1, 2].contains(&which_meeting) {
            msg.reply_err(
                ctx,
                "you should specify either 1 or 2 for the secret meeting choice",
            )
            .await?;
            return Ok(());
        }

        match which_meeting {
            1 => secret_meetings.0,
            2 => secret_meetings.1,
            _ => unreachable!("rustc, cpus, or the universe has failed, for the integer interval [1, 2], aka the set {1, 2} now contains more than two elements. woe is upon us"),
        }
    } else {
        if secret_meetings.0.unwrap().0 == 
    };

    Ok(())
}
