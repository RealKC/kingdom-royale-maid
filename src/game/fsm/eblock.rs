//! Follows DBlock
//!
//! During this state:
//!  * the meeting room gets closed
//!  * players are made to either eat a piece of food or starve
//!  * the Revolutionary assassinates

use super::{macros::state::*, *};
use crate::{
    game::tasks,
    game::{data::NUMBER_EMOJIS_ONE_TO_SIX, item, DeathCause},
    helpers::{choose_target::build_embed_for_target_choice, react::react_with},
};

use serenity::{framework::standard::CommandResult, model::id::UserId, prelude::*};
use std::collections::BTreeMap;
use tracing::{info, instrument};

#[derive(Debug, Clone)]
pub(super) struct EBlock {
    players: BTreeMap<UserId, Player>,
    day: u8,

    king_substitution_status: SubstitutionStatus,
}

impl EBlock {
    pub fn new(players: BTreeMap<UserId, Player>, day: u8, kss: SubstitutionStatus) -> Self {
        Self {
            players,
            day,
            king_substitution_status: kss,
        }
    }
}

impl GameState for EBlock {}
impl_timeblock!(EBlock);
impl CanCloseMeetingRoom for EBlock {}
impl_wrap!(EBlock);

impl GameMachine<EBlock> {
    #[instrument(skip(ctx))]
    pub(super) async fn next(mut self, ctx: &Context) -> Next<FBlock> {
        if self.state.all_alive_have_won() {
            Next::GameEnded(GameMachine {
                metadata: self.metadata,
                state: GameEnded::new(self.state.players, self.state.day),
            })
        } else {
            if let Err(err) = self.close_meeting_room(ctx).await {
                info!("{}", err);
            }
            if let Err(err) = self.make_players_eat_or_starve(ctx).await {
                info!("{}", err);
            }
            if let Err(err) = self.make_revolutionary_assassinate(ctx).await {
                info!("{}", err);
            }

            if self.state.king_substitution_status == SubstitutionStatus::CurrentlyIs {
                self.state.king_substitution_status = SubstitutionStatus::Has;
            }

            info!("Successfuly ran all actions for EBlock...");

            Next::Block(GameMachine {
                metadata: self.metadata,
                state: FBlock::new(
                    self.state.players,
                    self.state.day,
                    self.state.king_substitution_status,
                ),
            })
        }
    }

    async fn make_players_eat_or_starve(&mut self, ctx: &Context) -> CommandResult {
        for player in self.state.players_mut().iter_mut() {
            let items = player.1.items_mut();

            let food = items.get_item_mut(item::Item::FOOD_NAME);

            if food.0 > 0 {
                food.0 -= 1;
            } else {
                player
                    .1
                    .set_dead(DeathCause::Starvation, ctx, self.metadata.meeting_room)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn make_revolutionary_assassinate(&mut self, ctx: &Context) -> CommandResult {
        let revolutionary = {
            let mut res = None;
            for player in self.state.players().iter() {
                if player.1.is_alive() && player.1.role_name() == RoleName::Revolutionary {
                    res = Some(player);
                    break;
                }
            }

            match res {
                Some(rev) => rev,
                None => {
                    info!("Revolutionary is dead");
                    return Ok(());
                }
            }
        };

        let embed = build_embed_for_target_choice(
            ctx,
            &self.state.players().keys().copied().collect::<Vec<_>>(),
            "Please select a target for 「 Murder 」",
        )
        .await?;

        let msg = revolutionary
            .1
            .room()
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        react_with(ctx, &msg, &NUMBER_EMOJIS_ONE_TO_SIX).await?;

        tokio::task::spawn(tasks::handle_assassination(
            ctx.clone(),
            msg,
            *revolutionary.0,
            revolutionary.1.room(),
        ));

        Ok(())
    }

    impl_common_state_boilerplate!();
}
