use tracing::{info, instrument};

use super::*;

use serenity::model::id::UserId;
use serenity::prelude::*;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
pub(super) struct ABlock {
    players: BTreeMap<UserId, Player>,
    day: u8,
    king_substitution_status: SubstitutionStatus,
}

impl ABlock {
    pub fn new(players: BTreeMap<UserId, Player>, day: u8, kss: SubstitutionStatus) -> Self {
        Self {
            players,
            day,
            king_substitution_status: kss,
        }
    }
}

impl GameState for ABlock {}
impl CanOpenMeetingRoom for ABlock {}
impl_timeblock!(ABlock);
impl_wrap!(ABlock);

impl GameMachine<ABlock> {
    #[instrument(skip(ctx))]
    pub(super) async fn next(self, ctx: &Context) -> Next<BBlock> {
        // This state does not need to check self.state.all_alive_have_won() as the state machine
        // enters it either from NotStarted, in which case there can be no winners yet, or FBlock
        // checks that method already, and has no other logic than increasing the day number.

        if let Err(err) = self.open_meeting_room(ctx).await {
            info!("{:?}", err);
        }

        Next::Block(GameMachine::<BBlock> {
            metadata: self.metadata,
            state: BBlock::new(
                self.state.players,
                self.state.day,
                self.state.king_substitution_status,
            ),
        })
    }

    pub(super) fn set_king_substitution_status(&mut self, kss: SubstitutionStatus) {
        self.state.king_substitution_status = kss;
    }

    pub(super) fn king_has_substituted(&self) -> bool {
        self.state.king_substitution_status == SubstitutionStatus::Has
    }
}
