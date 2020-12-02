use crate::game::GameState;

use super::prelude::*;

use serenity::framework::standard::{macros::check, CommandOptions, Reason};

// This is the standard check ran before most game commands, it ensures that these commands have a valid game to work with.
#[check]
#[name = "StandardGameCheck"]
pub async fn standard_game(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    command: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if let Some(game) = game {
        let game = game.read().await;

        match game.state() {
            GameState::NotStarted => {
                let sent = msg
                    .reply(ctx, error_messages::GAME_NOT_STARTED[command.names[0]])
                    .await;
                return Err(Reason::Log(format!(
                    "\nStandardGameCheck: Game wasn't started. Error when sending message (if any): {:?}",
                    if sent.is_err() {
                        Some(sent.err())
                    } else {
                        None
                    }
                )));
            }
            GameState::GameEnded => {
                let sent = msg
                    .reply(ctx, error_messages::GAME_ENDED[command.names[0]])
                    .await;
                return Err(Reason::Log(format!(
                        "\nStandardGameCheck: Game has ended. Error when sending message (if any): {:?}",
                        if sent.is_err() {
                            Some(sent.err())
                        } else {
                            None
                        }
                    )));
            }
            _ => (),
        };
    } else {
        let sent = msg
            .reply(ctx, error_messages::NEEDS_GAME_TO_EXIST[command.names[0]])
            .await;
        return Err(Reason::Log(format!(
            "\nStandardGameCheck: No game exists. Error when sending message (if any): {:?}",
            if sent.is_err() {
                Some(sent.err())
            } else {
                None
            }
        )));
    }

    Ok(())
}

#[check]
#[name("GameCheckAllowGameEnded")]
pub async fn game_check_allow_game_ended(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    command: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if let Some(game) = game {
        let game = game.read().await;

        if game.state() == GameState::NotStarted {
            let sent = msg
                .reply(ctx, error_messages::GAME_NOT_STARTED[command.names[0]])
                .await;
            return Err(Reason::Log(format!(
                "\nStandardGameCheck: Game wasn't started. Error when sending message (if any): {:?}",
                if sent.is_err() {
                    Some(sent.err())
                } else {
                    None
                }
            )));
        }
    } else {
        let sent = msg
            .reply(ctx, error_messages::NEEDS_GAME_TO_EXIST[command.names[0]])
            .await;
        return Err(Reason::Log(format!(
            "\nStandardGameCheck: No game exists. Error when sending message (if any): {:?}",
            if sent.is_err() {
                Some(sent.err())
            } else {
                None
            }
        )));
    }
    Ok(())
}

mod error_messages {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    pub static NEEDS_GAME_TO_EXIST: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "endgathering",
            "You can't end a gathering if there's no game running!",
        );
        map.insert("give", "You can't give items when there's no game running!");
        map.insert(
            "inventory",
            "You can't look into your bag when there's no game running",
        );
        map.insert(
            "nextblock",
            "You can't go to the next time block if there's no game running!",
        );
        map.insert(
            "notes",
            "You can't take a look into your memo book when there isn't a game running on!",
        );
        map.insert(
            "stab",
            "You can't stab someone when there isn't a game running!",
        );
        map.insert(
            "startgathering",
            "You can't start a gathering if there's no game running!",
        );
        map.insert(
            "substitute",
            "You can't 「 substitute 」 with someone when you're not in a game!",
        );

        map
    });

    pub static GAME_NOT_STARTED: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "endgathering",
            "You can't end a meeting in the big room if the game hasn't started yet!",
        );
        map.insert("give", "You can't give items when there's no game running!");
        map.insert(
            "inventory",
            "You can't look into your bag when there's no game running",
        );
        map.insert(
            "nextblock",
            "You can't go to the next time block if there's no game running!",
        );
        map.insert(
            "notes",
            "You can't take a look into your memo book when there isn't a game running on!",
        );
        map.insert(
            "stab",
            "You can't stab someone when there isn't a game running!",
        );
        map.insert(
            "startgathering",
            "You can't start a gathering if there's no game running!",
        );
        map.insert(
            "substitute",
            "You can't 「 substitute 」 with someone when you're not in a game!",
        );

        map
    });

    pub static GAME_ENDED: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert(
            "endgathering",
            "You can't end a meeting in the big room if the game has ended!",
        );
        map.insert("give", "You can't give items after a game has ended!");
        map.insert(
            "nextblock",
            "You can't go to the next time block if the game has ended.",
        );
        map.insert("stab", "You can't stab someone after a game has ended!");
        map.insert(
            "startgathering",
            "You can't start a meeting in the big room after the game has ended!",
        );
        map.insert(
            "substitute",
            "You can't 「 substitute 」 with someone after the game has ended!",
        );

        map
    });
}
