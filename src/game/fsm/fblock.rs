//! Follows EBlock
//!
//! During this state, if all alive players are found to be winning the game ends, otherwise,
//! the day number gets increased

use super::{macros::state::*, *};

use serenity::model::id::UserId;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(super) struct FBlock {
    players: BTreeMap<UserId, Player>,
    day: u8,
    king_substitution_status: SubstitutionStatus,
}

impl FBlock {
    pub fn new(players: BTreeMap<UserId, Player>, day: u8, kss: SubstitutionStatus) -> Self {
        Self {
            players,
            day,
            king_substitution_status: kss,
        }
    }
}

impl GameState for FBlock {}
impl_timeblock!(FBlock);
impl_wrap!(FBlock);

impl GameMachine<FBlock> {
    pub(super) fn next(self) -> Next<ABlock> {
        if self.state.all_alive_have_won() {
            Next::GameEnded(GameMachine {
                metadata: self.metadata,
                state: GameEnded::new(self.state.players, self.state.day),
            })
        } else {
            Next::Block(GameMachine {
                metadata: self.metadata,
                state: ABlock::new(
                    self.state.players,
                    self.state.day,
                    self.state.king_substitution_status,
                ),
            })
        }
    }

    impl_common_state_boilerplate!();
}
