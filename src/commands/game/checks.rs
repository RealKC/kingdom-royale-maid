use std::collections::HashMap;

use crate::game::GameState;

use super::prelude::*;

use serenity::framework::standard::{macros::check, CommandOptions, Reason};

// This is the standard check ran before most game commands, it ensures that these commands have a valid game to work with.
#[check]
#[name = "StandardGameCheck"]
pub async fn standard_game(
    ctx: &Context,
    _: &Message,
    _: &mut Args,
    command: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if let Some(game) = game {
        let game = game.read().await;

        match game.state() {
            GameState::NotStarted => {
                return Err(make_reason(
                    command,
                    "Game wasn't started",
                    &*error_messages::GAME_NOT_STARTED,
                ));
            }
            GameState::GameEnded => {
                return Err(make_reason(
                    command,
                    "Game has ended",
                    &*error_messages::GAME_ENDED,
                ));
            }
            _ => (),
        };
    } else {
        return Err(make_reason(
            command,
            "No game exists",
            &*error_messages::NEEDS_GAME_TO_EXIST,
        ));
    }

    Ok(())
}

#[check]
#[name("GameCheckAllowGameEnded")]
pub async fn game_check_allow_game_ended(
    ctx: &Context,
    _: &Message,
    _: &mut Args,
    command: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let game = data.get::<GameContainer>();
    if let Some(game) = game {
        let game = game.read().await;

        if game.state() == GameState::NotStarted {
            return Err(make_reason(
                command,
                "Game wasn't started",
                &*error_messages::GAME_NOT_STARTED,
            ));
        }
    } else {
        return Err(make_reason(
            command,
            "No game exists",
            &*error_messages::NEEDS_GAME_TO_EXIST,
        ));
    }
    Ok(())
}

fn make_reason(command: &CommandOptions, log: &str, map: &HashMap<&str, &str>) -> Reason {
    use Reason::{Log, UserAndLog};
    match map.get(command.names[0]) {
        Some(message) => UserAndLog {
            user: message.to_string(),
            log: log.to_string(),
        },
        None => Log(format!(
            "\n{ascii}\n\tMissing entry in hashmaps for {cmd}",
            ascii = error_messages::CHECK_BAD,
            cmd = command.names[0]
        )),
    }
}

/// Module containing statics for different error messages
mod error_messages {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    /// Error messages for commands that need a game to exist in `ctx.data`
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
        map.insert(
            "writenote",
            "You can't write a note to your memo book when a game hasn't started yet",
        );
        map.insert(
            "shownote",
            "You can't show a note from your memo book when there isn't a game running!",
        );
        map.insert(
            "ripnote",
            "You can't rip a note out of your memo book when there's no game running",
        );
        map.insert(
            "showlogs",
            "You can't see secret meeting logs when there's no game running!",
        );

        map
    });

    /// Error messages that need `game.state()` to be different than `GameState::NotStarted`
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
        map.insert(
            "writenote",
            "You can't write a note to your memo book before the game starts",
        );
        map.insert(
            "shownote",
            "You can't show a note from your memo book before the game starts",
        );
        map.insert(
            "ripnote",
            "You can't rip a note out of your memo book when the game hasn't started yet",
        );
        map.insert(
            "showlogs",
            "You can't see secret meeting logs before a game started!",
        );

        map
    });

    /// Error messages for commands that need `game.state()` to be different than `GameState::GameEnded`
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
        map.insert(
            "writenote",
            "You can't write a note to your memo book after a game has ended",
        );
        map.insert(
            "ripnote",
            "You can't rip a note out of your memo book after the game has ended!",
        );

        map
    });

    pub static CHECK_BAD: &str = r#"

     _____ _    _ ______ _____ _  __  ____          _____  
    / ____| |  | |  ____/ ____| |/ / |  _ \   /\   |  __ \ 
   | |    | |__| | |__ | |    | ' /  | |_) | /  \  | |  | |
   | |    |  __  |  __|| |    |  <   |  _ < / /\ \ | |  | |
   | |____| |  | | |___| |____| . \  | |_) / ____ \| |__| |
    \_____|_|  |_|______\_____|_|\_\ |____/_/    \_\_____/ 
                                                           
    "#;
}
