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
