use super::*;

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
                state: GameEnded ::new(self.state.players, self.state.day),
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

    pub(super) fn set_king_substitution_status(&mut self, kss: SubstitutionStatus) {
        self.state.king_substitution_status = kss;
    }

    pub(super) fn king_has_substituted(&self) -> bool {
        self.state.king_substitution_status == SubstitutionStatus::Has
    }
}
