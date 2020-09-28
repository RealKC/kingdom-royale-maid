use super::player::Player;
use serenity::model::id::{GuildId, UserId};

type Host = UserId;

pub struct Game {
    guild: GuildId,
    state: GameState,
    host: Host,
    players: Vec<Player>,
    day: u8,
}

impl Game {
    pub fn state(&self) -> GameState{
        self.state
    }
}

#[derive(Copy, Clone)]
pub enum GameState {
    NotStarted, // Recruiting phase and stuff
    Pregame, // for giving players an introduction to the game _in character_
    // Blocks taken from the timetable in the book
    ABlock, // break, standby in own room
    BBlock, // Gathering in the big room, "First meeting"
    CBlock, // Secret meeting partner selection & meeting with them, King, Sorcerer, Knight can act, someone might die during this block
    DBlock, // Gathering in the big room, "Second meeting"
    EBlock, // Dinner, no food => death, Revolutionary can act
    FBlock, // Sleep & Break, is this useful?
    GameEnded
}
