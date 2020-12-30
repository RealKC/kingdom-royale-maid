
use super::*;
use serenity::model::id::UserId;
use tracing::{info, instrument};
use serenity::prelude::*;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub(super) struct BBlock {
    players: BTreeMap<UserId, Player>,
    day: u8,
    king_substitution_status: SubstitutionStatus
}

impl BBlock {
    pub fn new(players: BTreeMap<UserId, Player>, day: u8, kss: SubstitutionStatus) -> Self {
        Self { players, day ,king_substitution_status: kss}
    }
}

impl GameState for BBlock {}
impl_timeblock!(BBlock);
impl_wrap!(BBlock);
impl CanCloseMeetingRoom for BBlock {}

impl GameMachine<BBlock> {
    #[instrument(skip(ctx))]
    pub(super) async fn next(self, ctx: &Context) -> Next<CBlock> {
        if self.state.all_alive_have_won() {
            Next::GameEnded(GameMachine::<GameEnded> {
                metadata: self.metadata,
                state: GameEnded::new(self.state.players, self.state.day),
            })
        } else {
            if let Err(err) = self.close_meeting_room(ctx).await {
                info!("{:?}", err);
            }

            Next::Block(GameMachine {
                metadata: self.metadata,
                state: CBlock::new(self.state.players, self.state.day, self.state.king_substitution_status),
            })
        }
    }
    pub(super) fn set_king_substitution_status(&mut self, kss: SubstitutionStatus) {
        self.state.king_substitution_status = kss;
    }
}
