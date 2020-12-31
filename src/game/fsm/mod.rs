#![allow(dead_code)]

//! This module contains the various types, and traits needed for implementing the game state machine

#[rustfmt::skip]
mod notstarted;
mod ablock;
mod bblock;
mod cblock;
mod dblock;
mod eblock;
mod fblock;
mod gameended;

#[rustfmt::skip]
use notstarted::*;
use ablock::*;
use bblock::*;
use cblock::*;
use dblock::*;
use eblock::*;
use fblock::*;
use gameended::*;
use tracing::{info, warn};

use super::roles::RoleName;
use crate::game::data::*;
pub use crate::game::player::Player;
use crate::helpers::perms;

use serenity::framework::standard::CommandResult;
use serenity::model::id::UserId;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::BTreeMap;
use tracing::error;

/// Struct for the public API of the state machine
#[derive(Clone)]
pub struct Game(Wrapper);

impl Game {
    pub fn new(
        guild: GuildId,
        host: UserId,
        meeting_room: ChannelId,
        announcement_channel: ChannelId,
        player_role: RoleId,
        delete_rooms_category_on_game_end: bool,
    ) -> Self {
        Self(Wrapper::NotStarted(GameMachine {
            metadata: Metadata {
                guild,
                host,
                meeting_room,
                announcement_channel,
                player_role,
                delete_rooms_category_on_game_end,
            },
            state: NotStarted {
                joined_users: vec![],
            },
        }))
    }

    pub async fn transition_to_next_state(self, ctx: &Context) -> Self {
        Game(self.0.next(ctx).await)
    }

    pub async fn start(self, ctx: &Context) -> CommandResult<Self> {
        match self.0 {
            Wrapper::NotStarted(ns) => Ok(Self(ns.next(ctx).await?.wrap())),
            other => {
                error!("Game::start called on an already started game...");
                Ok(Self(other))
            }
        }
    }

    pub async fn end(&mut self, ctx: &Context) -> CommandResult {
        if let Some(players) = self.players() {
            let mut rooms_category = None;
            for player in players.iter() {
                self.guild()
                    .member(ctx, player.0)
                    .await?
                    .remove_role(ctx, self.metadata().player_role)
                    .await?;

                if self.metadata().delete_rooms_category_on_game_end {
                    info!("Deleting a room...");
                    let channel = player.1.room().to_channel(ctx).await?.guild().unwrap();
                    if rooms_category.is_none() {
                        rooms_category = channel.category_id;
                    }
                    channel.delete(ctx).await?;
                    info!("Room deleted.")
                }
            }

            if self.metadata().delete_rooms_category_on_game_end && rooms_category.is_some() {
                info!("Deleting the category...");
                rooms_category.unwrap().delete(ctx).await?;
                info!("Deleted the category.")
            }
        }

        Ok(())
    }

    pub fn join(&mut self, id: UserId) -> JoinResult {
        if let Wrapper::NotStarted(s) = &mut self.0 {
            s.join(id)
        } else {
            Err(JoinError::GameStarted)
        }
    }

    pub fn leave(&mut self, id: UserId) -> LeaveResult {
        if let Wrapper::NotStarted(s) = &mut self.0 {
            s.leave(id)
        } else {
            Err(LeaveError::GameStarted)
        }
    }

    pub fn set_king_substitution_status(&mut self, st: SubstitutionStatus) {
        match &mut self.0 {
            Wrapper::ABlock(s) => s.set_king_substitution_status(st),
            Wrapper::BBlock(s) => s.set_king_substitution_status(st),
            Wrapper::CBlock(s) => s.set_king_substitution_status(st),
            Wrapper::DBlock(s) => s.set_king_substitution_status(st),
            Wrapper::EBlock(s) => s.set_king_substitution_status(st),
            Wrapper::FBlock(s) => s.set_king_substitution_status(st),
            other => {
                warn!(
                "Game::set_king_substitution_status called in set_king_substition_status on {:?}",
                other
            );
            }
        }
    }

    pub fn host(&self) -> UserId {
        self.metadata().host
    }

    pub fn guild(&self) -> GuildId {
        self.metadata().guild
    }

    pub fn meeting_room(&self) -> ChannelId {
        self.metadata().meeting_room
    }

    pub fn announcement_channel(&self) -> ChannelId {
        self.metadata().announcement_channel
    }

    pub fn player_role(&self) -> RoleId {
        self.metadata().player_role
    }

    #[inline]
    fn metadata(&self) -> &Metadata {
        self.0.metadata()
    }

    pub fn is_started(&self) -> bool {
        !matches!(self.0, Wrapper::NotStarted(_))
    }

    pub fn is_ended(&self) -> bool {
        matches!(self.0, Wrapper::GameEnded(_))
    }

    pub fn can_start(&self) -> bool {
        match self.joined_users() {
            Some(j) => j.len() == 6,
            None => true, // technically it's already started at this point, I don't think this codepath should ever be exercised though
        }
    }

    pub fn can_start_gathering(&self) -> bool {
        matches!(self.0, Wrapper::ABlock(_) | Wrapper::DBlock(_))
    }

    pub fn can_end_gathering(&self) -> bool {
        matches!(self.0, Wrapper::BBlock(_) | Wrapper::DBlock(_))
    }

    pub fn secret_meetings_took_place(&self) -> bool {
        matches!(self.0, Wrapper::EBlock(_) | Wrapper::FBlock(_))
    }

    pub fn secret_meetings_are_happening(&self) -> bool {
        matches!(self.0, Wrapper::DBlock(_))
    }

    pub fn all_alive_have_won(&self) -> bool {
        match &self.0 {
            Wrapper::ABlock(s) => s.state.all_alive_have_won(),
            Wrapper::BBlock(s) => s.state.all_alive_have_won(),
            Wrapper::CBlock(s) => s.state.all_alive_have_won(),
            Wrapper::DBlock(s) => s.state.all_alive_have_won(),
            Wrapper::EBlock(s) => s.state.all_alive_have_won(),
            Wrapper::FBlock(s) => s.state.all_alive_have_won(),
            Wrapper::GameEnded(_) => true,
            Wrapper::NotStarted(_) => false,
        }
    }

    pub fn king_has_substituted(&self) -> Option<bool> {
        match &self.0 {
            Wrapper::ABlock(s) => Some(s.king_has_substituted()),
            Wrapper::BBlock(s) => Some(s.king_has_substituted()),
            Wrapper::CBlock(s) => Some(s.king_has_substituted()),
            Wrapper::DBlock(s) => Some(s.king_has_substituted()),
            Wrapper::EBlock(s) => Some(s.king_has_substituted()),
            Wrapper::FBlock(s) => Some(s.king_has_substituted()),
            _ => None,
        }
    }

    pub fn state_name(&self) -> &'static str {
        match self.0 {
            Wrapper::NotStarted(_) => "Not started",
            Wrapper::ABlock(_) => "<A>",
            Wrapper::BBlock(_) => "<B>",
            Wrapper::CBlock(_) => "<C>",
            Wrapper::DBlock(_) => "<D>",
            Wrapper::EBlock(_) => "<E>",
            Wrapper::FBlock(_) => "<F>",
            Wrapper::GameEnded(_) => "Game has ended",
        }
    }

    pub fn time_range(&self) -> Option<&'static str> {
        match self.0 {
            Wrapper::ABlock(_) => Some("~12"),
            Wrapper::BBlock(_) => Some("12~14"),
            Wrapper::CBlock(_) => Some("14~18"),
            Wrapper::DBlock(_) => Some("18~20"),
            Wrapper::EBlock(_) => Some("20~22"),
            Wrapper::FBlock(_) => Some("22~"),
            _ => None,
        }
    }

    pub fn day(&self) -> Option<u8> {
        match &self.0 {
            Wrapper::BBlock(s) => Some(s.day()),
            Wrapper::CBlock(s) => Some(s.day()),
            Wrapper::DBlock(s) => Some(s.day()),
            Wrapper::ABlock(s) => Some(s.day()),
            Wrapper::EBlock(s) => Some(s.day()),
            Wrapper::FBlock(s) => Some(s.day()),
            Wrapper::GameEnded(s) => Some(s.day()),
            _ => None,
        }
    }

    pub fn joined_users(&self) -> Option<&Vec<UserId>> {
        match &self.0 {
            Wrapper::NotStarted(s) => Some(&s.state.joined_users),
            _ => None,
        }
    }

    pub fn players(&self) -> Option<&BTreeMap<UserId, Player>> {
        match &self.0 {
            Wrapper::BBlock(s) => Some(s.players()),
            Wrapper::CBlock(s) => Some(s.players()),
            Wrapper::DBlock(s) => Some(s.players()),
            Wrapper::ABlock(s) => Some(s.players()),
            Wrapper::EBlock(s) => Some(s.players()),
            Wrapper::FBlock(s) => Some(s.players()),
            Wrapper::GameEnded(s) => Some(s.players()),
            _ => None,
        }
    }

    pub fn players_mut(&mut self) -> Option<&mut BTreeMap<UserId, Player>> {
        match &mut self.0 {
            Wrapper::BBlock(s) => Some(s.players_mut()),
            Wrapper::CBlock(s) => Some(s.players_mut()),
            Wrapper::DBlock(s) => Some(s.players_mut()),
            Wrapper::ABlock(s) => Some(s.players_mut()),
            Wrapper::EBlock(s) => Some(s.players_mut()),
            Wrapper::FBlock(s) => Some(s.players_mut()),
            Wrapper::GameEnded(s) => Some(s.players_mut()),
            _ => None,
        }
    }

    pub fn set_king_murder_target(&mut self, target: UserId) {
        match &mut self.0 {
            Wrapper::CBlock(s) => s.set_king_murder_target(target),
            other => warn!("set_king_murder_target got called in {:?}", other),
        }
    }

    pub fn king_murder_target(&self) -> Option<UserId> {
        match &self.0 {
            Wrapper::CBlock(c) => Some(c.king_murder_target()),
            _ => None,
        }
    }

    pub fn player(&self, user: UserId) -> Option<&Player> {
        match self.players() {
            None => None,
            Some(players) => players.get(&user),
        }
    }

    pub fn player_mut(&mut self, user: UserId) -> Option<&mut Player> {
        match self.players_mut() {
            None => None,
            Some(players) => players.get_mut(&user),
        }
    }
}

/// The "low-level" struct that drives the logic for the state machine
///
/// State transitions are represented through next() methods implemented on "specializations" of this type
#[derive(Debug, Clone)]
struct GameMachine<S>
where
    S: GameState + Clone,
{
    metadata: Metadata,
    state: S,
}

impl<S> GameMachine<S>
where
    S: TimeBlock + Clone,
{
    fn players(&self) -> &BTreeMap<UserId, Player> {
        self.state.players()
    }

    fn players_mut(&mut self) -> &mut BTreeMap<UserId, Player> {
        self.state.players_mut()
    }

    fn day(&self) -> u8 {
        self.state.day()
    }
}

impl<S> GameMachine<S>
where
    S: CanOpenMeetingRoom + Clone,
{
    async fn open_meeting_room(&self, ctx: &Context) -> CommandResult {
        self.metadata
            .meeting_room
            .create_permission(
                ctx,
                &perms::make_allowed_override_for_role(self.metadata.player_role),
            )
            .await?;
        Ok(())
    }
}

impl<S> GameMachine<S>
where
    S: CanCloseMeetingRoom + Clone,
{
    async fn close_meeting_room(&self, ctx: &Context) -> CommandResult {
        self.metadata
            .meeting_room
            .create_permission(
                ctx,
                &perms::make_denied_override_for_role(self.metadata.player_role),
            )
            .await?;

        Ok(())
    }
}

/// Stores Discord related information about a game
#[derive(Debug, Clone)]
struct Metadata {
    guild: GuildId,
    meeting_room: ChannelId,
    announcement_channel: ChannelId,
    host: UserId,
    player_role: RoleId,
    delete_rooms_category_on_game_end: bool,
}

/// Marker trait for a struct that represents a valid game state
pub trait GameState: std::fmt::Debug {}

/// Enum whose only purpose is to wrap the various type-states in a single type
#[derive(Debug, Clone)]
enum Wrapper {
    NotStarted(GameMachine<NotStarted>),
    ABlock(GameMachine<ABlock>),
    BBlock(GameMachine<BBlock>),
    CBlock(GameMachine<CBlock>),
    DBlock(GameMachine<DBlock>),
    EBlock(GameMachine<EBlock>),
    FBlock(GameMachine<FBlock>),
    GameEnded(GameMachine<GameEnded>),
}

impl Wrapper {
    #[inline]
    fn metadata(&self) -> &Metadata {
        match self {
            Wrapper::NotStarted(s) => &s.metadata,
            Wrapper::ABlock(s) => &s.metadata,
            Wrapper::BBlock(s) => &s.metadata,
            Wrapper::CBlock(s) => &s.metadata,
            Wrapper::DBlock(s) => &s.metadata,
            Wrapper::EBlock(s) => &s.metadata,
            Wrapper::FBlock(s) => &s.metadata,
            Wrapper::GameEnded(s) => &s.metadata,
        }
    }

    async fn next(self, ctx: &Context) -> Self {
        match self {
            Wrapper::ABlock(s) => s.next(ctx).await.wrap(),
            Wrapper::BBlock(s) => s.next(ctx).await.wrap(),
            Wrapper::CBlock(s) => s.next(ctx).await.wrap(),
            Wrapper::DBlock(s) => s.next(ctx).await.wrap(),
            Wrapper::EBlock(s) => s.next(ctx).await.wrap(),
            Wrapper::FBlock(s) => s.next().wrap(),
            Wrapper::GameEnded(s) => {
                info!("Can't call next on GameEnded");
                s.wrap()
            }
            w => w,
        }
    }

    async fn start(self, ctx: &Context) -> CommandResult<Self> {
        if let Wrapper::NotStarted(ne) = self {
            Ok(ne.next(ctx).await?.wrap())
        } else {
            Ok(self)
        }
    }
}

/// Trait that represents wrapping of a given type-state into the big Wrapper
trait Wrap {
    fn wrap(self) -> Wrapper;
}

#[macro_export]
macro_rules! impl_wrap {
    ($name:ident) => {
        impl Wrap for GameMachine<$name> {
            fn wrap(self) -> Wrapper {
                Wrapper::$name(self)
            }
        }
    };
}
pub use crate::impl_wrap;

#[macro_export]
macro_rules! impl_common_state_boilerplate {
    () => {
        pub(super) fn set_king_substitution_status(&mut self, kss: SubstitutionStatus) {
            self.state.king_substitution_status = kss;
        }

        pub(super) fn king_has_substituted(&self) -> bool {
            matches!(self.state.king_substitution_status, SubstitutionStatus::Has | SubstitutionStatus::CurrentlyIs)
        }

    }
}
pub use crate::impl_common_state_boilerplate;

/// Variant representing the next state after a TimeBlock
enum Next<S>
where
    S: TimeBlock + Clone,
{
    /// The TimeBlock following
    Block(GameMachine<S>),
    /// The game ended
    GameEnded(GameMachine<GameEnded>),
}

impl<S> Wrap for Next<S>
where
    S: TimeBlock + Clone,
    GameMachine<S>: Wrap,
{
    fn wrap(self) -> Wrapper {
        match self {
            Next::Block(s) => s.wrap(),
            Next::GameEnded(ge) => ge.wrap(),
        }
    }
}

/// Trait representing time blocks
pub trait TimeBlock: GameState {
    fn day(&self) -> u8;
    fn players(&self) -> &BTreeMap<UserId, Player>;
    fn players_mut(&mut self) -> &mut BTreeMap<UserId, Player>;

    fn all_alive_have_won(&self) -> bool
    where
        Self: Sized,
    {
        for player in self.players().iter() {
            if !player.1.win_condition_achieved(self) {
                return false;
            }
        }

        true
    }

    fn is_king_alive(&self) -> bool {
        self.is_alive(RoleName::King)
    }

    fn is_prince_alive(&self) -> bool {
        self.is_alive(RoleName::Prince)
    }

    fn is_the_double_alive(&self) -> bool {
        self.is_alive(RoleName::TheDouble)
    }

    fn is_sorcerer_alive(&self) -> bool {
        self.is_alive(RoleName::Sorcerer)
    }

    fn is_knight_alive(&self) -> bool {
        self.is_alive(RoleName::Knight)
    }

    fn is_revolutionary_alive(&self) -> bool {
        self.is_alive(RoleName::Revolutionary)
    }
    fn is_alive(&self, role: RoleName) -> bool {
        for player in self.players().iter() {
            if player.1.role_name() == role {
                return player.1.is_alive();
            }
        }

        unreachable!("There should always be a {:?} in the game", role)
    }
}

#[macro_export]
macro_rules! impl_timeblock {
    ($name:ident) => {
        impl TimeBlock for $name {
            fn day(&self) -> u8 {
                self.day
            }

            fn players(&self) -> &BTreeMap<UserId, Player> {
                &self.players
            }

            fn players_mut(&mut self) -> &mut BTreeMap<UserId, Player> {
                &mut self.players
            }
        }
    };
}
pub use crate::impl_timeblock;

/// Marker trait for TimeBlocks in which the meeting room can be opened
trait CanOpenMeetingRoom: TimeBlock {}

/// Marker trait for TimeBlocks in which the meeting room can be closed
trait CanCloseMeetingRoom: TimeBlock {}
