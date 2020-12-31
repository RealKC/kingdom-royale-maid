//! Follows CBlock
//!
//! The meeting room gets opened in this block. If all alive players are found to be winning, the game ends.

use super::*;

use serenity::{model::id::UserId, prelude::*};
use std::collections::BTreeMap;
use tracing::{info, instrument};

#[derive(Debug, Clone)]
pub(super) struct DBlock {
    players: BTreeMap<UserId, Player>,
    day: u8,
    king_substitution_status: SubstitutionStatus,
}

impl DBlock {
    pub fn new(players: BTreeMap<UserId, Player>, day: u8, kss: SubstitutionStatus) -> Self {
        Self {
            players,
            day,
            king_substitution_status: kss,
        }
    }
}

impl GameState for DBlock {}
impl_timeblock!(DBlock);
impl CanOpenMeetingRoom for DBlock {}
impl_wrap!(DBlock);

impl GameMachine<DBlock> {
    #[instrument(skip(ctx))]
    pub(super) async fn next(self, ctx: &Context) -> Next<EBlock> {
        if self.state.all_alive_have_won() {
            Next::GameEnded(GameMachine {
                metadata: self.metadata,
                state: GameEnded ::new(self.state.players, self.state.day),
            })
        } else {
            if let Err(err) = self.open_meeting_room(ctx).await {
                info!("{}", err);
            }

            Next::Block(GameMachine {
                metadata: self.metadata,
                state: EBlock::new(
                    self.state.players,
                    self.state.day,
                    self.state.king_substitution_status,
                ),
            })
        }
    }

    impl_common_state_boilerplate!();
}
