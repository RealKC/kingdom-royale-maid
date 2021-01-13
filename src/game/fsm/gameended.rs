//! Follows {A, B, C, D, E, F}Block
//!
//! State that exists for after game discussion in which players may share some things while the
//! data structures of a game are still alive.

use super::{macros::state::*, *};

#[derive(Debug, Clone)]
pub(super) struct GameEnded {
    players: BTreeMap<UserId, Player>,
    day: u8,
}
impl GameState for GameEnded {}
impl_wrap!(GameEnded);
impl_timeblock!(GameEnded); // Not really a timeblock, but has convenience methods this needs to export anyway

impl GameEnded {
    pub(super) fn new(players: BTreeMap<UserId, Player>, day: u8) -> Self {
        Self { players, day }
    }
}
