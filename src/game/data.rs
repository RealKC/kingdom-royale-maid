use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DeathCause {
    Sorcery,
    Beheading,
    Assassination,
    Starvation,
    Stab,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    NotStarted, // Recruiting phase and stuff
    // Blocks taken from the timetable in the book
    ABlock, // break, standby in own room
    BBlock, // Gathering in the big room, "First meeting"
    CBlock, // Secret meeting partner selection & meeting with them, King, Sorcerer, Knight can act, someone might die during this block
    DBlock, // Gathering in the big room, "Second meeting"
    EBlock, // Dinner, no food => death, Revolutionary can act
    FBlock, // Sleep & Break, is this useful?
    GameEnded,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::NotStarted => write!(f, "Not started"),
            GameState::ABlock => write!(f, "<A>"),
            GameState::BBlock => write!(f, "<B>"),
            GameState::CBlock => write!(f, "<C>"),
            GameState::DBlock => write!(f, "<D>"),
            GameState::EBlock => write!(f, "<E>"),
            GameState::FBlock => write!(f, "<F>"),
            GameState::GameEnded => write!(f, "Game has ended"),
        }
    }
}
