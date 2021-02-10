use std::{fmt::Debug, unimplemented};

use super::{
    db,
    fsm::TimeBlock,
    item::{Item, Items},
    roles::{RoleHolder, RoleName},
    DeathCause,
};
use serenity::{
    framework::standard::CommandResult,
    model::id::{ChannelId, GuildId, UserId},
    prelude::*,
};

pub type SecretMeeting = Option<(UserId, ChannelId)>;

#[derive(Clone)]
pub struct Player {
    id: UserId,
    guild_id: GuildId,
    role: RoleHolder,
    alive: bool,
    room: ChannelId,
}

impl Player {
    pub fn new(
        id: UserId,
        guild_id: GuildId,
        role: RoleHolder,
        room: ChannelId,
        _watch_colour: String,
    ) -> Self {
        // PONDER: We may want to allow disabling certain items
        //         If we do, how should that be handled? Should we just pass a reference to the Game and ask it for enabled items?
        Self {
            id,
            guild_id,
            role,
            room,
            alive: true,
        }
    }

    pub fn from_db(
        id: UserId,
        guild_id: GuildId,
        role: db::Role,
        room: ChannelId,
        alive: bool,
    ) -> Self {
        Self {
            id,
            guild_id,
            role: role.into(),
            alive,
            room,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn room(&self) -> ChannelId {
        self.room
    }

    pub fn secret_meeting_partner(&self) -> Option<UserId> {
        unimplemented!()
    }

    pub fn set_secret_meeting_partner(&mut self, partner: UserId) {
        unimplemented!()
    }

    pub fn add_secret_meeting(&mut self, day: u8, channel: ChannelId) {
        unimplemented!()
    }

    pub fn get_secret_meetings_for_day(&self, day: u8) -> Option<&(SecretMeeting, SecretMeeting)> {
        unimplemented!()
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub async fn set_dead(
        &mut self,
        cause: DeathCause,
        ctx: &Context,
        channel: ChannelId,
    ) -> CommandResult {
        self.alive = false;

        channel
            .say(ctx, format!("{} {}", self.id.mention(), cause))
            .await?;
        Ok(())
    }

    /// Function meant to be only used inside test functions
    pub fn set_dead_mock(&mut self) {
        self.alive = false;
    }

    pub fn items(&self) -> &Items {
        unimplemented!()
    }

    pub fn add_item(&mut self, item: Item) {
        unimplemented!()
    }

    pub fn items_mut(&mut self) -> &mut Items {
        unimplemented!()
    }

    pub fn win_condition_achieved(&self, block: &dyn TimeBlock) -> bool {
        self.role.win_condition_achieved(block)
    }

    pub fn role_name(&self) -> RoleName {
        self.role.name()
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("role", &self.role.name())
            .field("alive", &self.alive)
            .field("room", &self.room)
            .finish()
    }
}
