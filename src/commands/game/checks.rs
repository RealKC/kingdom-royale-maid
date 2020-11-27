use crate::game::GameState;

use super::prelude::*;

use serenity::framework::standard::{macros::check, CheckResult, CommandOptions};

// This is the standard check ran before most game commands, it ensures that these commands have a valid game to work with.
#[check]
#[name = "StandardGameCheck"]
pub async fn standard_game(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    command: &CommandOptions,
) -> CheckResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if let Some(game) = game {
        let game = game.read().await;

        match game.state() {
            GameState::NotStarted => {
                let sent = msg
                    .reply_err(
                        ctx,
                        error_messages::GAME_NOT_STARTED[command.names[0]].into(),
                    )
                    .await;
                return CheckResult::new_log(format!(
                    "\nStandardGameCheck: Game wasn't started. Error when sending message (if any): {:?}",
                    if sent.is_err() {
                        Some(sent.err())
                    } else {
                        None
                    }
                ));
            }
            GameState::GameEnded => {
                let sent = msg
                    .reply_err(ctx, error_messages::GAME_ENDED[command.names[0]].into())
                    .await;
                return CheckResult::new_log(format!(
                        "\nStandardGameCheck: Game has ended. Error when sending message (if any): {:?}",
                        if sent.is_err() {
                            Some(sent.err())
                        } else {
                            None
                        }
                    ));
            }
            _ => (),
        };
    } else {
        let sent = msg
            .reply_err(
                ctx,
                error_messages::NEEDS_GAME_TO_EXIST[command.names[0]].into(),
            )
            .await;
        return CheckResult::new_log(format!(
            "\nStandardGameCheck: No game exists. Error when sending message (if any): {:?}",
            if sent.is_err() {
                Some(sent.err())
            } else {
                None
            }
        ));
    }

    CheckResult::Success
}

#[check]
#[name("GameCheckAllowGameEnded")]
pub async fn game_check_allow_game_ended(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    command: &CommandOptions,
) -> CheckResult {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if let Some(game) = game {
        let game = game.read().await;

        if game.state() == GameState::NotStarted {
            let sent = msg
                .reply_err(
                    ctx,
                    error_messages::GAME_NOT_STARTED[command.names[0]].into(),
                )
                .await;
            return CheckResult::new_log(format!(
                "\nStandardGameCheck: Game wasn't started. Error when sending message (if any): {:?}",
                if sent.is_err() {
                    Some(sent.err())
                } else {
                    None
                }
            ));
        }
    } else {
        let sent = msg
            .reply_err(
                ctx,
                error_messages::NEEDS_GAME_TO_EXIST[command.names[0]].into(),
            )
            .await;
        return CheckResult::new_log(format!(
            "\nStandardGameCheck: No game exists. Error when sending message (if any): {:?}",
            if sent.is_err() {
                Some(sent.err())
            } else {
                None
            }
        ));
    }
    CheckResult::Success
}

mod error_messages {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    pub static NEEDS_GAME_TO_EXIST: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "endgathering",
            "you can't end a gathering if there's no game running!",
        );
        map.insert("give", "you can't give items when there's no game running!");
        map.insert(
            "inventory",
            "you can't look into your bag when there's no game running",
        );
        map.insert(
            "nextblock",
            "you can't go to the next time block if there's no game running!",
        );
        map.insert(
            "notes",
            "you can't take a look into your memo book when there isn't a game running on!",
        );
        map.insert(
            "stab",
            "you can't stab someone when there isn't a game running!",
        );
        map.insert(
            "startgathering",
            "you can't start a gathering if there's no game running!",
        );
        map.insert(
            "substitute",
            "you can't 「 substitute 」 with someone when you're not in a game!",
        );

        map
    });

    pub static GAME_NOT_STARTED: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "endgathering",
            "you can't end a meeting in the big room if the game hasn't started yet!",
        );
        map.insert("give", "you can't give items when there's no game running!");
        map.insert(
            "inventory",
            "you can't look into your bag when there's no game running",
        );
        map.insert(
            "nextblock",
            "you can't go to the next time block if there's no game running!",
        );
        map.insert(
            "notes",
            "you can't take a look into your memo book when there isn't a game running on!",
        );
        map.insert(
            "stab",
            "you can't stab someone when there isn't a game running!",
        );
        map.insert(
            "startgathering",
            "you can't start a gathering if there's no game running!",
        );
        map.insert(
            "substitute",
            "you can't 「 substitute 」 with someone when you're not in a game!",
        );

        map
    });

    pub static GAME_ENDED: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "endgathering",
            "you can't end a meeting in the big room if the game has ended!",
        );
        map.insert("give", "you can't give items after a game has ended!");
        map.insert(
            "nextblock",
            "you can't go to the next time block if the game has ended.",
        );
        map.insert("stab", "you can't stab someone after a game has ended!");
        map.insert(
            "startgathering",
            "you can't start a meeting in the big room after the game has ended!",
        );
        map.insert(
            "substitute",
            "you can't 「 substitute 」 with someone after the game has ended!",
        );

        map
    });
}
