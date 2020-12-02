use serenity::{model::id::UserId, prelude::Mentionable};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DeathCause {
    Sorcery,
    Beheading,
    Assassination,
    Starvation,
    Stab(UserId),
}

pub static NUMBER_EMOJIS_ONE_TO_SIX: [&str; 6] = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣"];
pub static YES_NO_EMOJIS: [&str; 2] = ["🇾", "🇳"];

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SubstitutionStatus {
    HasNot,
    CurrentlyIs,
    Has,
}

impl fmt::Display for DeathCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeathCause::Sorcery => write!(f, "was burnt to a crisp using sorcery."),
            DeathCause::Beheading => write!(f, "was beheaded."),
            DeathCause::Assassination => write!(f, "was assassinated."),
            DeathCause::Starvation => write!(f, "became a mumy due to starvation."),
            DeathCause::Stab(id) => write!(f, "was stabbed by {}", id.mention()),
        }
    }
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

impl GameState {
    pub fn to_time_range(&self) -> Option<String> {
        match self {
            GameState::NotStarted | GameState::GameEnded => None,
            GameState::ABlock => Some("~12".into()),
            GameState::BBlock => Some("12~14".into()),
            GameState::CBlock => Some("14~18".into()),
            GameState::DBlock => Some("18~20".into()),
            GameState::EBlock => Some("20~22".into()),
            GameState::FBlock => Some("22~".into()),
        }
    }
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

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type JoinResult = StdResult<(), JoinError>;
pub type LeaveResult = StdResult<(), LeaveError>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JoinError {
    GameFull,
    YoureTheHost,
    AlreadyIn,
}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JoinError::*;
        match self {
            GameFull => write!(f, "You can't join a full game"),
            YoureTheHost => write!(f, "You can't be both The Host, and a player"), // technically not following canon
            AlreadyIn => write!(f, "You can't join a game multiple times"),
        }
    }
}

impl std::error::Error for JoinError {}

#[derive(Copy, Clone, Debug)]
pub enum LeaveError {
    NotInAGame,
    YoureTheHost,
}

impl fmt::Display for LeaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LeaveError::*;
        match self {
            NotInAGame => write!(f, "You can't leave a game if you're not in one"),
            YoureTheHost => write!(
                f,
                "You can't leave a game if you're The Host, why would you anyway?"
            ),
        }
    }
}
