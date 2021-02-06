//! First state after NotStarted, follows FBlock.
//!
//! During this block the meeting room gets opened.

use super::{macros::state::*, *};

use serenity::{model::id::UserId, prelude::*};
use std::{collections::BTreeMap, ops::Sub};
use tracing::{info, instrument};

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

use crate::game::db::types as db;

impl From<db::RunningGame> for ABlock {
    fn from(rg: db::RunningGame) -> Self {
        debug_assert!(rg.gstate == db::GameState::ABlock);

        Self {
            players: BTreeMap::new(),
            day: rg.day as u8,
            king_substitution_status: SubstitutionStatus::HasNot,
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

    impl_common_state_boilerplate!();
}
