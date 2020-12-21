pub const ERROR_MESSAGE: &str = r#"This command either misses a `#[checks(StandardGameCheck)` or `#[checks(GameCheckAllowGameEnded)] attribute, or StandardGameCheck or GameCheckAllowGameEnded broke the contract of "This command will only run if there's a game here""#;

/// This locks the passed in `data`, and attempts to get a `GameContainer` from it.
/// If there is no `GameContainer`, it will log an error and return from the enclosing function.
/// If there is a `GameContainer`, it will lock it for reading and return it
#[macro_export]
macro_rules! expect_game {
    ($data:ident) => {{
        let game = $data.get::<GameContainer>();
        if let Some(game) = game {
            let game = game.read().await;

            game
        } else {
            tracing::error!(
                "\n _(read)_: {} ",
                $crate::commands::game::macros::ERROR_MESSAGE
            );
            return Ok(());
        }
    }};
}

/// This locks the passed in `data`, and attempts to get a `GameContainer` from it.
/// If there is no `GameContainer`, it will log an error and return from the enclosing function.
/// If there is a `GameContainer`, it will lock it for writing and return it
#[macro_export]
macro_rules! expect_game_mut {
    ($data:ident) => {{
        let game = $data.get::<GameContainer>();
        if let Some(game) = game {
            let game = game.write().await;

            game
        } else {
            tracing::error!(
                "\n _(write)_ {}",
                $crate::commands::game::macros::ERROR_MESSAGE
            );
            return Ok(());
        }
    }};
}

#[macro_export]
macro_rules! expect_player {
    ($game:ident, $id:expr) => {{
        let players = $game.players();
        if let Some(player) = players.get(&$id) {
            player
        } else {
            tracing::error!("\n _(read)_ This command either lacks the UserIsPlaying check or UserIsPlaying broke the contract it should enforce.");

            return Ok(());
        }
    }}
}
