//! Follows BBlock
//!
//! During this block:
//! * the secret meeting partners get chosen & the secret meetings happen
//! * the King selects a target & either the Sorcerer or Knight will decide whether to kill the target or not

use super::*;
use crate::{
    game::{data::*, player::Player, roles::RoleName, tasks},
    helpers::{
        choose_target::build_embed_for_target_choice,
        confirm_murder::build_embed_for_murder_confirmation, perms, react::react_with,
    },
};

use serenity::{
    builder::CreateEmbed,
    model::{
        channel::ChannelType,
        id::{ChannelId, GuildId, RoleId, UserId},
        prelude::User,
    },
    prelude::*,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write,
};
use tracing::{info, instrument};

#[derive(Debug, Clone)]
pub(super) struct CBlock {
    players: BTreeMap<UserId, Player>,
    day: u8,
    king_murder_target: UserId,
    king_substitution_status: SubstitutionStatus,
}

impl CBlock {
    pub fn new(players: BTreeMap<UserId, Player>, day: u8, kss: SubstitutionStatus) -> Self {
        Self {
            players,
            day,
            king_murder_target: UserId::default(),
            king_substitution_status: kss,
        }
    }
}

impl GameState for CBlock {}
impl_timeblock!(CBlock);
impl_wrap!(CBlock);

impl GameMachine<CBlock> {
    #[instrument(skip(ctx))]
    pub(super) async fn next(mut self, ctx: &Context) -> Next<DBlock> {
        if self.state.all_alive_have_won() {
            return Next::GameEnded(GameMachine {
                metadata: self.metadata,
                state: GameEnded::new(self.state.players, self.state.day),
            });
        }

        info!("Selecting secret meeting partners...");
        if let Err(e) = self.select_secret_meeting_partners(ctx).await {
            info!("{}", e);
        }
        info!("Announcing secret meeting partners...");
        if let Err(e) = self.announce_secret_meeting_partners(ctx).await {
            info!("{}", e);
        }
        info!("Opening the secret meeting rooms...");
        if let Err(e) = self.open_secret_meeting_rooms(ctx).await {
            info!("{}", e);
        }
        info!("Making the king select a target...");
        if let Err(e) = self.make_king_select_target(ctx).await {
            info!("{}", e);
        }
        info!("Making the king's assistant choose...");
        if let Err(e) = self.make_assistant_choose(ctx).await {
            info!("{}", e);
        }
        info!("Going to the next block...");

        Next::Block(GameMachine {
            metadata: self.metadata,
            state: DBlock::new(
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

    pub(super) fn set_king_murder_target(&mut self, target: UserId) {
        self.state.king_murder_target = target;
    }

    pub(super) fn king_murder_target(&self) -> UserId {
        self.state.king_murder_target
    }

    async fn select_secret_meeting_partners(&mut self, ctx: &Context) -> CommandResult {
        info!("Collecting rooms...");
        let rooms = self
            .state
            .players()
            .iter()
            .map(|player| (*player.0, player.1.room()))
            .collect::<Vec<_>>();
        info!("OK! Succesfully collected rooms");

        info!("Trying to build an embed");
        let embed = build_embed_for_target_choice(
            ctx,
            &self.state.players().keys().copied().collect::<Vec<_>>(),
            "Please select a partner for your secret meeting",
        )
        .await?;
        info!("Embed built successfuly");

        for user_and_room in rooms {
            info!("Trying to send messages...");
            let msg = user_and_room
                .1
                .send_message(ctx, |m| m.set_embed(embed.clone()))
                .await?;
            info!("We succeeded. Room={}", user_and_room.1.mention());

            react_with(ctx, &msg, &NUMBER_EMOJIS_ONE_TO_SIX).await?;

            tokio::task::spawn(tasks::handle_secret_meeting_selection(
                ctx.clone(),
                msg,
                user_and_room,
            ));
        }

        Ok(())
    }

    async fn announce_secret_meeting_partners(&self, ctx: &Context) -> CommandResult {
        let partners = {
            let mut res = String::new();

            for player in self.state.players().iter() {
                res.push_str(&format!(
                    "{} => {}",
                    player.0.mention(),
                    player.1.secret_meeting_partner().unwrap().mention()
                ));
            }

            res
        };

        let mut embed = CreateEmbed::default();
        embed
            .title("Secret meeting partners")
            .field("A => B", partners, true);

        self.metadata
            .announcement_channel
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        Ok(())
    }

    async fn open_secret_meeting_rooms(&mut self, ctx: &Context) -> CommandResult {
        let meetings_category = self
            .metadata
            .guild
            .create_channel(ctx, |ch| {
                ch.name(format!("Secret meetings for day {}", self.state.day()))
                    .kind(ChannelType::Category)
            })
            .await?;

        let mut players_mapped_to_secret_rooms: HashMap<UserId, ChannelId> = Default::default();

        for player in self.state.players().iter() {
            async fn get_suitable_name(user: User, ctx: &Context, guild: GuildId) -> String {
                user.nick_in(ctx, guild).await.unwrap_or_else(|| {
                    user.name
                        .chars()
                        .map(|c| {
                            if c.is_whitespace()
                                || ['"', ',', '.', '\'', '/', ';', '[', ']', '=', '\\'].contains(&c)
                            {
                                '-'
                            } else {
                                c
                            }
                        })
                        .collect()
                })
            }

            let guest = player.0.to_user(ctx).await?;
            let guest_id = guest.id;
            let guest_name = get_suitable_name(guest, ctx, self.metadata.guild).await;
            let host = player
                .1
                .secret_meeting_partner()
                .unwrap()
                .to_user(ctx)
                .await?;
            let host_id = host.id;
            let host_name = get_suitable_name(host, ctx, self.metadata.guild).await;

            let mut name = String::with_capacity(16 + guest_name.len() + host_name.len());
            write!(name, "{}-{}", guest_name, host_name)?;

            let at_everyone_perms = perms::make_denied_override_for_role(RoleId {
                0: self.metadata.guild.0,
            });
            let guest_perms = perms::make_allowed_override_for_user(guest_id);
            let host_perms = perms::make_allowed_override_for_user(host_id);

            let channel = self
                .metadata
                .guild
                .create_channel(ctx, |ch| {
                    ch.name(name)
                        .kind(ChannelType::Text)
                        .category(meetings_category.id)
                })
                .await?;

            players_mapped_to_secret_rooms.insert(guest_id, channel.id);

            channel.create_permission(ctx, &guest_perms).await?;
            channel.create_permission(ctx, &host_perms).await?;
            channel.create_permission(ctx, &at_everyone_perms).await?;
        }

        let day = self.state.day();
        for player in &mut self.state.players_mut().iter_mut() {
            let room = players_mapped_to_secret_rooms.get(player.0).unwrap();
            player.1.add_secret_meeting(day, *room);
        }

        Ok(())
    }

    async fn make_king_select_target(&mut self, ctx: &Context) -> CommandResult {
        let king = {
            // May not necessarily be the king proper, but the next one in the hierarchy
            // Hierarchy: King -(dies)-> The Double -(dies)-> Prince

            let mut candidates = vec![];
            for player in self.state.players().iter() {
                if player.1.role_name().is_king_like() {
                    candidates.push(player);
                }
            }

            candidates.sort_by(|a, b| {
                fn map_to_int(role: RoleName) -> u8 {
                    match role {
                        RoleName::King => 100,
                        RoleName::TheDouble => 50,
                        RoleName::Prince => 25,
                        _ => unreachable!(),
                    }
                };

                map_to_int(a.1.role_name()).cmp(&map_to_int(b.1.role_name()))
            });

            let mut res = None;
            for candidate in candidates {
                let player = candidate.1;
                if player.is_alive() {
                    res = Some(*candidate.0);
                }
            }
            res
        };

        let king = match king {
            Some(k) => k,
            None => return Err("There is a unusual lack of nobility".into()), // I *think* this shouldn't happen as no nobility => someone won, already
        };

        if !self.state.is_sorcerer_alive() || !self.state.is_knight_alive() {
            self.state
                .players()
                .get(&king)
                .unwrap()
                .room()
                .say(
                    ctx,
                    "You cannot ask the dead to commit murder for you. Maybe pick up that knife?",
                )
                .await?;
            return Ok(());
        }

        let embed = build_embed_for_target_choice(
            ctx,
            &self.state.players().keys().copied().collect::<Vec<_>>(),
            "Please select a target for 「 Murder 」",
        )
        .await?;

        let msg = self
            .state
            .players()
            .get(&king)
            .unwrap()
            .room()
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        react_with(ctx, &msg, &NUMBER_EMOJIS_ONE_TO_SIX).await?;

        let room_id = msg.channel_id;

        tokio::task::spawn(tasks::handle_king_choosing_target(
            ctx.clone(),
            msg,
            king,
            room_id,
        ));

        Ok(())
    }

    async fn make_assistant_choose(&mut self, ctx: &Context) -> CommandResult {
        let embed = build_embed_for_murder_confirmation(
            ctx,
            self.state.king_murder_target,
            self.metadata.guild,
        )
        .await?;

        let sorc_or_knight = {
            let mut res = None;

            for player in self.state.players().iter() {
                if player.1.is_alive()
                    && [RoleName::Knight, RoleName::Sorcerer].contains(&player.1.role_name())
                {
                    res = Some(player.0);
                    break;
                }
            }

            res
        };

        if sorc_or_knight.is_none() {
            return Ok(());
        }

        let sorc_or_knight = sorc_or_knight.unwrap();

        let msg = self
            .state
            .players()
            .get(sorc_or_knight)
            .unwrap()
            .room()
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;

        react_with(ctx, &msg, &YES_NO_EMOJIS).await?;

        let room_id = msg.channel_id;
        tokio::task::spawn(tasks::handle_assistant_choice(
            ctx.clone(),
            msg,
            *sorc_or_knight,
            room_id,
        ));

        Ok(())
    }
}
