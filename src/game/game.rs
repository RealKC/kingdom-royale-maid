use super::player::Player;
use serenity::model::id::{GuildId, UserId};

type Host = UserId;

pub struct Game {
    guild: GuildId,
    state: GameState,
    players: Vec<Player>,
    day: u8,
}

enum GameState {
    NotStarted,
    Pregame,
    FirstMeeting,
    SecretMeetings,
    SecondMeeting,
}
